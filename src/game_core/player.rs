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
    pub pause: f64,
    pub adjust_iter: u64,
}

impl Player {
    pub fn server_reconciliation(
        &mut self,
        t: &mut Transform,
        recon_to_tick: u64,
        pos: f32,
        server_tick: u64,
    ) {
        self.pending_inputs
            .retain(|input| input.tick >= server_tick);

        t.translation.x = pos;
        for sim_tick in server_tick..=recon_to_tick {
            if let Some(tick_input) = self
                .pending_inputs
                .iter()
                .find(|input| input.tick == sim_tick)
            {
                self.target.x = tick_input.target[0];
                self.target.y = tick_input.target[1];
            }
            //info!("sim tick: {}, recon tick {}", sim_tick, recon_to_tick);
            self.apply_input(t);
        }
    }

    pub fn apply_input(&mut self, t: &mut Transform) {
        let movement = self.calculate_movement(t);

        if (t.translation.x + movement.x).abs() <= WORLD_BOUNDS
            && (t.translation.y + movement.y).abs() <= WORLD_BOUNDS
            && self.pause == 0.
        {
            t.translation += Vec3::new(movement.x, 0.0, 0.0);
        }
    }

    pub fn calculate_movement(&self, t: &Transform) -> Vec2 {
        let direction = self.target - Vec2::new(t.translation.x, t.translation.y);

        let tolerance = 0.5;

        if direction.length() > tolerance {
            direction.normalize() * PLAYER_SPEED
        } else {
            Vec2::ZERO
        }
    }
}
