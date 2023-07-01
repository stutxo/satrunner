use bevy::{prelude::*, utils::Instant};

use crate::{
    game_util::components::{Player, Target},
    game_util::{components::LocalPlayer, resources::Server},
    network::messages::{ClientMsg, InputVec2},
};

pub fn input(
    mut query: Query<(&mut Target, &mut Player, With<LocalPlayer>)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut server: ResMut<Server>,
) {
    for (mut tg, mut p, _local) in query.iter_mut() {
        if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            for window in windows.iter() {
                if let Some(cursor) = window.cursor_position() {
                    let (camera, camera_transform) = camera_query.single();
                    let click_position =
                        get_click_position(window, camera, camera_transform, cursor);
                    tg.x = click_position.x;
                    tg.y = click_position.y;
                    tg.index += 1;
                    tg.last_input_time = Instant::now();
                    p.moving = true;

                    let input = ClientMsg::new(
                        InputVec2::new(click_position.x, click_position.y),
                        tg.index,
                    );

                    match server.write.as_mut().unwrap().try_send(input) {
                        Ok(()) => {}
                        Err(e) => info!("Error sending message: {} CHANNEL FULL???", e),
                    };
                }
            }
        }

        for touch in touches.iter_just_pressed() {
            let touch_pos = touch.position();
            let (camera, camera_transform) = camera_query.single();

            for window in windows.iter() {
                let touch_position =
                    get_touch_position(window, camera, camera_transform, touch_pos);
                tg.x = touch_position.x;
                tg.y = touch_position.y;
                tg.index += 1;
                tg.last_input_time = Instant::now();
                p.moving = true;

                let input =
                    ClientMsg::new(InputVec2::new(touch_position.x, touch_position.y), tg.index);

                match server.write.as_mut().unwrap().try_send(input) {
                    Ok(()) => {}
                    Err(e) => info!("Error sending message: {} CHANNEL FULL???", e),
                };
            }
        }
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
