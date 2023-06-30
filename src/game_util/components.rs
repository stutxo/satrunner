use bevy::{prelude::*, utils::Instant};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Player {
    pub moving: bool,
}

#[derive(Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub index: usize,
    pub last_input_time: Instant,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            index: 0,
            last_input_time: Instant::now(),
        }
    }
}

#[derive(Component)]
pub struct Particle();

#[derive(Component)]
pub struct Enemies();
