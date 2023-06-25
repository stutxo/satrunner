use bevy::prelude::*;

use crate::{
    components::{Player, Target},
    PlayerPos,
};

const WORLD_BOUNDS: f32 = 500.0;
const PLAYER_SPEED: f32 = 1.0;

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

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Target, &mut Player)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut windows: Query<&mut Window>,
    touches: Res<Touches>,
    mut pp: ResMut<PlayerPos>,
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
                        pp.pp += Vec3::new(movement.x, 0.0, 0.0);
                    } else {
                        t.translation = Vec3::new(tg.x, -50.0, 0.1);
                        pp.pp = Vec3::new(tg.x, -50.0, 0.1);
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
