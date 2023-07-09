use bevy::prelude::*;

use crate::{
    game_util::{components::LocalPlayer, resources::NetworkStuff},
    network::messages::PlayerInput,
};

use super::player::Player;

pub fn input(
    mut query: Query<(&mut Player, &mut Transform, With<LocalPlayer>)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    touches: Res<Touches>,
    mut outgoing: ResMut<NetworkStuff>,
) {
    for (mut player, mut t, _local) in query.iter_mut() {
        //always set local player above other players
        t.translation.z = 0.1;
        player.client_tick += 1;
        info!("client tick: {}", player.client_tick);

        let (camera, camera_transform) = camera_query.single();

        let get_position = |cursor_position: Vec2, window: &Window, is_touch: bool| {
            let screen_size = Vec2::new(window.width(), window.height());
            let screen_position = if is_touch {
                Vec2::new(
                    cursor_position.x / screen_size.x,
                    1.0 - (cursor_position.y / screen_size.y),
                )
            } else {
                cursor_position / screen_size
            };
            let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
            let mut position = camera
                .projection_matrix()
                .inverse()
                .project_point3(clip_position.extend(0.0));
            position = *camera_transform * position;
            position.truncate()
        };

        let mut handle_input = |cursor_position: Vec2, player: &mut Player| {
            player.target = cursor_position;

            let input = PlayerInput::new(player.target, player.id, player.client_tick);

            player.pending_inputs.push(input.clone());

            match outgoing.write.as_mut().unwrap().try_send(input) {
                Ok(()) => {}
                Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
            };

            player.apply_input(&mut t);
        };

        if mouse.pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
            if let Some(window) = windows.iter().next() {
                if let Some(cursor) = window.cursor_position() {
                    let position = get_position(cursor, window, false);
                    handle_input(position, &mut player);
                }
            }
        }

        for touch in touches.iter() {
            if let Some(window) = windows.iter().next() {
                let position = get_position(touch.position(), window, true);
                handle_input(position, &mut player);
            }
        }
    }
}
