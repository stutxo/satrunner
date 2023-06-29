use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};

use crate::{ClientMsg, GameState};

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

pub struct Dot(pub Vec3);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

//enemies
#[derive(Resource)]
pub struct EnemiesPool(pub VecDeque<Entity>);
#[derive(Resource)]
pub struct EnemiesPos(pub Vec<f32>);

//local player
#[derive(Resource)]
pub struct PlayerPos(pub Vec3);

#[derive(Resource, Default)]
pub struct LocalPlayerPos(pub f32);

//server
#[derive(Resource)]
pub struct Server {
    pub write: Option<Sender<ClientMsg>>,
    pub read: Option<Receiver<GameState>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }
}
