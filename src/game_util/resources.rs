use std::collections::VecDeque;

use bevy::{prelude::*, utils::Instant};
use futures::channel::mpsc::{Receiver, Sender};
//use uuid::Uuid;

use crate::network::messages::PlayerInput;

//dots
#[derive(Resource)]
pub struct Dots {
    pub pos: Vec<Vec3>,
    pub rng_seed: Option<u64>,
}

impl Dots {
    pub fn new() -> Self {
        Self {
            pos: Vec::new(),
            rng_seed: None,
        }
    }
}

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

//server
#[derive(Resource)]
pub struct NetworkStuff {
    pub write: Option<Sender<PlayerInput>>,
    pub read: Option<Receiver<Vec<u8>>>,
}

impl NetworkStuff {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }
}

#[derive(Resource)]
pub struct ClientTick {
    pub tick: u64,
    pub time: Instant,
}

impl ClientTick {
    pub fn new() -> Self {
        Self {
            tick: 0,
            time: Instant::now(),
        }
    }
}
