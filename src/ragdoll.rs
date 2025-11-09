use bevy::prelude::*;

use crate::body_parts::{create_joint, spawn_body_part, BodyPartConfig, JointConfig};
use crate::constants::RAGDOLL_SPAWN_KEY;
use crate::utils::get_cursor_world_position;

pub fn spawn_ragdoll_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(RAGDOLL_SPAWN_KEY) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            spawn_ragdoll(&mut commands, world_pos);
        }
    }
}

pub fn spawn_ragdoll_from_ui(commands: &mut Commands, position: Vec2) {
    spawn_ragdoll(commands, position);
}

fn spawn_ragdoll(commands: &mut Commands, position: Vec2) {
    let skin_color = Color::srgb(0.9, 0.7, 0.6);
    let shirt_color = Color::srgb(0.4, 0.6, 0.8);
    let pants_color = Color::srgb(0.3, 0.5, 0.7);

    let head_size = Vec2::new(20.0, 25.0);
    let torso_size = Vec2::new(30.0, 40.0);
    let upper_arm_size = Vec2::new(10.0, 25.0);
    let lower_arm_size = Vec2::new(8.0, 20.0);
    let upper_leg_size = Vec2::new(12.0, 30.0);
    let lower_leg_size = Vec2::new(10.0, 28.0);

    let head = spawn_body_part(
        commands,
        BodyPartConfig {
            size: head_size,
            position: position + Vec2::new(0.0, 60.0),
            color: skin_color,
            density: 1.2,
            health: 100.0,
            ..Default::default()
        },
    );

    let torso = spawn_body_part(
        commands,
        BodyPartConfig {
            size: torso_size,
            position: position + Vec2::new(0.0, 20.0),
            color: shirt_color,
            density: 1.5,
            health: 150.0,
            ..Default::default()
        },
    );

    let left_upper_arm = spawn_body_part(
        commands,
        BodyPartConfig {
            size: upper_arm_size,
            position: position + Vec2::new(-25.0, 15.0),
            color: skin_color,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 80.0,
            ..Default::default()
        },
    );

    let left_lower_arm = spawn_body_part(
        commands,
        BodyPartConfig {
            size: lower_arm_size,
            position: position + Vec2::new(-25.0, -10.0),
            color: skin_color,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 70.0,
            ..Default::default()
        },
    );

    let right_upper_arm = spawn_body_part(
        commands,
        BodyPartConfig {
            size: upper_arm_size,
            position: position + Vec2::new(25.0, 15.0),
            color: skin_color,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 80.0,
            ..Default::default()
        },
    );

    let right_lower_arm = spawn_body_part(
        commands,
        BodyPartConfig {
            size: lower_arm_size,
            position: position + Vec2::new(25.0, -10.0),
            color: skin_color,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 70.0,
            ..Default::default()
        },
    );

    let left_upper_leg = spawn_body_part(
        commands,
        BodyPartConfig {
            size: upper_leg_size,
            position: position + Vec2::new(-10.0, -15.0),
            color: pants_color,
            density: 1.2,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 90.0,
            ..Default::default()
        },
    );

    let left_lower_leg = spawn_body_part(
        commands,
        BodyPartConfig {
            size: lower_leg_size,
            position: position + Vec2::new(-10.0, -45.0),
            color: pants_color,
            density: 1.2,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 80.0,
            ..Default::default()
        },
    );

    let right_upper_leg = spawn_body_part(
        commands,
        BodyPartConfig {
            size: upper_leg_size,
            position: position + Vec2::new(10.0, -15.0),
            color: pants_color,
            density: 1.2,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 90.0,
            ..Default::default()
        },
    );

    let right_lower_leg = spawn_body_part(
        commands,
        BodyPartConfig {
            size: lower_leg_size,
            position: position + Vec2::new(10.0, -45.0),
            color: pants_color,
            density: 1.2,
            linear_damping: 0.3,
            angular_damping: 0.6,
            health: 80.0,
            ..Default::default()
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: head,
            child: torso,
            parent_anchor: Vec2::new(0.0, -head_size.y / 2.0),
            child_anchor: Vec2::new(0.0, torso_size.y / 2.0),
            min_angle: -0.5,
            max_angle: 0.5,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: torso,
            child: left_upper_arm,
            parent_anchor: Vec2::new(-torso_size.x / 2.0, torso_size.y / 2.5),
            child_anchor: Vec2::new(0.0, upper_arm_size.y / 2.0),
            min_angle: -2.0,
            max_angle: 2.0,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: left_upper_arm,
            child: left_lower_arm,
            parent_anchor: Vec2::new(0.0, -upper_arm_size.y / 2.0),
            child_anchor: Vec2::new(0.0, lower_arm_size.y / 2.0),
            min_angle: 0.0,
            max_angle: 2.5,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: torso,
            child: right_upper_arm,
            parent_anchor: Vec2::new(torso_size.x / 2.0, torso_size.y / 2.5),
            child_anchor: Vec2::new(0.0, upper_arm_size.y / 2.0),
            min_angle: -2.0,
            max_angle: 2.0,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: right_upper_arm,
            child: right_lower_arm,
            parent_anchor: Vec2::new(0.0, -upper_arm_size.y / 2.0),
            child_anchor: Vec2::new(0.0, lower_arm_size.y / 2.0),
            min_angle: 0.0,
            max_angle: 2.5,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: torso,
            child: left_upper_leg,
            parent_anchor: Vec2::new(-8.0, -torso_size.y / 2.0),
            child_anchor: Vec2::new(0.0, upper_leg_size.y / 2.0),
            min_angle: -1.5,
            max_angle: 1.0,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: left_upper_leg,
            child: left_lower_leg,
            parent_anchor: Vec2::new(0.0, -upper_leg_size.y / 2.0),
            child_anchor: Vec2::new(0.0, lower_leg_size.y / 2.0),
            min_angle: -2.5,
            max_angle: 0.0,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: torso,
            child: right_upper_leg,
            parent_anchor: Vec2::new(8.0, -torso_size.y / 2.0),
            child_anchor: Vec2::new(0.0, upper_leg_size.y / 2.0),
            min_angle: -1.5,
            max_angle: 1.0,
        },
    );

    create_joint(
        commands,
        JointConfig {
            parent: right_upper_leg,
            child: right_lower_leg,
            parent_anchor: Vec2::new(0.0, -upper_leg_size.y / 2.0),
            child_anchor: Vec2::new(0.0, lower_leg_size.y / 2.0),
            min_angle: -2.5,
            max_angle: 0.0,
        },
    );
}
