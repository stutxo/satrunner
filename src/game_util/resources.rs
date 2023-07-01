use std::collections::VecDeque;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use futures::channel::mpsc::{Receiver, Sender};

use crate::network::messages::{ClientMsg, GameState, Index};

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct ActivePlayers(pub HashMap<String, Index>);
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
