use bevy::prelude::*;

use crate::KeyboardState;

use self::{
    components::KeyBoard,
    layout::setup_keyboard,
    resources::CapitalizeToggle,
    systems::{physical_keyboard_system, virtual_capitalize_system, virtual_keyboard_system},
};

pub mod components;
pub mod layout;
pub mod resources;
mod styles;
mod systems;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .insert_resource(CapitalizeToggle(false))
            .add_systems(OnEnter(KeyboardState::On), setup_keyboard)
            .add_systems(
                Update,
                (
                    physical_keyboard_system,
                    virtual_keyboard_system,
                    virtual_capitalize_system,
                )
                    .run_if(in_state(KeyboardState::On)),
            )
            .add_systems(OnExit(KeyboardState::On), despawn_screen::<KeyBoard>);
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
