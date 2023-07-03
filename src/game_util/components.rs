use bevy::{prelude::*, utils::Instant};
use uuid::Uuid;

//player stuff
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub index: usize,
    pub last_input_time: Instant,
    pub id: Uuid,
    pub server_pos: f32,
    pub server_index: usize,
}
#[derive(Component)]
pub struct LocalPlayer;

//dots stuff
#[derive(Component)]
pub struct Particle;
