use bevy::prelude::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMsg {
    ServerMsg(GameState),
    ClientMsg(ClientMsg),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientMsg {
    pub input: InputVec2,
    pub index: usize,
    pub id: Option<String>,
}

impl ClientMsg {
    pub fn new(input: InputVec2, index: usize) -> Self {
        Self {
            input,
            index,
            id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub players_pos: Vec<Index>,
    pub dots: Vec<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub position: Vec2,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

impl InputVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
