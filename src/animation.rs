use bevy::prelude::*;

use crate::components::{ExplosionFlash, ExplosionShockwave, SmokeParticle};
use crate::utils::set_sprite_alpha;

pub fn animate_explosion_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionFlash, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut flash, mut transform, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());
        
        let progress = flash.timer.fraction();
        let scale = 1.0 + (flash.max_scale - 1.0) * progress;
        transform.scale = Vec3::splat(scale);
        
        let alpha = 1.0 - progress;
        set_sprite_alpha(&mut sprite, alpha);
        
        if flash.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_explosion_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionShockwave, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut shockwave, mut transform, mut sprite) in query.iter_mut() {
        shockwave.timer.tick(time.delta());
        
        let progress = shockwave.timer.fraction();
        let current_radius = shockwave.start_radius 
            + (shockwave.max_radius - shockwave.start_radius) * progress;
        
        transform.scale = Vec3::new(current_radius / 5.0, current_radius / 5.0, 1.0);
        
        let alpha = (1.0 - progress) * 0.8;
        let brightness = 1.0 - progress * 0.5;
        sprite.color = Color::srgba(
            brightness * 1.0,
            brightness * 0.6,
            brightness * 0.1,
            alpha
        );
        
        if shockwave.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_smoke_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SmokeParticle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut smoke, mut transform, mut sprite) in query.iter_mut() {
        smoke.lifetime.tick(time.delta());
        
        transform.translation += smoke.velocity.extend(0.0) * time.delta_secs();
        
        smoke.velocity.y += 30.0 * time.delta_secs();
        smoke.velocity *= 0.98;
        
        let progress = smoke.lifetime.fraction();
        let scale = 1.0 + progress * 2.0;
        transform.scale = Vec3::splat(scale);
        
        let alpha = (1.0 - progress) * 0.6;
        set_sprite_alpha(&mut sprite, alpha);
        
        if smoke.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
