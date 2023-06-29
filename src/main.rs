use std::collections::VecDeque;

use bevy::{prelude::*, render::camera::ScalingMode};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::WebSocketError;
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use rand::Rng;

mod components;
use components::*;
mod resources;
use resources::*;
mod input;
use input::*;

pub const WORLD_BOUNDS: f32 = 300.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientMsg {
    pub input: InputVec2,
}

impl ClientMsg {
    pub fn new(input: InputVec2) -> Self {
        Self { input }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub local_pos: f32,
    pub other_pos: Vec<f32>,
    pub dots: Vec<Vec3>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputVec2 {
    pub x: f32,
    pub y: f32,
}

impl InputVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

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
            handle_server.in_schedule(CoreSchedule::FixedUpdate),
            move_enemies.in_schedule(CoreSchedule::FixedUpdate),
            move_system.in_schedule(CoreSchedule::FixedUpdate),
            //temp_move_system.in_schedule(CoreSchedule::FixedUpdate),
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

fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut particle_pool: ResMut<ParticlePool>,
    mut enemies_pool: ResMut<EnemiesPool>,
) {
    clear_color.0 = Color::BLACK;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 0.5)),
                color: Color::ORANGE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -50., 0.1)),
            ..Default::default()
        })
        .insert(Player { moving: false })
        .insert(Target::default())
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 25., 0.)),
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    for _ in 0..1000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    color: Color::rgb(
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..1.0),
                    ),
                    ..default()
                },

                ..Default::default()
            })
            .insert(Particle())
            .insert(Visibility::Hidden)
            .id();
        particle_pool.0.push_back(particle);
    }

    for _ in 0..100 {
        let enemies = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    color: Color::GRAY,
                    ..default()
                },
                ..Default::default()
            })
            .insert(Enemies())
            .insert(Visibility::Hidden)
            .id();
        enemies_pool.0.push_back(enemies);
    }
}

fn websocket(mut server: ResMut<Server>) {
    let ws = WebSocket::open("ws://localhost:3030/play").unwrap();
    let (mut write, mut read) = ws.split();

    let (send_tx, mut send_rx) = futures::channel::mpsc::channel::<ClientMsg>(1000);
    let (mut read_tx, read_rx) = futures::channel::mpsc::channel::<GameState>(1000);

    server.write = Some(send_tx);
    server.read = Some(read_rx);

    spawn_local(async move {
        while let Some(message) = send_rx.next().await {
            match serde_json::to_string::<ClientMsg>(&message) {
                Ok(new_input) => {
                    write.send(Message::Text(new_input)).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to parse message as Vec2: {:?}", e);
                }
            }
        }
    });

    spawn_local(async move {
        while let Some(result) = read.next().await {
            match result {
                Ok(Message::Text(msg)) => match serde_json::from_str::<GameState>(&msg) {
                    Ok(new_player_vec) => match read_tx.try_send(new_player_vec) {
                        Ok(()) => {}
                        Err(e) => eprintln!("Error sending message: {} CHANNEL FULL???", e),
                    },
                    Err(e) => {
                        eprintln!("Failed to parse message: {:?}", e);
                    }
                },
                Ok(Message::Bytes(_)) => {}

                Err(e) => match e {
                    WebSocketError::ConnectionError => {}
                    WebSocketError::ConnectionClose(_) => {
                        //
                    }
                    WebSocketError::MessageSendError(_) => {}
                    _ => {}
                },
            }
        }
    });
}

fn handle_server(
    mut server: ResMut<Server>,
    mut local_player: ResMut<LocalPlayerPos>,
    mut enemies: ResMut<EnemiesPos>,
    mut dots: ResMut<DotPos>,
) {
    if let Some(ref mut receive_rx) = server.read {
        while let Ok(message) = receive_rx.try_next() {
            if let Some(server_msg) = message {
                enemies.0 = server_msg.other_pos;
                dots.0 = server_msg.dots;
                local_player.0 = server_msg.local_pos;
            }
        }
    }
}

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    dots: ResMut<DotPos>,
) {
    for dot in dots.0.iter() {
        if let Some(pool) = particle_pool.0.pop_front() {
            match particles.get_mut(pool) {
                Ok((_particle, mut visibility, mut transform)) => {
                    transform.translation = *dot;
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
            particle_pool.0.push_back(pool);
        }
    }
}

pub fn move_enemies(
    mut enemies_pool: ResMut<EnemiesPool>,
    mut enemies: Query<(&mut Enemies, &mut Visibility, &mut Transform)>,
    enemies_pos: ResMut<EnemiesPos>,
) {
    let mut pool_iter = enemies_pool.0.iter_mut();

    for enemy in enemies_pos.0.iter() {
        if let Some(pool) = pool_iter.next() {
            match enemies.get_mut(*pool) {
                Ok((_enemies, mut visibility, mut transform)) => {
                    transform.translation = Vec3::new(*enemy, -50., 0.1);
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
        }
    }

    for pool in pool_iter {
        if let Ok((_particle, mut visibility, _transform)) = enemies.get_mut(*pool) {
            *visibility = Visibility::Hidden;
        }
    }
}
