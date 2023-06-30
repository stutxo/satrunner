use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use futures::channel::mpsc::{Receiver, Sender};

use crate::network::{
    handle::EnemyPos,
    messages::{ClientMsg, GameState},
};

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

//enemies
#[derive(Resource)]
pub struct EnemiesPool(pub Vec<Entity>);
#[derive(Resource)]
pub struct EnemyState(pub HashMap<Entity, EnemyPos>);

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
