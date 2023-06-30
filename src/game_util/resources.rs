use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};

use crate::network::messages::{ClientMsg, GameState};

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

//enemies
#[derive(Resource)]
pub struct EnemiesPool(pub VecDeque<Entity>);
#[derive(Resource)]
pub struct EnemiesPos(pub Vec<f32>);

//local player
#[derive(Resource, Default)]
pub struct LocalPlayerPos {
    pub x: f32,
    pub index: usize,
}

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
