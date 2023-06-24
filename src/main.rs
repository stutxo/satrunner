use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};
use rand::Rng;

const WORLD_BOUNDS: f32 = 100.0;
const PLAYER_SPEED: f32 = 1.0;
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
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    // let border = 0.05;

    // Left border
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::WHITE,
    //         custom_size: Some(Vec2::new(border, WORLD_BOUNDS * 2.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(-WORLD_BOUNDS - border / 2.0, 0.0, 0.0)),
    //     ..default()
    // });

    // // Right border
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::WHITE,
    //         custom_size: Some(Vec2::new(border, WORLD_BOUNDS * 2.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(WORLD_BOUNDS + border / 2.0, 0.0, 0.0)),
    //     ..default()
    // });
}

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Target, &mut Player)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut windows: Query<&mut Window>,
    touches: Res<Touches>,
) {
    for (mut t, mut tg, mut p) in query.iter_mut() {
        if mouse.pressed(MouseButton::Left) || mouse.pressed(MouseButton::Right) {
            for window in windows.iter_mut() {
                if let Some(cursor) = window.cursor_position() {
                    let (camera, camera_transform) = camera_query.single();
                    let click_position =
                        get_click_position(&window, camera, camera_transform, cursor);
                    tg.x = click_position.x;
                    tg.y = click_position.y;
                    p.moving = true;
                }
            }
        }

        for touch in touches.iter() {
            let touch_pos = touch.position();
            let (camera, camera_transform) = camera_query.single();

            for window in windows.iter_mut() {
                let touch_position =
                    get_touch_position(&window, camera, camera_transform, touch_pos);
                tg.x = touch_position.x;
                tg.y = touch_position.y;
                p.moving = true;
            }
        }

        if p.moving {
            let current_position = Vec2::new(t.translation.x, t.translation.y);
            let direction = Vec2::new(tg.x, tg.y) - current_position;
            let distance_to_target = direction.length();

            if distance_to_target > 0.0 {
                let normalized_direction = direction / distance_to_target;
                let movement = normalized_direction * PLAYER_SPEED;

                let new_position = current_position + movement;

                if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                    if movement.length() < distance_to_target {
                        t.translation += Vec3::new(movement.x, 0.0, 0.0);
                    } else {
                        t.translation = Vec3::new(tg.x, -50.0, 0.1);
                        p.moving = false;
                    }
                } else {
                    p.moving = false;
                }
            } else {
                p.moving = false;
            }
        }
    }
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

pub fn get_click_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Vec2 {
    let screen_size = Vec2::new(window.width(), window.height());
    let screen_position = cursor_position / screen_size;
    let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
    let mut click_position = camera
        .projection_matrix()
        .inverse()
        .project_point3(clip_position.extend(0.0));
    click_position = *camera_transform * click_position;
    click_position.truncate()
}

pub fn get_touch_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Vec2 {
    let screen_size = Vec2::new(window.width(), window.height());
    let screen_position = Vec2::new(
        cursor_position.x / screen_size.x,
        1.0 - (cursor_position.y / screen_size.y),
    );
    let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
    let mut touch_position = camera
        .projection_matrix()
        .inverse()
        .project_point3(clip_position.extend(0.0));
    touch_position = *camera_transform * touch_position;
    touch_position.truncate()
}

#[derive(Component)]
pub struct Player {
    pub moving: bool,
}

#[derive(Default, Reflect, Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct FallingDot();

#[derive(Resource)]
struct DotPos {
    dots: Vec<Dot>,
}

struct Dot {
    pos: Vec3,
    direction: Vec2,
}
