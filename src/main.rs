use bevy::prelude::*;
use game_core::{
    dots::{handle_dots, pool_dots},
    handle::handle_server,
    input::input,
    movement::move_players,
};
use game_util::resources::{Dots, NetworkStuff, ParticlePool, PlayerInit, TickManager};
use network::websockets::websocket;
use std::collections::VecDeque;

mod game_core;
mod game_util;
mod network;

pub const TICK_RATE: f32 = 1.0 / 30.0;

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
        .add_startup_systems((websocket, pool_dots))
        .add_systems((
            input,
            move_players
                .in_schedule(CoreSchedule::FixedUpdate)
                .before(handle_server),
            handle_server.in_schedule(CoreSchedule::FixedUpdate),
            handle_dots.in_schedule(CoreSchedule::FixedUpdate),
        ))
        .insert_resource(FixedTime::new_from_secs(TICK_RATE))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Dots::new())
        .insert_resource(ParticlePool(VecDeque::new()))
        .insert_resource(NetworkStuff::new())
        .insert_resource(PlayerInit::new())
        .insert_resource(TickManager::new())
        .run();
}
