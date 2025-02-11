use bevy_ecs::change_detection::{ResMut, Res};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{EventWriter, Resource};
use common::logging;
use ecs_command::{CommandEvent, CommandKind};
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use ecs_time::Time;
use proto::common::Displayinfo;
use proto::p50::{Cs50102, PlayerInfo, Sc50101};

pub fn client_send_message(
    mut events: EventReader<ClientMessageEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
    mut command_event: EventWriter<CommandEvent>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs50102>() {
            logging::debug!("ClientSendMsg: {:?}", req);

            let line = req.content;
            let args = line.split_whitespace().collect::<Vec<_>>();
            let content = if args.len() > 0 {
                match args[0] {
                    "ship" => {
                        command_event.send(CommandEvent{
                            executor_uid: player.uid(),
                            kind: CommandKind::GiveAllShip,
                        });
                        r#"Already give all ships to you~ Have Fun!"#
                    },
                    "skin" => {
                        command_event.send(CommandEvent{
                            executor_uid: player.uid(),
                            kind: CommandKind::GiveAllSkin,
                        });
                        r#"Already give all skins to you~ Have Fun!"#
                    },
                    "help" | _ => {
                        r#"1. ship: Give all ships. 2. skin: Give all skins"#
                    },
                }
            } else {
                r#"Command Error, using `help` to show all available commands."#
            };

            message_output
                .send(Sc50101 {
                    content: content.to_string(),
                    r#type: 0,
                    player: PlayerInfo {
                        id: 0,
                        lv: 150,
                        name: "Server".to_string(),
                        display: Some(Displayinfo {
                            skin: 701042,
                            icon_frame: 501,
                            chat_frame: 103,
                            icon: 701044,
                            icon_theme: 0,
                            marry_flag: 1580000000,
                            transform_flag: 0,
                        }),
                    },
                }, 0);
        }
    }
}

#[derive(Resource, Default)]
pub struct ScheduledMsg(pub u64);

pub fn scheduled_message(
    time: Res<Time>,
    mut scheduled_msg: ResMut<ScheduledMsg>,
    mut message_output: ResMut<MessageOutput>,
) {
    if time.delta().as_secs() - scheduled_msg.0 > 60 {
        scheduled_msg.0 = time.delta().as_secs();

        message_output
            .send(Sc50101 {
                content: "Welcome to CheshireLane".to_string(),
                r#type: 0,
                player: PlayerInfo {
                    id: 0,
                    lv: 150,
                    name: "ServerNotify".to_string(),
                    display: Some(Displayinfo {
                        skin: 701042,
                        icon_frame: 501,
                        chat_frame: 103,
                        icon: 701044,
                        icon_theme: 0,
                        marry_flag: 1580000000,
                        transform_flag: 0,
                    }),
                },
            }, 0);
    }
}
