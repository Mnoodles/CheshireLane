use bevy_ecs::prelude::*;
use tokio::sync::mpsc;
use common::logging;
use ecs_persistence::Player;

#[derive(Resource, Clone)]
pub struct Database(#[allow(dead_code)] u32, mpsc::Sender<(u32, serde_json::Value)>);

impl Database {
    pub fn new(uid: u32, tx: mpsc::Sender<(u32, serde_json::Value)>) -> Self {
        Self(uid, tx)
    }

    pub fn save(&self, uid: u32, value: serde_json::Value) {
        let _ = self.1.blocking_send((uid, value));
    }
}

#[derive(Event)]
pub struct SavePlayerDataEvent(pub u32);

pub fn save_player_data(
    mut events: EventReader<SavePlayerDataEvent>,
    mut player: ResMut<Player>,
    database: Res<Database>,
) {
    for event in events.read() {
        logging::debug!("SavePlayerDataEvent: {}", event.0);

        if let Ok(value) = serde_json::to_value(player.info()) {
            database.save(event.0, value);
        } else {
            logging::error!("Failed to serialize player info");
        }
    }
}
