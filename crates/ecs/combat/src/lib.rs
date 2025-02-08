pub mod chapter;
pub mod fleet;
pub mod world;

use bevy_app::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fleet::FleetDataNotifyPlugin)
            .add_plugins(chapter::ChapterDataNotifyPlugin)
            .add_plugins(chapter::CurrentChapterNotifyPlugin)
            .add_plugins(world::notify::WorldDataNotifyPlugin)
            .add_systems(PreUpdate, world::get_world_boss);
    }
}
