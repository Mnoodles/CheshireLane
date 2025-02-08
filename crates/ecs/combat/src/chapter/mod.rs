use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct ChapterDataNotifyPlugin;

impl Plugin for ChapterDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyChapterDataEvent>()
            .add_systems(Update, notify_chapter_data);
    }
}

#[derive(Event)]
pub struct NotifyChapterDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_chapter_data(
    mut events: EventReader<NotifyChapterDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_chapter_info(),
                event.1,
                event.2,
                event.3);
        }
    }
}

pub struct CurrentChapterNotifyPlugin;

impl Plugin for CurrentChapterNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyCurrentChapterEvent>()
            .add_systems(Update, notify_current_chapter);
    }
}

#[derive(Event)]
pub struct NotifyCurrentChapterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_current_chapter(
    mut events: EventReader<NotifyCurrentChapterEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_current_chapter(),
                event.1,
                event.2,
                event.3);
        }
    }
}
