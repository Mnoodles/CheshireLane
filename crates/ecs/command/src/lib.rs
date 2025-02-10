use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_persistence::Player;
use common::logging;
use data::ship_data_template_data;
use ecs_database::SavePlayerDataEvent;
use ecs_inventory::ship::notify::NotifyShipDataEvent;
use ecs_inventory::skin::NotifyShipSkinDataEvent;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CommandEvent>()
            .add_systems(PreUpdate, command_handler);
    }
}

#[derive(Event)]
pub struct CommandEvent {
    pub executor_uid: u32,
    pub kind: CommandKind,
}

#[derive(Debug)]
pub enum CommandKind {
    GiveShip(u32),
    GiveAllShip,
    GiveAllSkin,
}

pub fn command_handler(
    mut events: EventReader<CommandEvent>,
    mut player: ResMut<Player>,
    mut notify_ship_data_event: EventWriter<NotifyShipDataEvent>,
    mut notify_ship_skin_data_event: EventWriter<NotifyShipSkinDataEvent>,
    mut save_player_data_event: EventWriter<SavePlayerDataEvent>,
) {
    for command in events.read() {
        logging::debug!("ECSCommand: uid({}), kind({:?})",
            command.executor_uid, command.kind);

        if player.uid() == command.executor_uid {
            match command.kind {
                CommandKind::GiveShip(ship_id) => {
                    if let Some(db_data) = ship_data_template_data::DATA.get() {
                        if db_data.0.keys().any(|id| id.eq(&ship_id.to_string())) {
                            logging::debug!("GiveShip({})", ship_id);
                            player.info().add_ship(ship_id);
                        }
                    }

                    notify_ship_data_event
                        .send(NotifyShipDataEvent(player.uid(), 0, 1, 1));
                },
                CommandKind::GiveAllShip => {
                    if let Some(db_data) = ship_data_template_data::DATA.get() {
                        let mut ship_num = 0;
                        db_data.0.iter().for_each(|(key, ship)| {
                            if ship.star == ship.star_max && ship.star >= 5 {
                                if let Ok(id) = key.parse::<u32>() {
                                    ship_num += 1;
                                    player.info().add_ship(id);
                                }
                            }
                        });
                        logging::debug!("GiveAllShip, Ship Number = {}", ship_num);
                    }

                    let extra_num = player.info().player_ships_data_chunk_extra_num();
                    notify_ship_data_event
                        .send(NotifyShipDataEvent(player.uid(), 0, 1, 1 + extra_num));
                },
                CommandKind::GiveAllSkin => {
                    logging::debug!("GiveAllSkin");
                    let ship_ids: Vec<u32> = player.info().ships.iter()
                        .map(|ship| ship.template_id)
                        .collect();
                    ship_ids.iter().for_each(|id| player.info().add_ship_skin(*id));

                    notify_ship_skin_data_event
                        .send(NotifyShipSkinDataEvent(player.uid(), 0, 1, 1));
                },
            }
        }

        save_player_data_event.send(SavePlayerDataEvent(player.uid()));
    }
}
