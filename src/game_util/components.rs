use bevy::{prelude::*, utils::Instant};
use uuid::Uuid;

use crate::{game_core::movement::apply_input, network::messages::PlayerInfo};

use super::resources::TickManager;

//player stuff
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub input_index: usize,
    pub last_input_time: Instant,
    pub id: Uuid,
    pub server_pos: f32,
    pub server_index: usize,
    pub score: usize,
    pub pending_inputs: Vec<NewInput>,
}

impl Player {
    pub fn reconcile_server(
        &mut self,
        transform: &mut Transform,
        server: &PlayerInfo,
        server_tick: u64,
    ) {
        if (server.pos.x - transform.translation.x).abs() >= 1.0 {
            info!(
                "LOCATION DIFFERENT server_pos: {:?}, local pos {:?}",
                server.pos.x, transform.translation.x
            );
        }

        transform.translation.x = server.pos.x;

        let mut i = 0;

        while i < self.pending_inputs.len() {
            let input = &self.pending_inputs[i];
            info!(
                "inputs len: {:?}, server tick {:?}, inputs to clear {:?}, input {:?}",
                self.pending_inputs.len(),
                server_tick,
                i,
                input
            );

            if input.tick <= server_tick {
                self.pending_inputs.remove(i);
            } else {
                // Not processed by the server yet. Re-apply it.

                apply_input(self, transform);
                i += 1;
            }
        }
    }
}
#[derive(Component)]
pub struct LocalPlayer;

//dots stuff
#[derive(Component)]
pub struct Particle;

#[derive(Debug)]
pub struct NewInput {
    tick: u64,
    target: Vec2,
}

impl NewInput {
    pub fn new(tick: u64, target: Vec2) -> Self {
        Self { tick, target }
    }
}
