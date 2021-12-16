use std::{collections::HashMap, convert::TryFrom};

use once_cell::sync::OnceCell;
use tokio::sync::{mpsc, oneshot};

use rusty_v8 as v8;

#[derive(Debug)]
pub enum Packet {
    Exec(usize, String, oneshot::Sender<Result<String, String>>),
    Close,
}

pub static SCRIPT_QUEUE: OnceCell<mpsc::Sender<Packet>> = OnceCell::new();

pub fn poll_executor(mut rx: mpsc::Receiver<Packet>) {
    let isolate = &mut v8::Isolate::new(Default::default());
    let isolate_scope = &mut v8::HandleScope::new(isolate);
    let executor = &mut ScriptExecutor::new(isolate_scope);

    loop {
        match rx.blocking_recv() {
            Some(Packet::Exec(id, source, sender)) => {
                let out = executor.execute_script(id, &source);
                // println!(
                //     "### Executing ###\n{}### Result ###\n{}",
                //     source,
                //     match &out {
                //         Ok(str) | Err(str) => str,
                //     }
                // );

                if sender.send(out).is_err() {
                    println!("Error sending result of script execution:\n{}", source);
                }
            }
            _ => break,
        }
    }
}

pub async fn execute_script(id: usize, source: String) -> Result<String, String> {
    let (tx, rx) = oneshot::channel();

    send_packet(Packet::Exec(id, source, tx))
        .await
        .expect("please init SCRIPT_QUEUE before execute_script");

    rx.await.unwrap()
}

pub async fn send_packet(packet: Packet) -> Result<(), ()> {
    match SCRIPT_QUEUE.get() {
        Some(sender) => sender.send(packet).await.map_err(|_| ()),
        None => Err(()),
    }
}

struct ScriptExecutor<'s, 'i> {
    context: v8::Local<'s, v8::Context>,
    context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
    local_map: HashMap<usize, v8::Local<'i, v8::Object>>,
}

impl<'s, 'i> ScriptExecutor<'s, 'i>
where
    's: 'i,
{
    fn new(isolate_scope: &'i mut v8::HandleScope<'s, ()>) -> Self {
        let context = v8::Context::new(isolate_scope);
        let context_scope = v8::ContextScope::new(isolate_scope, context);

        Self {
            context,
            context_scope,
            local_map: HashMap::new(),
        }
    }

    fn execute_script(&mut self, id: usize, source: &str) -> Result<String, String> {
        let source = format!(
            "(function (global, local) {{
                'use strict';

                let __local_output = '';
                const print = s => {{ __local_output += s.trim() + ' '; }};
                const println = s => {{ __local_output += s.trim() + '\\n'; }};

                {}

                return __local_output.trim();
            }})
            ",
            source
        );

        let local = self
            .local_map
            .entry(id)
            .or_insert(v8::Object::new(&mut self.context_scope));

        let script = v8::String::new(&mut self.context_scope, &source).unwrap();

        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let global: v8::Local<v8::Value> = self.context.global(try_catch).into();

        let exec = &mut || -> Option<String> {
            let script = v8::Script::compile(try_catch, script, None)?;

            let value = script.run(try_catch)?;
            let func = v8::Local::<v8::Function>::try_from(value).expect("value is not a function");

            let output = func.call(try_catch, global, &[global.into(), local.to_owned().into()])?;

            Some(output.to_string(try_catch)?.to_rust_string_lossy(try_catch))
        };

        match exec() {
            Some(v) => Ok(v),
            None => {
                let exception = try_catch.exception().unwrap();
                let exception_string = exception
                    .to_string(try_catch)
                    .unwrap()
                    .to_rust_string_lossy(try_catch);

                Err(exception_string)
            }
        }
    }
}
