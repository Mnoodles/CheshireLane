use bevy_app::prelude::*;
use tokio::sync::mpsc;
use common::logging;
use ecs_activity::ActivityPlugin;
use ecs_combat::CombatPlugin;
use ecs_command::{CommandKind, CommandPlugin, CommandEvent};
use ecs_database::{save_player_data, Database, SavePlayerDataEvent};
use ecs_inventory::InventoryPlugin;
use ecs_login::LoginPlugin;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::{ClientOutput, MessageOutput};
use ecs_persistence::Player;
use ecs_persistence::player_info::PlayerInfo;
use ecs_player::PlayerPlugin;
use ecs_system::SystemPlugin;
use ecs_technology::TechnologyPlugin;
use ecs_time::TimePlugin;

pub struct PlayerWorld(App);

impl PlayerWorld {
    pub fn new(
        uid: u32,
        player_info: PlayerInfo,
        output: ClientOutput,
        save_data_tx: mpsc::Sender<(u32, serde_json::Value)>,
    ) -> Self {
        let message_out = MessageOutput::new(uid, output);
        let player = Player::new(uid, player_info);
        let database = Database::new(uid, save_data_tx);

        let mut app = App::new();
        app
            .insert_resource(message_out)
            .insert_resource(player)
            .insert_resource(database)
            .add_event::<ClientMessageEvent>()
            .add_event::<SavePlayerDataEvent>()
            .add_systems(PostUpdate, save_player_data);

        app
            .add_plugins(PlayerPlugin)
            .add_plugins(CommandPlugin)
            .add_plugins(LoginPlugin)
            .add_plugins(ActivityPlugin)
            .add_plugins(CombatPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(SystemPlugin)
            .add_plugins(TechnologyPlugin)
            .add_plugins(TimePlugin);

        app.finish();
        app.cleanup();
        app.update();

        logging::info!("created world for player: {}", uid);

        Self(app)
    }

    pub fn add_packet(&mut self, uid: u32, cmd_id: u16, id: u16, data: Box<[u8]>) {
        self.0.world_mut()
            .send_event(ClientMessageEvent::new(uid, cmd_id, id, data));
    }

    pub fn update(&mut self) {
        self.0.update();
    }

    pub fn serialize_player_information(&mut self, _uid: u32) -> Option<serde_json::Value> {
        let mut player = self.0.world_mut().get_resource::<Player>().unwrap().clone();

        let player_info = player.info();

        serde_json::to_value(player_info).ok()
    }

    pub fn add_command(&mut self, uid: u32, kind: CommandKind) {
        self.0.world_mut()
            .send_event(CommandEvent {
                executor_uid: uid,
                kind,
            });
    }
}
