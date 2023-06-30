use bevy::{prelude::*, utils::Instant};

#[derive(Component)]
pub struct Player {
    pub moving: bool,
    pub id: String,
}

#[derive(Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub index: usize,
    pub last_input_time: Instant,
}

impl Target {
    pub fn new() -> Self {
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
