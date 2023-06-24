use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};
use rand::Rng;

mod components;
use components::*;
mod input;
use input::*;

const WORLD_BOUNDS: f32 = 100.0;
const FALL_SPEED: f32 = 0.5;

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
        .add_startup_system(setup)
        .add_systems((
            move_system.in_schedule(CoreSchedule::FixedUpdate),
            internal_server.in_schedule(CoreSchedule::FixedUpdate),
            out_server
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(internal_server),
            spawn_dots
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(out_server),
            despawn
                .in_schedule(CoreSchedule::FixedUpdate)
                .after(spawn_dots),
        ))
        .insert_resource(DotPos { dots: Vec::new() })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut clear_color: ResMut<ClearColor>,
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
}

fn internal_server(mut dots: ResMut<DotPos>) {
    let mut rng = rand::thread_rng();
    let num_balls: i32 = rng.gen_range(1..4);

    for _ in 0..num_balls {
        let x_position: f32 = rng.gen_range(-WORLD_BOUNDS..WORLD_BOUNDS);
        let y_position = WORLD_BOUNDS;

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

fn out_server(mut dots: ResMut<DotPos>) {
    for dot in dots.dots.iter_mut() {
        dot.pos.x += FALL_SPEED * dot.direction.x;
        dot.pos.y += FALL_SPEED * dot.direction.y;
    }
    dots.dots.retain(|dot| {
        dot.pos.y >= -WORLD_BOUNDS
            && dot.pos.y <= WORLD_BOUNDS
            && dot.pos.x >= -WORLD_BOUNDS
            && dot.pos.x <= WORLD_BOUNDS
    });
}

fn spawn_dots(mut commands: Commands, dots: ResMut<DotPos>) {
    for dot in dots.dots.iter() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    ..default()
                },
                transform: Transform::from_translation(dot.pos),
                ..Default::default()
            })
            .insert(FallingDot());
    }
}

fn despawn(mut commands: Commands, mut query: Query<(Entity, &FallingDot)>) {
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
struct DotPos {
    dots: Vec<Dot>,
}

struct Dot {
    pos: Vec3,
    direction: Vec2,
}
