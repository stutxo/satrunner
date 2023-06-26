use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Resource)]
pub struct DotPos(pub Vec<Dot>);

pub struct Dot(pub Vec3);

#[derive(Resource)]
pub struct PlayerPos(pub Vec3);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);
