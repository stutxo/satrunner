use bevy::prelude::*;
use uuid::Uuid;

use crate::game_core::dots::{PLAYER_SPEED, WORLD_BOUNDS};

//player stuff
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub id: Uuid,
    pub score: usize,
    pub pending_inputs: Vec<NewInput>,
    pub client_tick: u64,
    pub server_tick: u64,
    pub server_pos: f32,
}

impl Player {
    pub fn reconcile_server(&mut self, t: &mut Transform) {
        self.pending_inputs
            .retain(|input| input.tick >= self.server_tick);

        let mut sim_tick = self.server_tick;
        let recon_to_tick = self.client_tick;

        while sim_tick < recon_to_tick {
            if let Some(input) = self
                .pending_inputs
                .iter()
                .find(|input| input.tick == sim_tick)
            {
                self.target = input.target;
                self.apply_input(t);
            } else {
                self.apply_input(t);
            }

            sim_tick += 1;
        }
    }

    pub fn apply_input(&mut self, t: &mut Transform) {
        let direction = self.target - Vec2::new(t.translation.x, t.translation.y);
        let distance_to_target = direction.length();

        let movement = if distance_to_target <= PLAYER_SPEED {
            direction
        } else {
            direction.normalize() * PLAYER_SPEED
        };

        let new_position = Vec2::new(t.translation.x, t.translation.y) + movement;

        if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
            t.translation += Vec2::new(movement.x, 0.0).extend(0.0);
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
    pub tick: u64,
    pub target: Vec2,
}

impl NewInput {
    pub fn new(tick: u64, target: Vec2) -> Self {
        Self { tick, target }
    }
}
