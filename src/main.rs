use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};

use rand::Rng;

mod components;
use components::*;
mod resources;
use resources::*;
mod input;
use input::*;

pub const WORLD_BOUNDS: f32 = 300.0;
const FALL_SPEED: f32 = 0.5;

fn main() {
    //use log::Level;
    //console_log::init_with_level(Level::Info).expect("error initializing log");

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
        .add_startup_system(setup)
        .add_systems((
            internal_server.in_schedule(CoreSchedule::FixedUpdate),
            out_server
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(internal_server),
            move_system.in_schedule(CoreSchedule::FixedUpdate),
            move_dot
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(out_server),
        ))
        .insert_resource(DotPos { dots: Vec::new() })
        .insert_resource(PlayerPos {
            pp: Vec3::new(0.0, -50., 0.1),
        })
        .insert_resource(ParticlePool {
            particles: Vec::new(),
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut clear_color: ResMut<ClearColor>,
    mut particle_pool: ResMut<ParticlePool>,
) {
    clear_color.0 = Color::BLACK;

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(0.3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
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

    for _ in 0..10000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    ..default()
                },
                ..Default::default()
            })
            .insert(Particle {
                position: Vec3::ZERO,
            })
            .insert(Visibility::Hidden)
            .id();
        particle_pool.particles.push(particle);
    }
}

fn internal_server(mut dots: ResMut<DotPos>) {
    let mut rng = rand::thread_rng();
    let num_balls: i32 = rng.gen_range(1..10);

    for _ in 0..num_balls {
        let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);
        let y_position: f32 = 23.;

        let dot_start = Vec3::new(x_position, y_position, 0.1);

        let direction_x: f32 = 0.0;
        let direction_y: f32 = -1.0;
        let direction = Vec2::new(direction_x, direction_y).normalize();

        dots.dots.push(Dot {
            pos: dot_start,
            direction,
        });
    }
}

fn out_server(mut dots: ResMut<DotPos>, pp: ResMut<PlayerPos>) {
    for dot in dots.dots.iter_mut() {
        dot.pos.x += FALL_SPEED * dot.direction.x;
        dot.pos.y += FALL_SPEED * dot.direction.y;
    }

    let threshold_distance: f32 = 1.0;
    dots.dots.retain(|dot| {
        let distance_to_player = (dot.pos - pp.pp).length();
        dot.pos.y >= -WORLD_BOUNDS
            && dot.pos.y <= WORLD_BOUNDS
            && dot.pos.x >= -WORLD_BOUNDS
            && dot.pos.x <= WORLD_BOUNDS
            && distance_to_player > threshold_distance
    });
}

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    dots: ResMut<DotPos>,
) {
    let mut pool_iter = particle_pool.particles.iter_mut();
    for dot in dots.dots.iter() {
        if let Some(pool) = pool_iter.next() {
            match particles.get_mut(*pool) {
                Ok((mut particle, mut visibility, mut transform)) => {
                    *visibility = Visibility::Visible;
                    particle.position = dot.pos;
                    transform.translation = dot.pos;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
        }
    }
}
