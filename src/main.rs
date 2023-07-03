use bevy::prelude::*;
use game_core::{
    input::input,
    movement::{move_dot, move_local},
    setup::{internal_server, out_server, setup},
};
use game_util::resources::{DotPos, ParticlePool, PlayerInit, Server};
use network::{handle::handle_server, websockets::websocket};
use std::collections::VecDeque;

mod game_core;
mod game_util;
mod network;

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
        .add_startup_systems((websocket, setup))
        .add_systems((
            input,
            internal_server.in_schedule(CoreSchedule::FixedUpdate),
            out_server.in_schedule(CoreSchedule::FixedUpdate),
            handle_server.in_schedule(CoreSchedule::FixedUpdate),
            move_local.in_schedule(CoreSchedule::FixedUpdate),
            move_dot.in_schedule(CoreSchedule::FixedUpdate),
        ))
        .insert_resource(FixedTime::new_from_secs(1. / 30.))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DotPos(Vec::new()))
        .insert_resource(ParticlePool(VecDeque::new()))
        .insert_resource(Server::new())
        .insert_resource(PlayerInit::new())
        .run();
}
