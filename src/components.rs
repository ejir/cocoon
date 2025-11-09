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
pub struct RagdollPart;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Debris;

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
pub struct Flammable {
    pub ignition_threshold: f32,
}

#[derive(Component)]
pub struct FireParticle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}
