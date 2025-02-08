use std::io::Write;
use std::str::SplitWhitespace;
use std::sync::Arc;
use rustyline_async::{Readline, ReadlineEvent, SharedWriter};
use tokio::sync::Mutex;
use crate::logging;

#[async_trait::async_trait]
pub trait ExecutableCommand {
    async fn execute<'a>(&self, args: SplitWhitespace<'a>) -> String;
}

pub struct CheshireCommandManager<T> {
    cmd: Arc<Mutex<Option<Arc<T>>>>,
}

impl<T: Send + Sync + ExecutableCommand + 'static> CheshireCommandManager<T> {
    pub fn new() -> Self {
        Self { cmd: Arc::new(Mutex::new(None)) }
    }

    pub async fn set_cmd(&mut self, cmd: T) {
        let mut self_cmd = self.cmd.lock().await;
        *self_cmd = Some(Arc::new(cmd));
    }

    pub fn run(&mut self, rl: (Readline, SharedWriter)) {
        let cheshire_cmd = self.cmd.clone();
        let (mut rl, mut out) = rl;
        tokio::spawn(async move {
            loop {
                let line = match rl.readline().await {
                    Ok(ReadlineEvent::Line(line)) => line,
                    Ok(ReadlineEvent::Eof) | Ok(ReadlineEvent::Interrupted) => {
                        logging::info!("Received Ctrl+C, shutting down...");
                        rl.flush().unwrap();
                        std::process::exit(0);
                    }
                    _ => continue,
                };

                let args = line.split_whitespace();
                let cmd_guard = cheshire_cmd.lock().await;
                if let Some(cmd) = &*cmd_guard {
                    let output = cmd.execute(args).await;

                    if let Err(e) = writeln!(&mut out, "{output}") {
                        logging::error!("{e}");
                    }
                } else {
                    logging::error!("Cmd not set for CommandManager");
                }

                rl.add_history_entry(line);
            }
        });
    }
}
