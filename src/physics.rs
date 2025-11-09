use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::blood::spawn_blood_particles;
use crate::components::{Debris, Explosion, Health, RagdollPart};
use crate::physics_utils::apply_radial_impulse;
use crate::utils::fade_sprite_alpha;
use crate::wooden_box::WoodenBox;

pub fn apply_explosion(
    mut commands: Commands,
    explosion_query: Query<(Entity, &Explosion)>,
    mut physics_query: Query<
        (
            Entity,
            &Transform,
            &mut ExternalImpulse,
            Option<&mut Health>,
            Option<&RagdollPart>,
            Option<&WoodenBox>,
        ),
        With<RigidBody>,
    >,
) {
    for (explosion_entity, explosion) in explosion_query.iter() {
        for (entity, transform, mut impulse, health_opt, ragdoll_opt, wooden_box_opt) in
            physics_query.iter_mut()
        {
            let pos = transform.translation.truncate();

            let strength = apply_radial_impulse(
                &mut impulse,
                pos,
                explosion.position,
                explosion.radius,
                explosion.force,
                true,
            );

            if strength > 0.0 {
                if let Some(mut health) = health_opt {
                    if ragdoll_opt.is_some() || wooden_box_opt.is_some() {
                        let damage = strength * 0.002;
                        health.current -= damage;

                        if health.current <= 0.0 {
                            if ragdoll_opt.is_some() {
                                let direction = (pos - explosion.position).normalize();
                                spawn_blood_particles(
                                    &mut commands,
                                    pos,
                                    direction * strength * 0.01,
                                );
                            }
                            commands.entity(entity).despawn();
                        }
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

        fade_sprite_alpha(&mut sprite, time.delta_secs() * 0.3);

        let Srgba { alpha, .. } = sprite.color.to_srgba();
        if alpha <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
