//! Entity-related components

use bevy::prelude::*;

#[derive(Component)]
pub struct RagdollPart;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Debris;

#[derive(Component)]
pub struct Flammable {
    pub ignition_threshold: f32,
}
