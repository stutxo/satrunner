use speedy::{Readable, Writable};
use std::collections::HashMap;
use uuid::Uuid;

// Network messages
#[derive(Readable, Writable, Debug, Clone)]
pub enum NetworkMessage {
    GameUpdate(NewPos),
    NewGame(NewGame),
    ScoreUpdate(Score),
    PlayerConnected(PlayerConnected),
    PlayerDisconnected(Uuid),
    Ping,
}

#[derive(Readable, Writable, Debug, Clone)]
pub enum ClientMessage {
    PlayerInput(PlayerInput),
    PlayerName(String),
}

#[derive(Readable, Writable, Debug, Clone, Default)]
pub struct NewPos {
    pub input: [f32; 2],
    pub tick: u64,
    pub id: Uuid,
    pub pos: f32,
    pub tick_adjustment: i64,
    pub adjustment_iteration: u64,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct PlayerInput {
    pub target: [f32; 2],
    pub id: Uuid,
    pub tick: u64,
}

impl PlayerInput {
    pub fn new(target: [f32; 2], id: Uuid, tick: u64) -> Self {
        Self { target, id, tick }
    }
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct NewGame {
    pub id: Uuid,
    pub server_tick: u64,
    pub rng_seed: u64,
    pub player_positions: HashMap<Uuid, PlayerPos>,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct PlayerPos {
    pub pos: Option<f32>,
    pub target: [f32; 2],
    pub score: usize,
    pub name: String,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct Score {
    pub id: Uuid,
    pub score: usize,
}

#[derive(Readable, Writable, Debug, Clone)]
pub struct PlayerConnected {
    pub id: Uuid,
    pub name: String,
}
