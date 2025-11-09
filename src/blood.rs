use bevy::prelude::*;
use rand::Rng;

use crate::components::BloodParticle;
use crate::utils::set_sprite_alpha;

pub fn spawn_blood_particles(commands: &mut Commands, position: Vec2, impulse_direction: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..25 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..200.0);
        let base_velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        let velocity = base_velocity + impulse_direction * 0.3;

        let size = rng.gen_range(3.0..8.0);
        let red = rng.gen_range(0.6..0.9);
        let color = Color::srgba(red, 0.0, 0.0, rng.gen_range(0.8..1.0));

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, -0.3),
            BloodParticle {
                lifetime: Timer::from_seconds(rng.gen_range(1.0..3.0), TimerMode::Once),
                velocity,
            },
        ));
    }

    for _ in 0..15 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(100.0..300.0);
        let base_velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        let velocity = base_velocity + impulse_direction * 0.5;

        let size = rng.gen_range(2.0..5.0);
        let color = Color::srgba(0.8, 0.0, 0.0, 1.0);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, -0.3),
            BloodParticle {
                lifetime: Timer::from_seconds(rng.gen_range(0.5..2.0), TimerMode::Once),
                velocity,
            },
        ));
    }
}

pub fn animate_blood_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut BloodParticle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += particle.velocity.x * time.delta_secs();
            transform.translation.y += particle.velocity.y * time.delta_secs();

            particle.velocity.y -= 400.0 * time.delta_secs();

            let progress = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
            let new_alpha = 1.0 - progress;
            set_sprite_alpha(&mut sprite, new_alpha);
        }
    }
}
