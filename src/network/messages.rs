use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Network messages
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum NetworkMessage {
    GameUpdate(NewPos),
    NewInput(PlayerInput),
    NewGame(NewGame),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NewPos {
    pub pos: f32,
    pub tick: u64,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldUpdate {
    pub players: HashMap<Uuid, PlayerInfo>,
    pub rng_seed: u64,
    pub game_tick: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInfo {
    pub index: usize,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInput {
    pub target: Vec2,
    pub id: Uuid,
    pub tick: u64,
}

impl PlayerInput {
    pub fn new(target: Vec2, id: Uuid, tick: u64) -> Self {
        Self { target, id, tick }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewGame {
    pub id: Uuid,
    pub server_tick: u64,
}
