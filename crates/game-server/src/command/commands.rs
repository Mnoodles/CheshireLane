use std::str::SplitWhitespace;
use ecs_command::CommandKind;
use crate::AppState;

pub async fn ship<'a>(mut args: SplitWhitespace<'a>, state: &'static AppState) -> String {
    let uid = args.next();
    let ship = args.next();

    if let (Some(uid), Some(ship)) = (uid, ship) {
        let Some(uid) = uid.parse::<u32>().ok() else {
            return "Invalid uid".to_string();
        };

        match ship {
            "all" => {
                state.simulator.add_command(uid, CommandKind::GiveAllShip);

                format!("Give UID {} ALL Ship", uid)
            },
            ship => {
                let Some(ship) = ship.parse::<u32>().ok() else {
                    return "Invalid ship ID".to_string();
                };
                state.simulator.add_command(uid, CommandKind::GiveShip(ship));

                format!("Give UID {} Ship {}", uid, ship)
            }
        }
    } else {
        "Command error, use `help` to list all available commands.".to_string()
    }
}

pub async fn skin<'a>(mut args: SplitWhitespace<'a>, state: &'static AppState) -> String {
    let uid = args.next();
    let skin = args.next();

    if let (Some(uid), Some(skin)) = (uid, skin) {
        let Some(uid) = uid.parse::<u32>().ok() else {
            return "Invalid uid".to_string();
        };
        match skin {
            "all" => {
                state.simulator.add_command(uid, CommandKind::GiveAllSkin);

                format!("Give UID {} ALL Skin", uid)
            },
            _ => {
                "Command error, use `help` to list all available commands.".to_string()
            }
        }
    } else {
        "Command error, use `help` to list all available commands.".to_string()
    }
}

pub async fn ban<'a>(mut args: SplitWhitespace<'a>, state: &'static AppState) -> String {
    let uid = args.next();

    if let Some(uid) = uid {
        let Some(uid) = uid.parse::<u32>().ok() else {
            return "Invalid uid".to_string();
        };
        let _ = state.db.ban(uid).await;

        format!("Ban UID {}", uid)
    } else {
        "Command error, use `help` to list all available commands.".to_string()
    }
}

pub async fn unban<'a>(mut args: SplitWhitespace<'a>, state: &'static AppState) -> String {
    let uid = args.next();

    if let Some(uid) = uid {
        let Some(uid) = uid.parse::<u32>().ok() else {
            return "Invalid uid".to_string();
        };
        let _ = state.db.unban(uid).await;

        format!("Unban UID {}", uid)
    } else {
        "Command error, use `help` to list all available commands.".to_string()
    }
}
