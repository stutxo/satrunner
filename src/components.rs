use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub moving: bool,
}

#[derive(Default, Reflect, Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Particle();
