use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    game_core::dots::{PLAYER_SPEED, WORLD_BOUNDS},
    network::messages::PlayerInput,
};

// player stuff
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub id: Uuid,
    pub score: usize,
    pub pending_inputs: Vec<PlayerInput>,
    pub client_tick: u64,
    pub server_tick: u64,
}

impl Player {
    pub fn server_reconciliation(&mut self, t: &mut Transform) {
        self.pending_inputs
            .retain(|input| input.tick >= self.server_tick);

        let recon_to_tick = self.client_tick;

        for sim_tick in self.server_tick..=recon_to_tick {
            if let Some(input) = self
                .pending_inputs
                .iter()
                .find(|input| input.tick == sim_tick)
            {
                self.target = input.target;
            }
            self.apply_input(t);
        }
    }

    pub fn apply_input(&mut self, t: &mut Transform) {
        let movement = self.calculate_movement(t);

        if (t.translation.x + movement.x).abs() <= WORLD_BOUNDS
            && (t.translation.y + movement.y).abs() <= WORLD_BOUNDS
        {
            t.translation += Vec3::new(movement.x, 0.0, 0.0);
        }
    }

    pub fn calculate_movement(&self, t: &Transform) -> Vec2 {
        let direction = self.target - Vec2::new(t.translation.x, t.translation.y);
        let distance_to_target = direction.length();

        if distance_to_target <= PLAYER_SPEED {
            direction
        } else {
            direction.normalize() * PLAYER_SPEED
        }
    }
}

#[derive(Component)]
pub struct LocalPlayer;

// dots stuff
#[derive(Component)]
pub struct Particle;
