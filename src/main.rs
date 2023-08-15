use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_egui::EguiPlugin;
use game_core::{
    game_loop::{enemy_loop, player_loop, tick},
    gui::{check_disconnected, disconnected, game_over, score_board, setup_menu},
    handle::handle_server,
    input::input,
    objects::{handle_bolt, handle_rain},
    sprites::{pool_bolt, pool_rain, spawn_ldtk},
};

use game_util::resources::{
    BoltPool, ClientTick, NetworkStuff, Objects, PingTimer, PlayerName, RainPool,
};
use network::websockets::websocket;
use std::collections::VecDeque;

mod game_core;
mod game_util;
mod network;

pub const TICK_RATE: f32 = 1. / 10.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rain.run".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            EguiPlugin,
            LdtkPlugin,
        ))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            level_background: LevelBackground::Nonexistent,
            ..Default::default()
        })
        .register_ldtk_entity::<MyBundle>("background")
        .add_state::<GameStage>()
        .add_systems(Startup, (spawn_ldtk, websocket, pool_rain, pool_bolt))
        .add_systems(Update, setup_menu.run_if(in_state(GameStage::Menu)))
        .add_systems(Update, (handle_server, score_board, check_disconnected))
        .add_systems(FixedUpdate, (tick, enemy_loop, handle_rain, handle_bolt))
        .add_systems(Update, (input).run_if(in_state(GameStage::InGame)))
        .add_systems(
            Update,
            (disconnected).run_if(in_state(GameStage::Disconnected)),
        )
        .add_systems(Update, (game_over).run_if(in_state(GameStage::GameOver)))
        .add_systems(
            FixedUpdate,
            (player_loop).run_if(in_state(GameStage::InGame)),
        )
        .insert_resource(FixedTime::new_from_secs(TICK_RATE))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Objects::new())
        .insert_resource(RainPool(VecDeque::new()))
        .insert_resource(BoltPool(VecDeque::new()))
        .insert_resource(NetworkStuff::new())
        .insert_resource(ClientTick::new())
        .insert_resource(PlayerName::new())
        .insert_resource(PingTimer::new())
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameStage {
    #[default]
    Menu,
    InGame,
    Disconnected,
    GameOver,
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}
