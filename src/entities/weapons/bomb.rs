use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::components::Bomb;
use crate::core::constants::{BOMB_SPAWN_KEY, EXPLOSION_RADIUS};
use crate::core::utils::get_cursor_world_position;
use crate::entities::weapons::explosion::{spawn_explosion_debris, spawn_smoke_particles};
use crate::entities::weapons::shockwave::{spawn_shockwave, spawn_shockwave_visuals};
use crate::systems::damage::connection::Connectable;
use crate::systems::input::drag::Draggable;

pub fn spawn_bomb_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(BOMB_SPAWN_KEY) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            spawn_bomb(&mut commands, world_pos);
        }
    }
}

pub fn spawn_bomb_from_ui(commands: &mut Commands, position: Vec2) {
    spawn_bomb(commands, position);
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
        Velocity::zero(),  // Initialize with zero velocity for stable physics
        ExternalImpulse::default(),
        Bomb {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        Draggable,
        Connectable,
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

            let peak_pressure = 80000.0;
            spawn_shockwave(&mut commands, position, EXPLOSION_RADIUS, peak_pressure);
            
            spawn_shockwave_visuals(&mut commands, position, EXPLOSION_RADIUS);
            spawn_explosion_debris(&mut commands, position);
            spawn_smoke_particles(&mut commands, position);
        }
    }
}
