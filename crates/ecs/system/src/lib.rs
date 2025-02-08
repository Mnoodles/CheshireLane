pub mod build;
pub mod dorm;
pub mod game_room;
pub mod navy_academy;
pub mod shop;
mod military_exercise;
mod guild;
mod educate;
pub mod commander;
pub mod daily;
pub mod task;
pub mod bay;
pub mod mail;
pub mod notification;
pub mod server_notice;
mod remaster;

use bevy_app::prelude::*;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(game_room::GameRoomNotifyPlugin)
            .add_plugins(build::notify::BuildShipNotifyPlugin)
            .add_plugins(dorm::DormDataNotifyPlugin)
            .add_plugins(navy_academy::NavalAcademyNotifyPlugin)
            .add_plugins(shop::notify::ShopMonthNotifyPlugin)
            .add_plugins(commander::notify::CommanderDataNotifyPlugin)
            .add_plugins(daily::DailyLevelRegisterNotifyPlugin)
            .add_plugins(task::init::InitTaskInfoPlugin)
            .add_plugins(task::week_progress::WeekTaskProgressInfoPlugin)
            .add_plugins(task::total_task::TotalTaskRegisterPlugin)
            .add_plugins(bay::EnergyRecoverTimePlugin)
            .add_plugins(dorm::ApartmentRegisterPlugin)
            .add_plugins(mail::MailRegisterPlugin)
            .add_plugins(mail::compensate::CompensateRegisterPlugin)
            .add_plugins(notification::NotificationRegisterPlugin)
            .add_plugins(server_notice::ServerNoticeRegisterPlugin)
            .add_systems(PreUpdate, military_exercise::get_season_info)
            .add_systems(PreUpdate, guild::get_guild_info)
            .add_systems(PreUpdate, guild::get_public_guild_user_data_create)
            .add_systems(PreUpdate, educate::educate_request)
            .add_systems(PreUpdate, educate::new_educate_request)
            .add_systems(PreUpdate, guild::get_user_info_create)
            .add_systems(PreUpdate, educate::get_endings)
            .add_systems(PreUpdate, commander::get_commander_home)
            .add_systems(PreUpdate, shop::get_charge_list)
            .add_systems(PreUpdate, remaster::remaster_info_request)
            .add_systems(PreUpdate, shop::sell_item)
            .add_systems(PreUpdate, task::trigger_task);
    }
}
