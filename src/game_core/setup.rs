use bevy::{prelude::*, render::camera::ScalingMode};
use rand::Rng;

use crate::{
    game_util::components::{Particle, Player, Target},
    game_util::{components::LocalPlayer, resources::ParticlePool},
};

pub const WORLD_BOUNDS: f32 = 300.0;
pub const PLAYER_SPEED: f32 = 1.0;

// pub fn setup(
//     mut commands: Commands,
//     mut particle_pool: ResMut<ParticlePool>,
//     player_id: Res<PlayerInit>,
// ) {

//     // for _ in 0..1000 {
//     //     let particle = commands
//     //         .spawn(SpriteBundle {
//     //             sprite: Sprite {
//     //                 custom_size: Some(Vec2::new(0.5, 0.5)),
//     //                 color: Color::rgb(
//     //                     rand::thread_rng().gen_range(0.0..1.0),
//     //                     rand::thread_rng().gen_range(0.0..1.0),
//     //                     rand::thread_rng().gen_range(0.0..1.0),
//     //                 ),
//     //                 ..default()
//     //             },

//     //             ..Default::default()
//     //         })
//     //         .insert(Particle)
//     //         .insert(Visibility::Hidden)
//     //         .id();
//     //     particle_pool.0.push_back(particle);
//     // }
// }
