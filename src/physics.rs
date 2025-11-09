use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::blood::spawn_blood_particles;
use crate::components::{Debris, Explosion, Health, RagdollPart};

pub fn apply_explosion(
    mut commands: Commands,
    explosion_query: Query<(Entity, &Explosion)>,
    mut physics_query: Query<(
        Entity,
        &Transform,
        &mut ExternalImpulse,
        Option<&mut Health>,
        Option<&RagdollPart>,
    ), With<RigidBody>>,
) {
    for (explosion_entity, explosion) in explosion_query.iter() {
        for (entity, transform, mut impulse, health_opt, ragdoll_opt) in physics_query.iter_mut() {
            let pos = transform.translation.truncate();
            let delta = pos - explosion.position;
            let distance = delta.length();

            if distance < explosion.radius && distance > 0.1 {
                let direction = delta.normalize();
                let strength = (1.0 - distance / explosion.radius) * explosion.force;
                let force = direction * strength;

                impulse.impulse += force;

                let torque = rand::thread_rng().gen_range(-5000.0..5000.0)
                    * (1.0 - distance / explosion.radius);
                impulse.torque_impulse += torque;

                if let (Some(mut health), Some(_ragdoll)) = (health_opt, ragdoll_opt) {
                    let damage = strength * 0.002;
                    health.current -= damage;

                    if health.current <= 0.0 {
                        spawn_blood_particles(&mut commands, pos, direction * strength * 0.01);
                        commands.entity(entity).despawn();
                    }
                }
            }
        }

        commands.entity(explosion_entity).despawn();
    }
}

pub fn cleanup_debris(
    mut commands: Commands,
    time: Res<Time>,
    mut debris_query: Query<(Entity, &Transform, &mut Sprite), With<Debris>>,
) {
    for (entity, transform, mut sprite) in debris_query.iter_mut() {
        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }

        let Srgba { red, green, blue, alpha } = sprite.color.to_srgba();
        let new_alpha = alpha - time.delta_secs() * 0.3;
        if new_alpha <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            sprite.color = Color::srgba(red, green, blue, new_alpha);
        }
    }
}
