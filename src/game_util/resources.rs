use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::network::messages::{ClientMsg, GameState};

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);
//server
#[derive(Resource)]
pub struct Server {
    pub write: Option<Sender<ClientMsg>>,
    pub read: Option<Receiver<GameState>>,
    pub input: Option<Receiver<ClientMsg>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
            input: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct ServerPlayerPos {
    pub x: f32,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub position: Vec2,
    index: usize,
}
