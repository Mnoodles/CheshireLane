pub mod blueprint;
pub mod tech_set;
pub mod technology;

use bevy_app::prelude::*;

pub struct TechnologyPlugin;

impl Plugin for TechnologyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(technology::notify::TechnologyDataNotifyPlugin)
            .add_plugins(blueprint::BlueprintDataNotifyPlugin)
            .add_plugins(tech_set::TechSetListsNotifyPlugin)
            .add_systems(PreUpdate, technology::metadata_request);
    }
}
