use std::collections::VecDeque;

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
    use log::Level;
    console_log::init_with_level(Level::Info).expect("error initializing log");

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
            move_system.in_schedule(CoreSchedule::FixedUpdate),
            move_dot.in_schedule(CoreSchedule::FixedUpdate),
            internal_server.in_schedule(CoreSchedule::FixedUpdate),
            out_server
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(internal_server),
        ))
        .insert_resource(DotPos(Vec::new()))
        .insert_resource(PlayerPos(Vec3::new(0., -50., 0.1)))
        .insert_resource(ParticlePool(VecDeque::new()))
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

    for _ in 0..1000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),

                    ..default()
                },

                ..Default::default()
            })
            .insert(Particle())
            .insert(Visibility::Hidden)
            .id();
        particle_pool.0.push_back(particle);
    }
}

fn internal_server(mut dots: ResMut<DotPos>) {
    let mut rng = rand::thread_rng();
    let num_balls: i32 = rng.gen_range(1..10);

    for _ in 0..num_balls {
        let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);
        let y_position: f32 = 25.;

        let dot_start = Vec3::new(x_position, y_position, 0.1);

        dots.0.push(Dot(dot_start));
    }
}

fn out_server(mut dots: ResMut<DotPos>, pp: ResMut<PlayerPos>) {
    for dot in dots.0.iter_mut() {
        dot.0.x += FALL_SPEED * 0.0;
        dot.0.y += FALL_SPEED * -1.0;
    }

    let threshold_distance: f32 = 1.0;
    dots.0.retain(|dot| {
        let distance_to_player = (dot.0 - pp.0).length();
        dot.0.y >= -WORLD_BOUNDS
            && dot.0.y <= WORLD_BOUNDS
            && dot.0.x >= -WORLD_BOUNDS
            && dot.0.x <= WORLD_BOUNDS
            && distance_to_player > threshold_distance
    });
}

// pub fn move_dot(
//     mut particle_pool: ResMut<ParticlePool>,
//     mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
//     dots: ResMut<DotPos>,
// ) {
//     let mut pool_iter = particle_pool.0.iter_mut();

//     for dot in dots.0.iter() {
//         if let Some(pool) = pool_iter.next() {
//             match particles.get_mut(*pool) {
//                 Ok((_particle, mut visibility, mut transform)) => {
//                     *visibility = Visibility::Visible;
//                     transform.translation = dot.0;
//                 }
//                 Err(err) => {
//                     info!("Error: {:?}", err);
//                 }
//             }
//         }
//     }

//     for pool in pool_iter {
//         if let Ok((_particle, mut visibility, _transform)) = particles.get_mut(*pool) {
//             *visibility = Visibility::Hidden;
//         }
//     }
// }

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    dots: ResMut<DotPos>,
    // camera_query: Query<(&Camera, &GlobalTransform)>,
    // windows: Query<&Window>,
) {
    // let (_camera, camera_transform) = camera_query.single();
    // let window = windows.single(); // Assuming there's a primary window

    // let scale_factor = window.scale_factor() as f32; // Convert scale_factor to f32
    // let win_width = window.width() / scale_factor;

    // let camera_position = camera_transform.translation().x;

    // // Calculate the range of allowed x positions for dot spawning
    // let min_x = camera_position - (win_width / 2.0);
    // let max_x = camera_position + (win_width / 2.0);

    // info!("Window width: {}", win_width);
    // info!("Camera position: {}", camera_position);
    // info!("Allowed x range: {} - {}", min_x, max_x);

    // if dots.0.len() < 1000 {
    //     return;
    // }

    for dot in dots.0.iter() {
        if let Some(pool) = particle_pool.0.pop_front() {
            // Only spawn the dot if its x position is within the allowed range
            // if dot.0.x >= min_x && dot.0.x <= max_x {
            match particles.get_mut(pool) {
                Ok((_particle, mut visibility, mut transform)) => {
                    transform.translation = dot.0;
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
            particle_pool.0.push_back(pool);
        }
        // }
    }
}
