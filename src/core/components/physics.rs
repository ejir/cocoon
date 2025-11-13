//! Physics-related components

use bevy::prelude::*;

#[derive(Component)]
pub struct Bomb {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Explosion {
    pub position: Vec2,
    pub radius: f32,
    pub force: f32,
}

#[derive(Component)]
pub struct ShockwaveRing {
    pub origin: Vec2,
    pub current_radius: f32,
    pub max_radius: f32,
    pub peak_pressure: f32,
    pub wave_speed: f32,
    pub wave_thickness: f32,
    pub lifetime: Timer,
}
