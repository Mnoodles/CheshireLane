mod connection_mgr;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use common::logging;
use ecs_activity::activity::notify::{NotifyActivityDataEvent, NotifyInstagramRegisterEvent};
use ecs_activity::activity::permanent::NotifyActivityPermanentRegisterEvent;
use ecs_activity::event::NotifyEventInfoEvent;
use ecs_activity::reflux::NotifyRefluxDataEvent;
use ecs_combat::chapter::{NotifyChapterDataEvent, NotifyCurrentChapterEvent};
use ecs_combat::fleet::NotifyFleetDataEvent;
use ecs_combat::world::notify::NotifyWorldDataEvent;
use ecs_database::SavePlayerDataEvent;
use ecs_inventory::bag::NotifyBagDataEvent;
use ecs_inventory::equip::equip_skin::NotifyEquipSkinDataEvent;
use ecs_inventory::equip::NotifyEquipDataEvent;
use ecs_inventory::ship::notify::NotifyShipDataEvent;
use ecs_inventory::ship::statistics::NotifyStatisticsEvent;
use ecs_inventory::skin::NotifyShipSkinDataEvent;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use ecs_player::plugin::{NotifyPlayerBuffEvent, NotifyPlayerDataEvent};
use ecs_system::bay::NotifyEnergyRecoverTimeEvent;
use ecs_system::build::notify::NotifyBuildShipEvent;
use ecs_system::commander::notify::NotifyCommanderDataEvent;
use ecs_system::daily::NotifyDailyLevelRegisterEvent;
use ecs_system::dorm::{NotifyApartmentRegisterEvent, NotifyDormDataEvent};
use ecs_system::game_room::NotifyGameRoomEvent;
use ecs_system::mail::compensate::NotifyCompensateRegisterEvent;
use ecs_system::mail::NotifyMailRegisterEvent;
use ecs_system::navy_academy::NotifyNavalAcademyEvent;
use ecs_system::notification::NotifyNotificationRegisterEvent;
use ecs_system::server_notice::NotifyServerNoticeRegisterEvent;
use ecs_system::shop::notify::NotifyShopMonthEvent;
use ecs_system::task::init::NotifyInitTaskInfoEvent;
use ecs_system::task::total_task::NotifyTotalTaskRegisterEvent;
use ecs_system::task::week_progress::NotifyWeekTaskProgressInfoEvent;
use ecs_technology::blueprint::NotifyBlueprintDataEvent;
use ecs_technology::tech_set::NotifyTechSetListsEvent;
use ecs_technology::technology::notify::NotifyTechnologyDataEvent;
use proto::p10::{Cs10022, Cs10024, Sc10023, Sc10025};
use proto::p11::{Cs11001, Sc11000, Sc11002};

#[derive(Event)]
pub struct FinishLoadPlayerData(pub u32, pub u16, pub u8, pub u8);

#[derive(Event)]
pub struct TimeSynchronization(pub u32, pub u16, pub u8, pub u8);

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FinishLoadPlayerData>()
            .add_event::<TimeSynchronization>()
            .add_systems(PreUpdate, server_login)
            .add_systems(PreUpdate, create_new_player)
            .add_systems(PreUpdate, load_player_data)
            .add_systems(PostUpdate, finish_load_player_data)
            .add_systems(Update, time_synchronization)
            .add_plugins(connection_mgr::ConnectionMgrPlugin);
    }
}

fn server_login(
    mut events: EventReader<ClientMessageEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs10022>() {
            logging::debug!("ServerLogin: {:?}", req);

            message_output.send(Sc10023 {
                db_load: None,
                result: 0,
                user_id: player.uid(),
                server_ticket: req.server_ticket,
                server_load: None,
            },
            message.id());
        }
    }
}

