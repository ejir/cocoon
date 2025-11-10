use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Health;
use crate::constants::IRON_BLOCK_SPAWN_KEY;
use crate::drag::Draggable;
use crate::utils::get_cursor_world_position;

#[derive(Component)]
pub struct IronBlock;

pub fn spawn_iron_block_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(IRON_BLOCK_SPAWN_KEY) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            spawn_iron_block(&mut commands, world_pos);
        }
    }
}

pub fn spawn_iron_block_from_ui(commands: &mut Commands, position: Vec2) {
    spawn_iron_block(commands, position);
}

fn spawn_iron_block(commands: &mut Commands, position: Vec2) {
    let width = 60.0;
    let height = 60.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.6, 0.65),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(width / 2.0, height / 2.0),
        ColliderMassProperties::Density(7.8),
        Restitution::coefficient(0.2),
        Friction::coefficient(0.5),
        ExternalImpulse::default(),
        Velocity::default(),
        IronBlock,
        Draggable,
        Health {
            current: 300.0,
            max: 300.0,
        },
    ));
}
