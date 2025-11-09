use bevy::prelude::*;
use rand::Rng;

use crate::utils::{fade_sprite_alpha, set_sprite_alpha};

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub gravity: f32,
    pub drag: f32,
    pub fade_mode: FadeMode,
    pub scale_mode: ScaleMode,
}
#[derive(Component, Clone)]
pub enum FadeMode {
    Linear,
    Constant(f32),
}
#[derive(Component, Clone)]
pub enum ScaleMode {
    None,
    GrowLinear(f32),
    ShrinkLinear(f32),
}

pub fn animate_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        transform.translation += particle.velocity.extend(0.0) * time.delta_secs();

        particle.velocity.y += particle.gravity * time.delta_secs();
        let drag = particle.drag;
        particle.velocity *= drag;

        let progress = particle.lifetime.fraction();

        match particle.scale_mode {
            ScaleMode::None => {}
            ScaleMode::GrowLinear(max_scale) => {
                let scale = 1.0 + progress * (max_scale - 1.0);
                transform.scale = Vec3::splat(scale);
            }
            ScaleMode::ShrinkLinear(min_scale) => {
                let scale = 1.0 - progress * (1.0 - min_scale);
                transform.scale = Vec3::splat(scale);
            }
        }

        match particle.fade_mode {
            FadeMode::Linear => {
                let alpha = 1.0 - progress;
                set_sprite_alpha(&mut sprite, alpha);
            }
            FadeMode::Constant(alpha_multiplier) => {
                let base_alpha = 1.0 - progress;
                set_sprite_alpha(&mut sprite, base_alpha * alpha_multiplier);
            }
        }

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct ParticleSpawnConfig {
    pub count: usize,
    pub position: Vec2,
    pub z_index: f32,
    pub size_range: (f32, f32),
    pub speed_range: (f32, f32),
    pub lifetime_range: (f32, f32),
    pub color_fn: Box<dyn Fn(&mut rand::rngs::ThreadRng) -> Color>,
    pub velocity_fn: Box<dyn Fn(&mut rand::rngs::ThreadRng, f32) -> Vec2>,
    pub gravity: f32,
    pub drag: f32,
    pub fade_mode: FadeMode,
    pub scale_mode: ScaleMode,
}

impl Default for ParticleSpawnConfig {
    fn default() -> Self {
        Self {
            count: 10,
            position: Vec2::ZERO,
            z_index: 0.0,
            size_range: (3.0, 8.0),
            speed_range: (50.0, 150.0),
            lifetime_range: (1.0, 2.0),
            color_fn: Box::new(|_| Color::WHITE),
            velocity_fn: Box::new(|rng, speed| {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                Vec2::new(angle.cos(), angle.sin()) * speed
            }),
            gravity: -400.0,
            drag: 0.98,
            fade_mode: FadeMode::Linear,
            scale_mode: ScaleMode::None,
        }
    }
}

pub fn spawn_particles(commands: &mut Commands, config: ParticleSpawnConfig) {
    let mut rng = rand::thread_rng();

    for _ in 0..config.count {
        let size = rng.gen_range(config.size_range.0..config.size_range.1);
        let speed = rng.gen_range(config.speed_range.0..config.speed_range.1);
        let lifetime = rng.gen_range(config.lifetime_range.0..config.lifetime_range.1);

        let color = (config.color_fn)(&mut rng);
        let velocity = (config.velocity_fn)(&mut rng, speed);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(config.position.x, config.position.y, config.z_index),
            Particle {
                lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
                velocity,
                gravity: config.gravity,
                drag: config.drag,
                fade_mode: config.fade_mode.clone(),
                scale_mode: config.scale_mode.clone(),
            },
        ));
    }
}
