use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};
use common::logging;

#[derive(Resource)]
pub struct Time {
    pub start_instant: Instant,
    pub last_time: i64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
            last_time: common::time::now_timestamp_s(),
        }
    }

    pub fn tick(&mut self) {
        self.last_time = common::time::now_timestamp_s();
    }

    pub fn delta(&self) -> Duration {
        self.start_instant.elapsed()
    }
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::new())
            .add_systems(PreUpdate, tick);
    }
}

fn tick(mut time: ResMut<Time>) {
    time.tick();
    logging::debug!("Tick: now({}), delta({})",
        time.last_time, time.delta().as_millis());
}
