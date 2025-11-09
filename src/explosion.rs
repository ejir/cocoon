use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{Debris, ExplosionFlash, ExplosionShockwave, SmokeParticle};
use crate::constants::EXPLOSION_RADIUS;

pub fn spawn_explosion_visuals(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.9, 0.3, 1.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.0),
        ExplosionFlash {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            max_scale: 15.0,
        },
    ));

    for i in 0..3 {
        let delay = i as f32 * 0.05;
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.6, 0.1, 0.8),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.5),
            ExplosionShockwave {
                timer: Timer::from_seconds(0.5 + delay, TimerMode::Once),
                max_radius: EXPLOSION_RADIUS * 1.2,
                start_radius: 20.0,
            },
        ));
    }
}

pub fn spawn_explosion_debris(commands: &mut Commands, position: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..30 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(150.0..500.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(3.0..12.0);
        let color = Color::srgb(
            rng.gen_range(0.8..1.0),
            rng.gen_range(0.3..0.7),
            rng.gen_range(0.0..0.2),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            RigidBody::Dynamic,
            Collider::ball(size / 2.0),
            Velocity::linear(velocity),
            ColliderMassProperties::Density(0.5),
            ExternalImpulse::default(),
            Debris,
        ));
    }

    for _ in 0..20 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..200.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(2.0..6.0);
        let color = Color::srgba(1.0, 0.8, 0.0, 1.0);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            RigidBody::Dynamic,
            Collider::ball(size / 2.0),
            Velocity::linear(velocity),
            ColliderMassProperties::Density(0.3),
            ExternalImpulse::default(),
            Debris,
        ));
    }
}

pub fn spawn_smoke_particles(commands: &mut Commands, position: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..40 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(30.0..120.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(8.0..20.0);
        let gray = rng.gen_range(0.2..0.5);
        let color = Color::srgba(gray, gray, gray, rng.gen_range(0.4..0.8));

        let offset = Vec2::new(
            rng.gen_range(-20.0..20.0),
            rng.gen_range(-20.0..20.0),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x + offset.x, position.y + offset.y, -0.5),
            SmokeParticle {
                lifetime: Timer::from_seconds(rng.gen_range(1.0..2.5), TimerMode::Once),
                velocity: velocity + Vec2::new(0.0, rng.gen_range(30.0..80.0)),
            },
        ));
    }
}
