use std::sync::atomic::AtomicUsize;

use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

use crate::script::execute_script;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ProcessingContext {
    None,
    Inline,
    Script,
    NeedScriptTerminator,
}

impl std::fmt::Display for ProcessingContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingContext::None => write!(f, " "),
            ProcessingContext::Inline => write!(f, "!"),
            ProcessingContext::Script => write!(f, "$"),
            ProcessingContext::NeedScriptTerminator => write!(f, "#"),
        }
    }
}

pub struct Processor<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> {
    id: usize,
    reader: BufReader<R>,
    writer: W,
    next_buffer: Vec<u8>,
    context: ProcessingContext,
    prev_context: ProcessingContext,
}

impl<R, W> Processor<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    pub fn new(reader: R, writer: W) -> Self {
        static ID_GEN: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: ID_GEN.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            reader: BufReader::new(reader),
            writer,
            next_buffer: Vec::new(),
            context: ProcessingContext::None,
            prev_context: ProcessingContext::None,
        }
    }

    pub async fn process(&mut self) -> Result<(), anyhow::Error> {
        let mut line = String::with_capacity(4096);

        loop {
            let mut echo = true;

            line.clear();
            if self.reader.read_line(&mut line).await? == 0 {
                if self.next_buffer.is_empty() {
                    break;
                }

                match self.context {
                    ProcessingContext::Inline => self.flush_inline().await?,
                    ProcessingContext::Script | ProcessingContext::NeedScriptTerminator => {
                        self.flush_script().await?
                    }
                    _ => match self.prev_context {
                        ProcessingContext::Inline => self.flush_inline().await?,
                        ProcessingContext::Script | ProcessingContext::NeedScriptTerminator => {
                            self.flush_script().await?
                        }
                        _ => unreachable!(),
                    },
                }

                break;
            }

            if line.len() > 2 && line.starts_with("##") {
                let c = line.chars().nth(2).unwrap();
                match c {
                    '!' => {
                        self.set_context(ProcessingContext::Inline);
                        if matches!(
                            self.prev_context,
                            ProcessingContext::Script | ProcessingContext::NeedScriptTerminator
                        ) {
                            self.flush_script().await?;
                        }

                        let str: &str = &line[3..];
                        self.next_buffer.extend_from_slice(str.trim().as_bytes());
                        self.next_buffer.push(b' ');
                    }
                    '$' => {
                        self.set_context(ProcessingContext::Script);
                        if matches!(self.prev_context, ProcessingContext::NeedScriptTerminator) {
                            self.flush_script().await?;
                        }

                        self.next_buffer.extend_from_slice(line[3..].as_bytes());
                    }
                    '#' => {
                        self.flush_script().await?;
                        echo = false;
                    }
                    _ => {}
                }
            } else {
                match self.context {
                    ProcessingContext::Inline => {
                        self.flush_inline().await?;
                        echo = false;
                    }
                    ProcessingContext::Script => {
                        self.set_context(ProcessingContext::NeedScriptTerminator);
                        echo = false;
                    }
                    ProcessingContext::NeedScriptTerminator => {
                        echo = false;
                    }
                    _ => {}
                }
            }

            if echo {
                self.writer.write_all(line.trim_end().as_bytes()).await?;
                self.writer.write(b"\n").await?;
            }
        }

        self.writer.flush().await?;
        Ok(())
    }

    async fn flush_script(&mut self) -> Result<(), anyhow::Error> {
        let str = std::str::from_utf8(&self.next_buffer).unwrap().to_owned();
        // println!(
        //     "### Flushed({}, {}) ###\n{}",
        //     self.prev_context, self.context, str
        // );

        match execute_script(self.id, str).await {
            Ok(str) | Err(str) => {
                self.writer.write_all(str.as_bytes()).await?;
                if !str.is_empty() {
                    self.writer.write(b"\n").await?;
                }
            }
        }

        self.writer.write_all(b"###\n").await?;
        self.next_buffer.clear();
        self.set_context(ProcessingContext::None);

        Ok(())
    }

    async fn flush_inline(&mut self) -> Result<(), anyhow::Error> {
        let str = std::str::from_utf8(&self.next_buffer).unwrap();
        // println!(
        //     "### Flushed({}, {}) ###\n{}",
        //     self.prev_context, self.context, str
        // );

        match execute_script(self.id, format!("println(`{}`)", str)).await {
            Ok(str) | Err(str) => self.writer.write_all(str.as_bytes()).await?,
        }

        self.writer.write_all(b"\n").await?;
        self.next_buffer.clear();
        self.set_context(ProcessingContext::None);

        Ok(())
    }

    fn set_context(&mut self, ctx: ProcessingContext) {
        self.prev_context = self.context;
        self.context = ctx;
    }
}
