//! Visual effects components

use bevy::prelude::*;

#[derive(Component)]
pub struct ExplosionFlash {
    pub timer: Timer,
    pub max_scale: f32,
}

#[derive(Component)]
pub struct ExplosionShockwave {
    pub timer: Timer,
    pub max_radius: f32,
    pub start_radius: f32,
}

#[derive(Component)]
pub struct SmokeParticle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct BloodParticle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct OnFire {
    pub intensity: f32,
    pub duration: Timer,
}

#[derive(Component)]
pub struct FireParticle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}
