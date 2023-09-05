use bevy::prelude::*;
use virtual_joystick::{
    TintColor, VirtualJoystickEvent, VirtualJoystickEventType, VirtualJoystickNode,
};

use crate::{
    game_util::resources::{ClientTick, NetworkStuff},
    network::messages::{ClientMessage, PlayerInput},
};

use super::player::Player;
use std::f32::consts::PI;

pub fn input(
    mut query: Query<&mut Player>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
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
                    client_tick.tick.unwrap(),
                    true,
                );

                player.pending_inputs.push(input.clone());

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
        }
    }
}

pub fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent<String>>,
    mut joystick_color: Query<(&mut TintColor, &VirtualJoystickNode<String>)>,
    mut query: Query<(&Transform, &mut Player)>,
    mut outgoing: ResMut<NetworkStuff>,
    client_tick: Res<ClientTick>,
) {
    if client_tick.pause == 0 {
        for j in joystick.iter() {
            match j.get_type() {
                VirtualJoystickEventType::Press | VirtualJoystickEventType::Drag => {
                    let (mut color, node) = joystick_color.single_mut();
                    if node.id == j.id() {
                        *color = TintColor(Color::WHITE);
                    }
                }
                VirtualJoystickEventType::Up => {
                    let (mut color, node) = joystick_color.single_mut();
                    if node.id == j.id() {
                        *color = TintColor(Color::WHITE.with_a(0.2));
                    }
                }
            }

            for (t, mut player) in query.iter_mut() {
                let axis = j.axis().normalize();

                let mut should_send = false;

                let mut current_direction = String::from("");
                let angle = axis.y.atan2(axis.x);
                let degree = angle * 180.0 / PI;
                let tolerance = 30.0;

                if degree >= -tolerance && degree <= tolerance {
                    current_direction = "Right".to_string();
                } else if degree >= 90.0 - tolerance && degree <= 90.0 + tolerance {
                    current_direction = "Up".to_string();
                } else if degree >= -90.0 - tolerance && degree <= -90.0 + tolerance {
                    current_direction = "Down".to_string();
                } else if degree >= 180.0 - tolerance || degree <= -180.0 + tolerance {
                    current_direction = "Left".to_string();
                }

                if player.last_direction.is_none()
                    || player.last_direction.as_ref().unwrap() != &current_direction
                {
                    should_send = true;
                    player.last_direction = Some(current_direction.clone());
                }

                if should_send {
                    match current_direction.as_str() {
                        "Up" => player.target = Vec2::new(0.0, 1.0) * 1000.,
                        "Down" => player.target = Vec2::new(0.0, -1.0) * 1000.,
                        "Right" => player.target = Vec2::new(1.0, 0.0) * 1000.,
                        "Left" => player.target = Vec2::new(-1.0, 0.0) * 1000.,
                        _ => player.target = Vec2::new(t.translation.x, t.translation.y),
                    };

                    let input = PlayerInput::new(
                        [player.target.x, player.target.y],
                        player.id,
                        client_tick.tick.unwrap(),
                        true,
                    );

                    player.pending_inputs.push(input.clone());

                    match outgoing
                        .write
                        .as_mut()
                        .unwrap()
                        .try_send(ClientMessage::PlayerInput(input))
                    {
                        Ok(()) => {}
                        Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                    };
                }
            }
        }
    }
}
