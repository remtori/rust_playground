use std::{process::Stdio, time::Duration};

use anyhow::Context;
use futures::{Future, SinkExt};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    process::{Child, Command},
    sync::Mutex,
};
use tungstenite::{handshake::server::Request, Message};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port = std::option_env!("PORT")
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080u16);

    let try_socket = TcpListener::bind(("0.0.0.0", port)).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

lazy_static! {
    static ref PROCCESSES: chashmap::CHashMap<String, Mutex<Child>> = chashmap::CHashMap::new();
}

async fn accept_connection(stream: TcpStream) -> Result<(), anyhow::Error> {
    let addr = stream
        .peer_addr()
        .context("connected streams should have a peer address")?;

    println!("Peer address: {}", addr);

    let path = &mut None;
    let ws_stream = tokio_tungstenite::accept_hdr_async(stream, |req: &Request, res| {
        path.replace(req.uri().path().to_owned());
        Ok(res)
    })
    .await
    .context("Error during the websocket handshake occurred")?;

    let path = path.as_ref().unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    println!("New WebSocket connection: {} at {}", addr, path);

    let child_process_guard = match PROCCESSES.get(path) {
        Some(mutex) => mutex,
        None => {
            let proc = Command::new("cmd")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .context("Spawning command")?;

            PROCCESSES.insert_new(path.to_owned(), Mutex::new(proc));
            PROCCESSES.get(path).unwrap()
        }
    };

    // Other ws with same path will be block here
    let child_process = &mut child_process_guard.lock().await;

    let mut stdin = child_process.stdin.take().unwrap();
    let mut stdout = child_process.stdout.take().unwrap();
    let mut stderr = child_process.stderr.take().unwrap();

    let mut interval = tokio::time::interval(Duration::from_millis(100));

    let out_buffer = &mut Vec::new();
    let err_buffer = &mut Vec::new();

    loop {
        tokio::select! {
            Ok(_) = stdout.read_buf(out_buffer) => {
                ws_sender
                    .send(Message::Text(
                        String::from_utf8_lossy(&out_buffer).to_string(),
                    ))
                    .await?;

                out_buffer.clear();
            }
            Ok(_) = stderr.read_buf(err_buffer) => {
                ws_sender
                    .send(Message::Text(
                        String::from_utf8_lossy(&err_buffer).to_string(),
                    ))
                    .await?;

                    err_buffer.clear();
            }
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            stdin.write_all(&msg.into_data()).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                // ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    let _ = child_process.stdin.insert(stdin);
    let _ = child_process.stdout.insert(stdout);
    let _ = child_process.stderr.insert(stderr);

    Ok(())
}
