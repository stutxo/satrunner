use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};
use uuid::Uuid;

use crate::network::messages::PlayerInput;

//dots
#[derive(Resource)]
pub struct DotPos(pub Vec<Vec3>);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
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
