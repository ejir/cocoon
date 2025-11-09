use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Bomb, Explosion};
use crate::constants::{BOMB_SPAWN_KEY, EXPLOSION_FORCE, EXPLOSION_RADIUS};
use crate::explosion::{spawn_explosion_debris, spawn_explosion_visuals, spawn_smoke_particles};

pub fn spawn_bomb_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(BOMB_SPAWN_KEY) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                spawn_bomb(&mut commands, world_pos);
            }
        }
    }
}

fn spawn_bomb(commands: &mut Commands, position: Vec2) {
    let radius = 15.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        Collider::ball(radius),
        ColliderMassProperties::Density(2.0),
        Restitution::coefficient(0.5),
        ExternalImpulse::default(),
        Bomb {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

pub fn bomb_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bomb, &Transform)>,
) {
    for (entity, mut bomb, transform) in query.iter_mut() {
        bomb.timer.tick(time.delta());

        if bomb.timer.just_finished() {
            let position = transform.translation.truncate();

            commands.entity(entity).despawn();

            commands.spawn(Explosion {
                position,
                radius: EXPLOSION_RADIUS,
                force: EXPLOSION_FORCE,
            });

            spawn_explosion_visuals(&mut commands, position);
            spawn_explosion_debris(&mut commands, position);
            spawn_smoke_particles(&mut commands, position);
        }
    }
}
