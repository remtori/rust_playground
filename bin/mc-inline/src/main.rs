use std::{path::PathBuf, sync::mpsc, time::Duration};

use anyhow::Context;
use config::RunOptions;
use notify::Watcher;
use process::Processor;
use tokio::fs::OpenOptions;

use rusty_v8 as v8;

mod config;
mod process;
mod script;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<(), anyhow::Error> {
    let options = config::RunOptions::new()?;

    let executor_handle = {
        let platform = v8::new_default_platform(1, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let (tx, rx) = tokio::sync::mpsc::channel(64);
        script::SCRIPT_QUEUE.set(tx).unwrap();

        tokio::task::spawn_blocking(|| script::poll_executor(rx))
    };

    // let mut iteration = 0;
    let mut num_processed = 0;
    let mut num_file = 0;

    for paths in options.paths.values() {
        num_file += paths.len();

        let handles = paths
            .iter()
            .map(|path| tokio::spawn(run(path.clone())))
            .collect::<Vec<_>>();

        // println!("Iteration #{:02}", iteration);
        // iteration += 1;

        for result in futures::future::join_all(handles).await {
            match result.map_err(anyhow::Error::from) {
                Err(err) | Ok(Err(err)) => {
                    println!("Error: {:?}", err);
                }
                _ => {
                    num_processed += 1;
                }
            }
        }
    }

    println!("Processed {}/{} files", num_processed, num_file);

    if options.is_watch {
        watch(&options).await?;
    }

    script::send_packet(script::Packet::Close).await.unwrap();
    executor_handle.await?;
    Ok(())
}

async fn run(input_path: PathBuf) -> Result<(), anyhow::Error> {
    let output_path = {
        let mut path = input_path.clone().into_os_string();
        path.push(".tmp");
        path
    };

    // println!("Processing: {}", input_path.to_str().unwrap());

    let input_file = OpenOptions::new()
        .read(true)
        .open(&input_path)
        .await
        .with_context(|| format!("open input file: {:?}", input_path))?;

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output_path)
        .await
        .with_context(|| format!("open output file: {:?}", output_path))?;

    Processor::new(input_file, output_file)
        .process()
        .await
        .context("processing file")?;

    tokio::fs::rename(&output_path, &input_path)
        .await
        .with_context(|| format!("rename {:?} -> {:?}", output_path, input_path))?;

    Ok(())
}

async fn watch(options: &RunOptions) -> Result<(), anyhow::Error> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::watcher(tx, Duration::from_secs(1))?;
    watcher.watch(&options.base_dir, notify::RecursiveMode::Recursive)?;

    println!(
        "Watching {} for changes ...",
        options.base_dir.to_str().unwrap()
    );

    loop {
        let event = rx.recv()?;
        // println!("RecvEvent: {:?}", event);

        match event {
            notify::DebouncedEvent::Write(path) => {
                if path.is_file() {
                    run(path).await?;
                }
            }
            notify::DebouncedEvent::Error(err, path) => {
                println!("WatchError: {:?}: {:?}", path, err);
            }
            _ => {}
        }
    }
}
