use bevy::{prelude::*, utils::Instant};

//player stuff
#[derive(Component)]
pub struct Player {
    pub moving: bool,
    pub id: Option<String>,
    pub server_pos: f32,
    pub server_index: usize,
}

#[derive(Component)]
pub struct LocalPlayer;

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

//dots stuff
#[derive(Component)]
pub struct Particle;
