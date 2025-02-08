mod commands;

use std::str::SplitWhitespace;
use common::command::ExecutableCommand;
use crate::AppState;

pub struct GameServerCommand(&'static AppState);

impl GameServerCommand {
    pub fn new(state: &'static AppState) -> Self { Self(state) }
}

macro_rules! create_game_server_command {
    ($(($m:ident::$func:ident):$help:literal;)*) => {
        fn help() -> String {
            let mut help_string = String::from("Game Server Commands: \n");
            $(
                let help_info = format!("{}\t{}\n", stringify!($func), $help);
                help_string.push_str(&help_info);
            )*

            help_string
        }

        #[async_trait::async_trait]
        impl ExecutableCommand for GameServerCommand {
            async fn execute<'a>(&self, mut args: SplitWhitespace<'a>) -> String {
                match args.next() {
                    $(
                        Some(stringify!($func)) => $m::$func(args, self.0).await,
                    )*
                    _ => help(),
                }
            }
        }
    };
}

create_game_server_command! {
    (commands::ship):"ship [uid] [ship_id | all]";
    (commands::skin):"skin [uid] [all]";
    (commands::ban):"ban [uid]";
    (commands::unban):"unban [uid]";
}
