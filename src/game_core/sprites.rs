use bevy::{prelude::*, utils::Instant};

use bevy_ecs_ldtk::LdtkWorldBundle;
use uuid::Uuid;

use crate::{
    game_util::{
        components::{Bolt, NamePlates, NamePlatesLocal, Rain},
        resources::{BoltPool, RainPool},
    },
    GameStage,
};

use super::player::{Enemy, Player};

const FONT_SIZE: f32 = 15.0;

const PLAYER_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const DOTS_SIZE: Vec2 = Vec2::new(10., 10.);
const LN_SIZE: Vec2 = Vec2::new(10., 10.);

pub fn spawn_player(
    commands: &mut Commands,
    id: &Uuid,
    asset_server: &Res<AssetServer>,
    next_state: &mut ResMut<NextState<GameStage>>,
) {
    let text = Text::from_sections([TextSection::new(
        String::new(),
        TextStyle {
            font_size: FONT_SIZE,
            color: Color::LIME_GREEN,
            ..Default::default()
        },
    )]);

    let player_image = asset_server.load("umbrella.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: player_image,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
            ..Default::default()
        })
        .insert(Player {
            id: *id,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
            adjust_iter: 0,
            name: String::new(),
            spawn_time: None,
            death_time: None,
        })
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 1.0)),
                projection: OrthographicProjection {
                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn(Text2dBundle {
                    text: text.with_alignment(TextAlignment::Center),
                    transform: Transform {
                        translation: Vec3::new(0.0, -30., 1.0),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(NamePlatesLocal);
        });

    next_state.set(GameStage::Menu);
}

pub fn spawn_enemies(
    commands: &mut Commands,
    player_id: &Uuid,
    player_pos: Option<[f32; 2]>,
    target: Option<[f32; 2]>,
    score: usize,
    enemy_name: Option<String>,
    asset_server: &Res<AssetServer>,
) {
    let target = target.unwrap_or([0.0, 0.0]);
    let player_pos = player_pos.unwrap_or([0.0, 0.0]);

    if let Some(enemy_name) = enemy_name {
        let text = Text::from_sections([TextSection::new(
            format!("{}:", enemy_name),
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..Default::default()
            },
        )]);

        let player_image = asset_server.load("umbrella.png");

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..default()
                },
                texture: player_image,
                transform: Transform::from_translation(Vec3::new(
                    player_pos[0],
                    player_pos[1],
                    0.0,
                )),
                ..Default::default()
            })
            .insert(Enemy {
                id: *player_id,
                target: Vec2 {
                    x: target[0],
                    y: target[1],
                },
                score,
                name: enemy_name,
                spawn_time: Instant::now(),
            })
            .with_children(|parent| {
                parent
                    .spawn(Text2dBundle {
                        text: text.with_alignment(TextAlignment::Center),
                        transform: Transform {
                            translation: Vec3::new(0.0, -30., 0.0),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(NamePlates);
            });
    }
}

pub fn pool_rain(
    mut commands: Commands,
    mut rain_pool: ResMut<RainPool>,
    asset_server: Res<AssetServer>,
) {
    let rain_image = asset_server.load("droplet.png");

    for _ in 0..1000 {
        let rain = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(DOTS_SIZE),
                    ..Default::default()
                },
                texture: rain_image.clone(),
                ..Default::default()
            })
            .insert(Rain)
            .insert(Visibility::Hidden)
            .id();
        rain_pool.0.push_back(rain);
    }
}

pub fn pool_bolt(
    mut commands: Commands,
    mut bolt_pool: ResMut<BoltPool>,
    asset_server: Res<AssetServer>,
) {
    let bolt_image = asset_server.load("high-voltage.png");

    for _ in 0..200 {
        let ln = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(LN_SIZE),
                    ..Default::default()
                },
                texture: bolt_image.clone(),
                ..Default::default()
            })
            .insert(Bolt)
            .insert(Visibility::Hidden)
            .id();
        bolt_pool.0.push_back(ln);
    }
}

pub fn spawn_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        ..Default::default()
    });
}
