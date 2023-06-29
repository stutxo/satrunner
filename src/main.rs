use std::collections::VecDeque;

use bevy::prelude::*;

use game_engine::*;
use input::*;
use messages::*;
use networking::*;
use resources::*;
use setup::*;

mod components;
mod game_engine;
mod input;
mod messages;
mod networking;
mod resources;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sat Runner".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FixedTime::new_from_secs(1. / 30.))
        .add_startup_systems((setup, websocket))
        .add_systems((
            input.in_schedule(CoreSchedule::FixedUpdate),
            move_local.in_schedule(CoreSchedule::FixedUpdate),
            handle_server.in_schedule(CoreSchedule::FixedUpdate),
            move_enemies.in_schedule(CoreSchedule::FixedUpdate),
            move_dot.in_schedule(CoreSchedule::FixedUpdate),
        ))
        .insert_resource(DotPos(Vec::new()))
        .insert_resource(EnemiesPos(Vec::new()))
        .insert_resource(PlayerPos(Vec3::new(0., -50., 0.1)))
        .insert_resource(ParticlePool(VecDeque::new()))
        .insert_resource(EnemiesPool(VecDeque::new()))
        .insert_resource(Server::new())
        .insert_resource(LocalPlayerPos::default())
        .run();
}
