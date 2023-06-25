use bevy::prelude::*;

#[derive(Resource)]
pub struct DotPos {
    pub dots: Vec<Dot>,
}

pub struct Dot {
    pub pos: Vec3,
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct PlayerPos {
    pub pp: Vec3,
}

#[derive(Resource)]
pub struct ParticlePool {
    pub particles: Vec<Entity>,
}
