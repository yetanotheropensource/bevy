use crate::time::Time;
use bevy_property::Properties;
use legion::prelude::{ComMut, Res};
use std::time::Duration;

#[derive(Clone, Debug, Default, Properties)]
pub struct Timer {
    pub elapsed: f32,
    pub duration: f32,
    pub finished: bool,
}

impl Timer {
    pub fn from_seconds(seconds: f32) -> Self {
        Timer {
            duration: seconds,
            ..Default::default()
        }
    }
    pub fn new(duration: Duration) -> Self {
        Timer {
            duration: duration.as_secs_f32(),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration);
        if self.elapsed >= self.duration {
            self.finished = true;
        }
    }

    pub fn reset(&mut self) {
        self.finished = false;
        self.elapsed = 0.0;
    }
}

pub fn timer_system(time: Res<Time>, mut timer: ComMut<Timer>) {
    timer.tick(time.delta_seconds);
}