fn create_new_player(
    mut events: EventReader<ClientMessageEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
    mut save_player_data_event: EventWriter<SavePlayerDataEvent>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs10024>() {
            logging::debug!("CreateNewPlayer: {:?}", req);

            player.info().init(message.uid(), req.nick_name, req.ship_id);

            message_output.send(Sc10025 {
                result: 0,
                user_id: player.uid(),
            },
            message.id());

            save_player_data_event.send(SavePlayerDataEvent(player.uid()));
        }
    }
}

#[derive(SystemParam)]
pub struct NotifyEvents<'w> {
    pub notify_player_data_events: EventWriter<'w, NotifyPlayerDataEvent>,
    pub notify_reflux_data_event: EventWriter<'w, NotifyRefluxDataEvent>,
    pub notify_game_room_event: EventWriter<'w, NotifyGameRoomEvent>,
    pub notify_statistics_event: EventWriter<'w, NotifyStatisticsEvent>,
    pub notify_ship_data_event: EventWriter<'w, NotifyShipDataEvent>,
    pub notify_ship_skin_data_event: EventWriter<'w, NotifyShipSkinDataEvent>,
    pub notify_technology_data_event: EventWriter<'w, NotifyTechnologyDataEvent>,
    pub notify_blueprint_data_event: EventWriter<'w, NotifyBlueprintDataEvent>,
    pub notify_tech_set_lists_event: EventWriter<'w, NotifyTechSetListsEvent>,
    pub notify_fleet_data_event: EventWriter<'w, NotifyFleetDataEvent>,
    pub notify_shop_month_event: EventWriter<'w, NotifyShopMonthEvent>,
    pub notify_chapter_data_event: EventWriter<'w, NotifyChapterDataEvent>,
    pub notify_bag_data_event: EventWriter<'w, NotifyBagDataEvent>,
    pub notify_equip_data_event: EventWriter<'w, NotifyEquipDataEvent>,
    pub notify_equip_skin_data_event: EventWriter<'w, NotifyEquipSkinDataEvent>,
    pub notify_build_ship_event: EventWriter<'w, NotifyBuildShipEvent>,
    pub notify_activity_data_event: EventWriter<'w, NotifyActivityDataEvent>,
    pub notify_dorm_data_event: EventWriter<'w, NotifyDormDataEvent>,
    pub notify_naval_academy_event: EventWriter<'w, NotifyNavalAcademyEvent>,
    pub notify_world_data_event: EventWriter<'w, NotifyWorldDataEvent>,
    pub notify_player_buff_event: EventWriter<'w, NotifyPlayerBuffEvent>,
    pub notify_commander_data_event: EventWriter<'w, NotifyCommanderDataEvent>,
    pub notify_current_chapter_event: EventWriter<'w, NotifyCurrentChapterEvent>,
    pub notify_event_info_event: EventWriter<'w, NotifyEventInfoEvent>,
    pub notify_daily_level_register_event: EventWriter<'w, NotifyDailyLevelRegisterEvent>,
    pub notify_init_task_info_event: EventWriter<'w, NotifyInitTaskInfoEvent>,
    pub notify_week_task_progress_info_event: EventWriter<'w, NotifyWeekTaskProgressInfoEvent>,
    pub notify_total_task_register_event: EventWriter<'w, NotifyTotalTaskRegisterEvent>,
    pub notify_energy_recover_time_event: EventWriter<'w, NotifyEnergyRecoverTimeEvent>,
    pub notify_apartment_register_event: EventWriter<'w, NotifyApartmentRegisterEvent>,
    pub notify_mail_register_event: EventWriter<'w, NotifyMailRegisterEvent>,
    pub notify_compensate_register_event: EventWriter<'w, NotifyCompensateRegisterEvent>,
    pub notify_notification_register_event: EventWriter<'w, NotifyNotificationRegisterEvent>,
    pub notify_instagram_register_event: EventWriter<'w, NotifyInstagramRegisterEvent>,
    pub notify_activity_permanent_register_event:
        EventWriter<'w, NotifyActivityPermanentRegisterEvent>,
    pub notify_server_notice_register_event: EventWriter<'w, NotifyServerNoticeRegisterEvent>,
}

