use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::{Debris, SmokeParticle};

pub fn spawn_object_fragments(
    commands: &mut Commands,
    position: Vec2,
    size: Vec2,
    color: Color,
    velocity: Vec2,
    is_wooden: bool,
) {
    let mut rng = rand::thread_rng();
    
    let fragment_count = if is_wooden { 
        rng.gen_range(8..15)
    } else {
        rng.gen_range(6..12)
    };
    
    let average_size = (size.x + size.y) / 2.0;
    
    for _ in 0..fragment_count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let spread_speed = rng.gen_range(100.0..400.0);
        let spread_velocity = Vec2::new(angle.cos(), angle.sin()) * spread_speed;
        
        let fragment_velocity = velocity * rng.gen_range(0.6..1.2) + spread_velocity;
        
        let fragment_size = average_size * rng.gen_range(0.15..0.35);
        
        let color_variation = if is_wooden {
            Color::srgb(
                (color.to_srgba().red + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
                (color.to_srgba().green + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
                (color.to_srgba().blue + rng.gen_range(-0.05..0.1)).clamp(0.0, 1.0),
            )
        } else {
            color
        };
        
        let offset = Vec2::new(
            rng.gen_range(-size.x / 2.0..size.x / 2.0),
            rng.gen_range(-size.y / 2.0..size.y / 2.0),
        );
        
        let shape_choice = rng.gen_range(0..3);
        let (collider, visual_size) = if shape_choice == 0 {
            (
                Collider::ball(fragment_size / 2.0),
                Vec2::new(fragment_size, fragment_size),
            )
        } else {
            let width = fragment_size * rng.gen_range(0.6..1.4);
            let height = fragment_size * rng.gen_range(0.6..1.4);
            (
                Collider::cuboid(width / 2.0, height / 2.0),
                Vec2::new(width, height),
            )
        };
        
        commands.spawn((
            Sprite {
                color: color_variation,
                custom_size: Some(visual_size),
                ..default()
            },
            Transform::from_xyz(position.x + offset.x, position.y + offset.y, 0.0)
                .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::TAU))),
            RigidBody::Dynamic,
            collider,
            Velocity {
                linvel: fragment_velocity,
                angvel: rng.gen_range(-10.0..10.0),
            },
            ColliderMassProperties::Density(if is_wooden { 0.7 } else { 1.0 }),
            ExternalImpulse::default(),
            Debris,
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

        let offset = Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));

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
