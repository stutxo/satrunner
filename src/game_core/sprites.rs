use bevy::prelude::*;
use rand::Rng;
use uuid::Uuid;

use crate::game_util::{
    components::{NamePlates, NamePlatesLocal, Particle},
    resources::ParticlePool,
};

use super::player::{Enemy, Player};

const FONT_SIZE: f32 = 15.0;

const PLAYER_SIZE: Vec2 = Vec2::new(3., 3.0);
const DOTS_SIZE: Vec2 = Vec2::new(3., 3.);

pub fn spawn_player(commands: &mut Commands, id: &Uuid) {
    let text = Text::from_sections([
        TextSection::new(
            String::new(),
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::LIME_GREEN,
                ..Default::default()
            },
        ),
        TextSection::new(
            "0",
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::LIME_GREEN,
                ..Default::default()
            },
        ),
    ]);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                color: Color::LIME_GREEN,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -150., 0.1)),
            ..Default::default()
        })
        .insert(Player {
            id: *id,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
            adjust_iter: 0,
            name: String::new(),
        })
        .insert(Visibility::Hidden)
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 150., 0.)),
                projection: OrthographicProjection {
                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn(Text2dBundle {
                    text: text.with_alignment(TextAlignment::Center),
                    transform: Transform::from_translation(Vec3::new(0.0, 15., 0.0)),
                    ..Default::default()
                })
                .insert(NamePlatesLocal);
        });
}

pub fn pool_dots(mut commands: Commands, mut particle_pool: ResMut<ParticlePool>) {
    for _ in 0..1000 {
        let particle = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(DOTS_SIZE),
                    color: Color::rgb(
                        rand::thread_rng().gen_range(0.0..1.0),
                        rand::thread_rng().gen_range(0.0..2.0),
                        rand::thread_rng().gen_range(0.0..3.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Particle)
            .insert(Visibility::Hidden)
            .id();
        particle_pool.0.push_back(particle);
    }
}

pub fn spawn_enemies(
    commands: &mut Commands,
    player_id: &Uuid,
    player_pos: Option<f32>,
    target: Option<[f32; 2]>,
    score: usize,
    enemy_name: Option<String>,
) {
    let target = target.unwrap_or([0.0, 0.0]);
    let player_pos = player_pos.unwrap_or(0.0);

    if let Some(enemy_name) = enemy_name {
        let text = Text::from_sections([
            TextSection::new(
                format!("{}:", enemy_name),
                TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::GRAY,
                    ..Default::default()
                },
            ),
            TextSection::new(
                format!("{}", score),
                TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::GRAY,
                    ..Default::default()
                },
            ),
        ]);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    color: Color::RED,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(player_pos, -150., 0.0)),
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
            })
            .with_children(|parent| {
                parent
                    .spawn(Text2dBundle {
                        text: text.with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(Vec3::new(0.0, 15., 0.0)),
                        ..Default::default()
                    })
                    .insert(NamePlates);
            });
    }
}