fn load_player_data(
    mut events: EventReader<ClientMessageEvent>,
    mut player: ResMut<Player>,
    mut time_synchronization: EventWriter<TimeSynchronization>,
    mut finish_load_player_data_events: EventWriter<FinishLoadPlayerData>,
    mut notify_events: NotifyEvents,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11001>() {
            logging::debug!("LoadPlayerData: {:?}", req);

            let extra_num = player.info().player_ships_data_chunk_extra_num();

            time_synchronization.send(TimeSynchronization(
                player.uid(),
                message.id(),
                1,
                38 + extra_num,
            ));

            // Notify Data
            notify_events
                .notify_player_data_events
                .send(NotifyPlayerDataEvent(
                    player.uid(),
                    message.id(),
                    2,
                    38 + extra_num,
                ));
            notify_events
                .notify_player_buff_event
                .send(NotifyPlayerBuffEvent(
                    player.uid(),
                    message.id(),
                    3,
                    38 + extra_num,
                ));
            notify_events
                .notify_reflux_data_event
                .send(NotifyRefluxDataEvent(
                    player.uid(),
                    message.id(),
                    4,
                    38 + extra_num,
                ));
            notify_events
                .notify_naval_academy_event
                .send(NotifyNavalAcademyEvent(
                    player.uid(),
                    message.id(),
                    5,
                    38 + extra_num,
                ));
            notify_events
                .notify_game_room_event
                .send(NotifyGameRoomEvent(
                    player.uid(),
                    message.id(),
                    6,
                    38 + extra_num,
                ));
            notify_events
                .notify_commander_data_event
                .send(NotifyCommanderDataEvent(
                    message.uid(),
                    message.id(),
                    7,
                    38 + extra_num,
                ));
            notify_events
                .notify_statistics_event
                .send(NotifyStatisticsEvent(
                    player.uid(),
                    message.id(),
                    8,
                    38 + extra_num,
                ));
            notify_events
                .notify_build_ship_event
                .send(NotifyBuildShipEvent(
                    player.uid(),
                    message.id(),
                    9,
                    38 + extra_num,
                ));
            notify_events
                .notify_ship_data_event
                .send(NotifyShipDataEvent(
                    player.uid(),
                    message.id(),
                    10,
                    38 + extra_num,
                ));
            notify_events
                .notify_fleet_data_event
                .send(NotifyFleetDataEvent(
                    player.uid(),
                    message.id(),
                    11 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_ship_skin_data_event
                .send(NotifyShipSkinDataEvent(
                    player.uid(),
                    message.id(),
                    12 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_technology_data_event
                .send(NotifyTechnologyDataEvent(
                    player.uid(),
                    message.id(),
                    13 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_blueprint_data_event
                .send(NotifyBlueprintDataEvent(
                    player.uid(),
                    message.id(),
                    14 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_tech_set_lists_event
                .send(NotifyTechSetListsEvent(
                    player.uid(),
                    message.id(),
                    15 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_chapter_data_event
                .send(NotifyChapterDataEvent(
                    player.uid(),
                    message.id(),
                    16 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_current_chapter_event
                .send(NotifyCurrentChapterEvent(
                    player.uid(),
                    message.id(),
                    17 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_event_info_event
                .send(NotifyEventInfoEvent(
                    player.uid(),
                    message.id(),
                    18 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_daily_level_register_event
                .send(NotifyDailyLevelRegisterEvent(
                    player.uid(),
                    message.id(),
                    19 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_shop_month_event
                .send(NotifyShopMonthEvent(
                    player.uid(),
                    message.id(),
                    20 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_world_data_event
                .send(NotifyWorldDataEvent(
                    player.uid(),
                    message.id(),
                    21 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_equip_data_event
                .send(NotifyEquipDataEvent(
                    player.uid(),
                    message.id(),
                    22 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_equip_skin_data_event
                .send(NotifyEquipSkinDataEvent(
                    player.uid(),
                    message.id(),
                    23 + extra_num,
                    38 + extra_num,
                ));
            notify_events.notify_bag_data_event.send(NotifyBagDataEvent(
                player.uid(),
                message.id(),
                24 + extra_num,
                38 + extra_num,
            ));
            notify_events
                .notify_init_task_info_event
                .send(NotifyInitTaskInfoEvent(
                    player.uid(),
                    message.id(),
                    25 + extra_num,
                    38 + extra_num,
                ));
            notify_events.notify_week_task_progress_info_event.send(
                NotifyWeekTaskProgressInfoEvent(
                    player.uid(),
                    message.id(),
                    26 + extra_num,
                    38 + extra_num,
                ),
            );
            notify_events
                .notify_total_task_register_event
                .send(NotifyTotalTaskRegisterEvent(
                    player.uid(),
                    message.id(),
                    27 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_dorm_data_event
                .send(NotifyDormDataEvent(
                    player.uid(),
                    message.id(),
                    28 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_energy_recover_time_event
                .send(NotifyEnergyRecoverTimeEvent(
                    player.uid(),
                    message.id(),
                    29 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_apartment_register_event
                .send(NotifyApartmentRegisterEvent(
                    player.uid(),
                    message.id(),
                    30 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_mail_register_event
                .send(NotifyMailRegisterEvent(
                    player.uid(),
                    message.id(),
                    31 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_compensate_register_event
                .send(NotifyCompensateRegisterEvent(
                    player.uid(),
                    message.id(),
                    32 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_notification_register_event
                .send(NotifyNotificationRegisterEvent(
                    player.uid(),
                    message.id(),
                    33 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_instagram_register_event
                .send(NotifyInstagramRegisterEvent(
                    player.uid(),
                    message.id(),
                    34 + extra_num,
                    38 + extra_num,
                ));
            notify_events
                .notify_activity_data_event
                .send(NotifyActivityDataEvent(
                    player.uid(),
                    message.id(),
                    35 + extra_num,
                    38 + extra_num,
                ));
            notify_events.notify_activity_permanent_register_event.send(
                NotifyActivityPermanentRegisterEvent(
                    player.uid(),
                    message.id(),
                    36 + extra_num,
                    38 + extra_num,
                ),
            );
            notify_events.notify_server_notice_register_event.send(
                NotifyServerNoticeRegisterEvent(
                    player.uid(),
                    message.id(),
                    37 + extra_num,
                    38 + extra_num,
                ),
            );

            finish_load_player_data_events.send(FinishLoadPlayerData(
                player.uid(),
                message.id(),
                38 + extra_num,
                38 + extra_num,
            ));
        }
    }
}

fn finish_load_player_data(
    mut events: EventReader<FinishLoadPlayerData>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            logging::debug!("FinishLoadPlayerData: {}", event.0);

            let monday_0oclock = "2020-11-23 07:00:00";
            let monday_0oclock = common::time::get_timestamp_s(monday_0oclock) as u32;
            let rsp = Sc11002 {
                ship_count: player.info().ship_count(),
                timestamp: common::time::now_timestamp_s() as u32,
                monday_0oclock_timestamp: monday_0oclock,
            };

            message_output.send_seq(rsp, event.1, event.2, event.3);
        }
    }
}

fn time_synchronization(
    mut events: EventReader<TimeSynchronization>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            logging::debug!("TimeSynchronization: {}", event.0);

            let monday_0oclock = "2020-11-23 07:00:00";
            let monday_0oclock = common::time::get_timestamp_s(monday_0oclock) as u32;
            let rsp = Sc11000 {
                timestamp: common::time::now_timestamp_s() as u32,
                monday_0oclock_timestamp: monday_0oclock,
            };

            message_output.send_seq(rsp, event.1, event.2, event.3);
        }
    }
}
