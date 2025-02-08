use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::common::Collectioninfo;
use proto::p13::Sc13002;

pub struct EventInfoNotifyPlugin;

impl Plugin for EventInfoNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyEventInfoEvent>()
            .add_systems(Update, notify_event_info);
    }
}

#[derive(Event)]
pub struct NotifyEventInfoEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_event_info(
    mut events: EventReader<NotifyEventInfoEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc13002 {
                    max_team: 2,
                    collection_list: [30107, 30102, 30101, 30103, 20113].iter()
                        .map(|id| Collectioninfo {
                            id: *id,
                            ..Default::default()
                        }).collect(),
                },
                event.1,
                event.2,
                event.3);
        }
    }
}
