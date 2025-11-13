use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::components::{Flammable, Health};
use crate::core::constants::WOODEN_BOX_SPAWN_KEY;
use crate::core::utils::get_cursor_world_position;
use crate::systems::damage::connection::Connectable;
use crate::systems::input::drag::Draggable;

#[derive(Component)]
pub struct WoodenBox;

pub fn spawn_wooden_box_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(WOODEN_BOX_SPAWN_KEY) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            spawn_wooden_box(&mut commands, world_pos);
        }
    }
}

pub fn spawn_wooden_box_from_ui(commands: &mut Commands, position: Vec2) {
    spawn_wooden_box(commands, position);
}

fn spawn_wooden_box(commands: &mut Commands, position: Vec2) {
    let width = 60.0;
    let height = 60.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(width / 2.0, height / 2.0),
        ColliderMassProperties::Density(0.8),
        Restitution::coefficient(0.3),
        Friction::coefficient(0.7),
        ExternalImpulse::default(),
        Velocity::default(),
        WoodenBox,
        Draggable,
        Connectable,
        Health {
            current: 100.0,
            max: 100.0,
        },
        Flammable {
            ignition_threshold: 0.5,
        },
    ));
}
