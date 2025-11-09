use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::blood::spawn_blood_particles;
use crate::components::{FireParticle, Flammable, Health, OnFire, RagdollPart};
use crate::constants::{FIRE_DAMAGE_PER_SEC, FIRE_DURATION, FIRE_SPAWN_KEY, FIRE_SPREAD_RADIUS};
use crate::entity_finder::find_closest_entity;
use crate::utils::{get_cursor_world_position, set_sprite_alpha};

pub fn ignite_ragdoll_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    flammable_query: Query<(Entity, &Transform, &Flammable), Without<OnFire>>,
) {
    if keyboard.just_pressed(FIRE_SPAWN_KEY) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            if let Some(entity) = find_closest_entity(flammable_query.iter(), world_pos, 100.0) {
                commands.entity(entity).insert(OnFire {
                    intensity: 1.0,
                    duration: Timer::from_seconds(FIRE_DURATION, TimerMode::Once),
                });
            }
        }
    }
}

pub fn apply_fire_damage(
    mut commands: Commands,
    time: Res<Time>,
    mut fire_query: Query<(
        Entity,
        &Transform,
        &mut OnFire,
        &mut Health,
        Option<&RagdollPart>,
    )>,
) {
    for (entity, transform, mut on_fire, mut health, ragdoll_opt) in fire_query.iter_mut() {
        on_fire.duration.tick(time.delta());

        let damage = FIRE_DAMAGE_PER_SEC * time.delta_secs() * on_fire.intensity;
        health.current -= damage;

        spawn_fire_particles(
            &mut commands,
            transform.translation.truncate(),
            on_fire.intensity,
        );

        if health.current <= 0.0 {
            if ragdoll_opt.is_some() {
                spawn_blood_particles(
                    &mut commands,
                    transform.translation.truncate(),
                    Vec2::new(0.0, 50.0),
                );
            }
            commands.entity(entity).despawn();
        } else if on_fire.duration.finished() {
            commands.entity(entity).remove::<OnFire>();
        }
    }
}

pub fn spread_fire(
    mut commands: Commands,
    fire_query: Query<(&Transform, &OnFire)>,
    flammable_query: Query<(Entity, &Transform, &Flammable), Without<OnFire>>,
) {
    for (fire_transform, on_fire) in fire_query.iter() {
        let fire_pos = fire_transform.translation.truncate();

        for (entity, transform, _flammable) in flammable_query.iter() {
            let pos = transform.translation.truncate();
            let distance = fire_pos.distance(pos);

            if distance < FIRE_SPREAD_RADIUS {
                let spread_chance =
                    (1.0 - distance / FIRE_SPREAD_RADIUS) * on_fire.intensity * 0.01;
                if rand::thread_rng().gen::<f32>() < spread_chance {
                    commands.entity(entity).insert(OnFire {
                        intensity: on_fire.intensity * 0.8,
                        duration: Timer::from_seconds(FIRE_DURATION, TimerMode::Once),
                    });
                }
            }
        }
    }
}

fn spawn_fire_particles(commands: &mut Commands, position: Vec2, intensity: f32) {
    let mut rng = rand::thread_rng();

    for _ in 0..(3.0 * intensity) as i32 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(20.0..60.0);
        let velocity = Vec2::new(angle.cos() * speed, rng.gen_range(40.0..100.0));

        let size = rng.gen_range(4.0..10.0) * intensity;
        let color_choice = rng.gen_range(0..3);
        let color = match color_choice {
            0 => Color::srgba(1.0, 0.9, 0.0, rng.gen_range(0.6..1.0)),
            1 => Color::srgba(1.0, 0.5, 0.0, rng.gen_range(0.6..1.0)),
            _ => Color::srgba(1.0, 0.2, 0.0, rng.gen_range(0.6..1.0)),
        };

        let offset = Vec2::new(rng.gen_range(-8.0..8.0), rng.gen_range(-8.0..8.0));

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x + offset.x, position.y + offset.y, 0.5),
            FireParticle {
                lifetime: Timer::from_seconds(rng.gen_range(0.3..0.8), TimerMode::Once),
                velocity,
            },
        ));
    }
}

pub fn animate_fire_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut fire_query: Query<(Entity, &mut Transform, &mut Sprite, &mut FireParticle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in fire_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();

        particle.velocity.y += 50.0 * time.delta_secs();
        particle.velocity *= 0.95;

        let progress = particle.lifetime.fraction();
        let scale = 1.0 + progress * 0.5;
        transform.scale = Vec3::splat(scale);

        let Srgba { alpha, .. } = sprite.color.to_srgba();
        let new_alpha = (1.0 - progress) * alpha;
        set_sprite_alpha(&mut sprite, new_alpha);

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
