use std::collections::VecDeque;

use bevy::prelude::*;
use futures::channel::mpsc::{Receiver, Sender};

use crate::ClientMsg;

#[derive(Resource)]
pub struct DotPos(pub Vec<Dot>);

pub struct Dot(pub Vec3);

#[derive(Resource)]
pub struct PlayerPos(pub Vec3);

#[derive(Resource)]
pub struct ParticlePool(pub VecDeque<Entity>);

#[derive(Resource)]
pub struct Server {
    pub write: Option<Sender<ClientMsg>>,
    pub read: Option<Receiver<f32>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct ReceivedMessages {
    pub messages: Vec<f32>,
}
