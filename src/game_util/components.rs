use bevy::{prelude::*, utils::Instant};
use uuid::Uuid;

use crate::{game_core::movement::apply_input, network::messages::PlayerInfo};

//player stuff
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub input_index: usize,
    pub last_input_time: Instant,
    pub id: Uuid,
    pub server_pos: f32,
    pub server_tick: u64,
    pub score: usize,
    pub pending_inputs: Vec<NewInput>,
    pub client_tick: u64,
}

impl Player {
    pub fn reconcile_server(&mut self, transform: &mut Transform) {
        self.pending_inputs
            .retain(|input| input.tick >= self.server_tick);

        let mut sim_tick = self.server_tick;
        let recon_to_tick = self.client_tick;
        info!(
            "tick {:?}, before pos: {:?}",
            self.client_tick, transform.translation.x
        );
        while sim_tick < recon_to_tick {
            //info!("recon tick {:?}, sim tick {:?}", recon_to_tick, sim_tick,);
            if let Some(input) = self
                .pending_inputs
                .iter()
                .find(|input| input.tick == sim_tick)
            {
                self.target = input.target;
                apply_input(self, transform);
            } else {
                // info!("no input for tick {:?}", sim_tick);
                apply_input(self, transform);

                // No input for this tick. Just apply the current velocity
            }
            sim_tick += 1;
        }
        info!(
            "after tick {:?}, pos: {:?}",
            self.client_tick, transform.translation.x
        );
    }
}

#[derive(Component)]
pub struct LocalPlayer;

//dots stuff
#[derive(Component)]
pub struct Particle;

#[derive(Debug)]
pub struct NewInput {
    pub tick: u64,
    pub target: Vec2,
}

impl NewInput {
    pub fn new(tick: u64, target: Vec2) -> Self {
        Self { tick, target }
    }
}
