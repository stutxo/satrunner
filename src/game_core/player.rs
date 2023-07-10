use bevy::prelude::*;
use uuid::Uuid;

use crate::{game_core::dots::WORLD_BOUNDS, network::messages::PlayerInput};

pub const PLAYER_SPEED: f32 = 1.0;

#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub id: Uuid,
    pub score: usize,
    pub pending_inputs: Vec<PlayerInput>,
    pub server_tick: u64,
    pub recon_target: Vec2,
}

impl Player {
    pub fn server_reconciliation(&mut self, t: &mut Transform, recon_to_tick: u64, pos: f32) {
        if self.server_tick > recon_to_tick {
            t.translation.x = pos;
            info!("we are behind server!!! aaaa")
        }
        // self.pending_inputs
        //     .retain(|input| input.tick >= self.server_tick);

        //for every tick between server tick and client tick, check if input was the same as server tick
        // for sim_tick in self.server_tick..=recon_to_tick {

        if let Some(tick_input) = self
            .pending_inputs
            .iter()
            .find(|input| input.tick == self.server_tick)
        {
            info!("found input for tick: {}", tick_input.tick);
            if self.recon_target != tick_input.target {
                t.translation.x = pos;
                info!(
                    " input mismatch: server tick: {}, recon target {:?}, input target {:?}",
                    self.server_tick, self.recon_target, tick_input.target
                );
            }
        }
        //self.apply_input(t);
        // }
    }

    pub fn apply_input(&mut self, t: &mut Transform) {
        let movement = self.calculate_movement(t);

        if (t.translation.x + movement.x).abs() <= WORLD_BOUNDS
            && (t.translation.y + movement.y).abs() <= WORLD_BOUNDS
        {
            t.translation += Vec3::new(movement.x, 0.0, 0.0);
            //  info!("recon side pos: {:?}", t.translation.x);
        }
    }

    pub fn client_side_prediction(&mut self, t: &mut Transform) {
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
