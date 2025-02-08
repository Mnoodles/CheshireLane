pub mod player_info;

use bevy_ecs::system::Resource;
use player_info::PlayerInfo;

#[derive(Resource, Clone)]
pub struct Player(#[allow(dead_code)] u32, PlayerInfo);

impl Player {
    pub fn new(uid: u32, info: PlayerInfo) -> Self { Self(uid, info) }

    pub fn uid(&self) -> u32 { self.1.uid() }

    pub fn info(&mut self) -> &mut PlayerInfo { &mut self.1 }
}
