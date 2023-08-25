use std::collections::VecDeque;

use bevy::{prelude::*, utils::Instant};
use futures::channel::mpsc::{Receiver, Sender};
use uuid::Uuid;

use crate::{game_core::objects::ObjectPos, network::messages::ClientMessage};

#[derive(Resource)]
pub struct Objects {
    pub rain_pos: Vec<ObjectPos>,
    pub bolt_pos: Vec<ObjectPos>,
    pub rng_seed: Option<u64>,
    pub high_scores: Vec<(String, u64)>,
}

impl Objects {
    pub fn new() -> Self {
        Self {
            rain_pos: Vec::new(),
            bolt_pos: Vec::new(),
            rng_seed: None,
            high_scores: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct RainPool(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct BoltPool(pub VecDeque<Entity>);

//server
#[derive(Resource)]
pub struct NetworkStuff {
    pub write: Option<Sender<ClientMessage>>,
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
    pub tick: Option<u64>,
    pub pause: i64,
}

impl ClientTick {
    pub fn new() -> Self {
        Self {
            tick: None,
            pause: 0,
        }
    }
}

#[derive(Resource)]
pub struct PingTimer {
    pub disconnected_rx: Option<Receiver<()>>,
    pub disconnected_tx: Option<Sender<()>>,
}

impl PingTimer {
    pub fn new() -> Self {
        Self {
            disconnected_rx: None,
            disconnected_tx: None,
        }
    }
}

#[derive(Resource)]
pub struct PlayerName {
    pub name: String,
    pub submitted: bool,
    pub id: Option<Uuid>,
}

impl PlayerName {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            submitted: false,
            id: None,
        }
    }
}
