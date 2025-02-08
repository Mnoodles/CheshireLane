pub use tracing::{Level, info, debug, warn, error, trace};

use rustyline_async::SharedWriter;
use rustyline_async::Readline;
use tracing_subscriber::fmt::MakeWriter;

pub fn init(level: Level) {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .with_level(true)
        .init();
}

struct CheshireWriter {
    out: Option<SharedWriter>,
}

impl<'a> MakeWriter<'a> for CheshireWriter {
    type Writer = Box<dyn std::io::Write>;

    fn make_writer(&'a self) -> Self::Writer {
        match &self.out {
            None => Box::new(std::io::stdout()),
            Some(out) => Box::new(out.clone()),
        }
    }
}

pub fn init_with_readline(level: Level) -> Option<(Readline, SharedWriter)> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    let rl = Readline::new(String::from(">> ")).ok();
    let out = rl.as_ref()
        .map(|(_, out)| out.clone());

    tracing_subscriber::fmt()
        .with_writer(CheshireWriter { out })
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .with_level(true)
        .init();

    rl
}
