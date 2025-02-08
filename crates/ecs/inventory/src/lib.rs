pub mod bag;
pub mod equip;
pub mod ship;
pub mod skin;

use bevy_app::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ship::statistics::StatisticsNotifyPlugin)
            .add_plugins(ship::notify::ShipDataNotifyPlugin)
            .add_plugins(skin::ShipSkinDataNotifyPlugin)
            .add_plugins(bag::BagDataNotifyPlugin)
            .add_plugins(equip::EquipDataNotifyPlugin)
            .add_plugins(equip::equip_skin::EquipSkinDataNotifyPlugin)
            .add_systems(PreUpdate, ship::meta_character_tactics_info_request)
            .add_systems(PreUpdate, ship::set_ship_skin);
    }
}
