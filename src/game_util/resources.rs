use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};
use uuid::Uuid;

use crate::network::messages::PlayerInput;

//dots
#[derive(Resource)]
pub struct Dots {
    pub pos: Vec<Vec3>,
    pub rng_seed: Option<u64>,
    pub server_tick: u64,
    pub client_tick: u64,
}

impl Dots {
    pub fn new() -> Self {
        Self {
            pos: Vec::new(),
            rng_seed: None,
            server_tick: 0,
            client_tick: 0,
        }
    }
}

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct PlayerInit {
    pub id: Option<Uuid>,
}

impl PlayerInit {
    pub fn new() -> Self {
        Self { id: None }
    }
}

//server
#[derive(Resource)]
pub struct Server {
    pub write: Option<Sender<PlayerInput>>,
    pub read: Option<Receiver<String>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }
}
