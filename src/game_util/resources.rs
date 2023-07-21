use std::collections::VecDeque;

use bevy::{prelude::*, utils::Instant};
use futures::{
    channel::mpsc::{Receiver, Sender},
    future::Fuse,
};

use crate::network::messages::ClientMessage;

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
    pub write: Option<Sender<ClientMessage>>,
    pub read: Option<Receiver<Vec<u8>>>,
    pub disconnected: Option<Fuse<futures::channel::oneshot::Receiver<()>>>,
}

impl NetworkStuff {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
            disconnected: None,
        }
    }
}

#[derive(Resource)]
pub struct ClientTick {
    pub tick: Option<u64>,
    pub time: Instant,
    pub pause: i64,
}

impl ClientTick {
    pub fn new() -> Self {
        Self {
            tick: None,
            time: Instant::now(),
            pause: 0,
        }
    }
}

#[derive(Resource)]
pub struct PlayerName {
    pub name: String,
    pub submitted: bool,
}

impl PlayerName {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            submitted: false,
        }
    }
}
