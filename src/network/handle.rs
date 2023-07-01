use bevy::{prelude::*, utils::Instant};

use crate::game_util::{
    components::{Player, Target},
    resources::{DotPos, Server, ServerPlayerPos},
};

pub fn handle_server(
    mut server: ResMut<Server>,
    mut dots: ResMut<DotPos>,
    mut query: Query<(&mut Player)>,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(message) = receive_rx.try_next() {
            if let Some(server_msg) = message {
                dots.0 = server_msg.dots;
                for mut player in query.iter_mut() {
                    // server_msg.player_pos;
                }
            }
        }
    }
}

pub fn new_player(
    mut commands: Commands,
    mut server: ResMut<Server>,
    mut query: Query<(&mut Target, &mut Player)>,
) {
    if let Some(ref mut receive_rx) = server.input {
        while let Ok(message) = receive_rx.try_next() {
            if let Some(client_msg) = message {
                // Check if a player with the same id already exists
                let mut player_exists = false;
                for (mut target, mut player) in query.iter_mut() {
                    if player.id == client_msg.id {
                        target.x = client_msg.input.x;
                        target.y = client_msg.input.y;
                        target.index += 1;
                        target.last_input_time = Instant::now();
                        player.moving = true;
                        player_exists = true;
                        break;
                    }
                }

                // If player does not exist, create a new one
                if !player_exists {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(0.5, 1.0)),
                                color: Color::RED,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(0., -50., 0.1)),
                            ..Default::default()
                        })
                        .insert(Player {
                            moving: false,
                            id: client_msg.id,
                            server_pos: 0.0,
                            server_index: 1,
                        })
                        .insert(Target {
                            x: client_msg.input.x,
                            y: client_msg.input.y,
                            index: client_msg.index,
                            last_input_time: Instant::now(),
                        });
                }
            }
        }
    }
}
