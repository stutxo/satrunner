use std::collections::VecDeque;

use bevy::{
    prelude::*,
    time::Stopwatch,
    utils::{HashMap, Instant},
};
use uuid::Uuid;

use crate::{game_util::resources::ClientTick, network::messages::PlayerInput};

use super::objects::{X_BOUNDS, Y_BOUNDS};

pub const PLAYER_SPEED: f32 = 2.5;
#[derive(Component)]
pub struct Player {
    pub target: Vec2,
    pub id: Uuid,
    pub score: usize,
    pub pending_inputs: Vec<PlayerInput>,
    pub adjust_iter: u64,
    pub name: String,
    pub spawn_time: Option<Instant>,
    pub death_time: Option<u64>,
}

impl Player {
    pub fn server_reconciliation(
        &mut self,
        t: &mut Transform,
        client_tick: &ClientTick,
        pos: [f32; 2],
        server_tick: u64,
    ) {
        self.pending_inputs
            .retain(|input| input.tick >= server_tick);

        t.translation.x = pos[0];
        t.translation.y = pos[1];

        for sim_tick in server_tick..client_tick.tick.unwrap() {
            if let Some(tick_input) = self
                .pending_inputs
                .iter()
                .find(|input| input.tick == sim_tick)
            {
                self.target.x = tick_input.target[0];
                self.target.y = tick_input.target[1];
            }
            self.apply_input(t, client_tick);
        }
    }

    pub fn apply_input(&mut self, t: &mut Transform, client_tick: &ClientTick) {
        let movement = self.calculate_movement(t);
        if (t.translation.x + movement.x).abs() <= X_BOUNDS
            && (t.translation.y + movement.y).abs() <= Y_BOUNDS
            && client_tick.pause == 0
        {
            t.translation += Vec3::new(movement.x, movement.y, 0.2);
        }
    }

    pub fn calculate_movement(&self, t: &Transform) -> Vec2 {
        let direction = self.target - Vec2::new(t.translation.x, t.translation.y);

        let tolerance = 6.0;

        if direction.length() > tolerance {
            let mut speed = PLAYER_SPEED;

            if direction.y < 0.0 {
                speed *= 2.0;
            }

            direction.normalize() * speed
        } else {
            Vec2::ZERO
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub target: Vec2,
    pub id: Uuid,
    pub score: usize,
    pub name: String,
    pub spawn_time: Stopwatch,
    pub past_pos: HashMap<u64, Vec3>,
    pub pending_inputs: VecDeque<PlayerInput>,
    pub dead: bool,
}

impl Enemy {
    pub fn enemy_reconciliation(
        &mut self,
        t: &mut Transform,
        client_tick: &ClientTick,
        pos: [f32; 2],
        enemy_tick: u64,
    ) {
        t.translation.x = pos[0];
        t.translation.y = pos[1];

        for _ in enemy_tick..client_tick.tick.unwrap() {
            self.apply_input(t, client_tick);
        }
    }

    pub fn apply_input(&mut self, t: &mut Transform, client_tick: &ClientTick) {
        let movement = self.calculate_movement(t);

        if (t.translation.x + movement.x).abs() <= X_BOUNDS
            && (t.translation.y + movement.y).abs() <= Y_BOUNDS
            && client_tick.pause == 0
        {
            t.translation += Vec3::new(movement.x, movement.y, 0.0);
        }
    }

    pub fn calculate_movement(&self, t: &Transform) -> Vec2 {
        let direction = self.target - Vec2::new(t.translation.x, t.translation.y);

        let tolerance = 6.0;

        if direction.length() > tolerance {
            let mut speed = PLAYER_SPEED;

            if direction.y < 0.0 {
                speed *= 2.0;
            }

            direction.normalize() * speed
        } else {
            Vec2::ZERO
        }
    }
}
