use std::collections::HashMap;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;
use tokio::sync::mpsc;
use common::logging;
use ecs_command::CommandKind;
use ecs_message::output::ClientOutput;
use ecs_persistence::player_info::PlayerInfo;
use crate::player_world::PlayerWorld;
use crate::command::LogicCommand;

#[derive(Clone)]
pub struct LogicSimulator(std::sync::mpsc::Sender<LogicCommand>);

impl LogicSimulator {
    pub fn spawn(save_data_tx: mpsc::Sender<(u32, serde_json::Value)>) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(|| simulation_loop(save_data_tx, rx));
        Self(tx)
    }

    pub fn create_world(&self, uid: u32, player_information: PlayerInfo, output: ClientOutput) {
        self.0.send(LogicCommand::CreateWorld {
            uid,
            player_info: player_information,
            out: output,
        }).unwrap();
    }

    pub fn add_client_packet(
        &self,
        uid: u32,
        cmd_id: u16,
        id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    ) {
        self.0.send(LogicCommand::ClientInput {
            uid,
            cmd_id,
            id,
            data,
            immediate_mode,
        }).unwrap();
    }

    pub fn update_world(&self, uid: u32) {
        self.0.send(LogicCommand::WorldUpdate(uid)).unwrap();
    }

    pub fn add_command(&self, uid: u32, kind: CommandKind) {
        self.0.send(LogicCommand::ExecuteCommand {
            uid,
            kind,
        }).unwrap();
    }
}

fn simulation_loop(
    save_data_tx: mpsc::Sender<(u32, serde_json::Value)>,
    command_rx: std::sync::mpsc::Receiver<LogicCommand>,
) {
    let mut player_world_map: HashMap<u32, PlayerWorld> = HashMap::new();
    let mut player_save_time_map: HashMap<u32, i64> = HashMap::new();

    let tick_interval = Duration::from_secs(5);

    // Use `recv` for normal message receiving.
    // Here, we use `recv_timeout` to set a timeout duration,
    // which updates all the worlds upon timeout. (`Tick`)
    loop {
        match command_rx.recv_timeout(tick_interval) {
            Ok(LogicCommand::CreateWorld {
                uid,
                player_info,
                out,
            }) => {
                player_save_time_map.insert(uid, common::time::now_timestamp_s());
                player_world_map.insert(
                    uid,
                    PlayerWorld::new(uid, player_info, out, save_data_tx.clone()),
                );
            },
            Ok(LogicCommand::ClientInput {
                uid,
                cmd_id,
                id,
                data,
                immediate_mode,
            }) => {
                if let Some(world) = player_world_map.get_mut(&uid) {
                    world.add_packet(uid, cmd_id, id, data);
                    if immediate_mode {
                        world.update();
                    }

                    let save_time = player_save_time_map.get_mut(&uid).unwrap();
                    let cur_time = common::time::now_timestamp_s();
                    if (cur_time - *save_time) >= 30 {
                        *save_time = cur_time;
                        let _ = save_data_tx.blocking_send((
                            uid,
                            world.serialize_player_information(uid).unwrap()
                        ));
                    }
                }
            },
            Ok(LogicCommand::WorldUpdate(uid)) => {
                if let Some(world) = player_world_map.get_mut(&uid) {
                    world.update();
                }
            },
            Ok(LogicCommand::ExecuteCommand { uid, kind }) => {
                if let Some(world) = player_world_map.get_mut(&uid) {
                    world.add_command(uid, kind);
                    world.update();
                } else {
                    // TODO: save_data_tx.send(...)
                    logging::warn!("Player not login, change player data in database");
                }
            },
            Err(RecvTimeoutError::Timeout) => {
                // Timeout, update all the worlds (`Tick`)
                for world in player_world_map.values_mut() {
                    world.update();
                }
            },
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }
}
