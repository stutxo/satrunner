use std::collections::VecDeque;

use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;

use components::*;
use game_engine::*;
use input::*;
use messages::*;
use networking::*;
use resources::*;

mod components;
mod game_engine;
mod input;
mod messages;
mod networking;
mod resources;

pub const WORLD_BOUNDS: f32 = 300.0;

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
