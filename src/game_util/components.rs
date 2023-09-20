use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct Rain;
#[derive(Component)]
pub struct Bolt;

#[derive(Component)]
pub struct Badge;

#[derive(Component)]
pub struct NamePlates {
    pub id: Uuid,
}

#[derive(Component)]
pub struct NamePlatesLocal;
