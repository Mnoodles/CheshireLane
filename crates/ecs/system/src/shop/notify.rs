use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p16::Sc16200;

pub struct ShopMonthNotifyPlugin;

impl Plugin for ShopMonthNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyShopMonthEvent>()
            .add_systems(Update, notify_shop_month_academy);
    }
}

#[derive(Event)]
pub struct NotifyShopMonthEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_shop_month_academy(
    mut events: EventReader<NotifyShopMonthEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc16200 {
                    month: common::time::get_month_timestamp_s() as u32,
                    ..Default::default()
                },
                event.1,
                event.2,
                event.3);
        }
    }
}
