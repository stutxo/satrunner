use bevy::prelude::*;

use crate::{
    game_util::resources::{ClientTick, NetworkStuff},
    network::messages::{ClientMessage, PlayerInput},
};

use super::player::Player;

pub fn input(
    mut query: Query<&mut Player>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut outgoing: ResMut<NetworkStuff>,
    client_tick: Res<ClientTick>,
) {
    for mut player in query.iter_mut() {
        let (camera, camera_transform) = camera_query.single();

        let get_position = |cursor_position: Vec2, window: &Window| {
            let screen_size = Vec2::new(window.width(), window.height());
            let screen_position = Vec2::new(
                cursor_position.x / screen_size.x,
                1.0 - (cursor_position.y / screen_size.y),
            );

            let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
            let mut position = camera
                .projection_matrix()
                .inverse()
                .project_point3(clip_position.extend(0.0));
            position = *camera_transform * position;
            position.truncate()
        };

        if client_tick.pause == 0 {
            let mut handle_input = |cursor_position: Vec2, player: &mut Player| {
                player.target = cursor_position;

                let input = PlayerInput::new(
                    [player.target.x, player.target.y],
                    player.id,
                    client_tick.tick,
                );

                player.pending_inputs.push(input.clone());

                // info!(
                //     "Sending input: {:?}, player pos: {:?}",
                //     input, t.translation.x
                // );

                match outgoing
                    .write
                    .as_mut()
                    .unwrap()
                    .try_send(ClientMessage::PlayerInput(input))
                {
                    Ok(()) => {}
                    Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                };
            };

            if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
                if let Some(window) = windows.iter().next() {
                    if let Some(cursor) = window.cursor_position() {
                        let position = get_position(cursor, window);
                        handle_input(position, &mut player);
                    }
                }
            }

            for touch in touches.iter_just_pressed() {
                if let Some(window) = windows.iter().next() {
                    let position = get_position(touch.position(), window);
                    handle_input(position, &mut player);
                }
            }
        }
    }
}
