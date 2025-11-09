use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Flammable, Health, RagdollPart};
use crate::constants::RAGDOLL_SPAWN_KEY;
use crate::drag::Draggable;

pub fn spawn_ragdoll_on_keypress(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.just_pressed(RAGDOLL_SPAWN_KEY) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                spawn_ragdoll(&mut commands, world_pos);
            }
        }
    }
}

fn spawn_ragdoll(commands: &mut Commands, position: Vec2) {
    let head_size = Vec2::new(20.0, 25.0);
    let torso_size = Vec2::new(30.0, 40.0);
    let upper_arm_size = Vec2::new(10.0, 25.0);
    let lower_arm_size = Vec2::new(8.0, 20.0);
    let upper_leg_size = Vec2::new(12.0, 30.0);
    let lower_leg_size = Vec2::new(10.0, 28.0);

    let head = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.7, 0.6),
                custom_size: Some(head_size),
                ..default()
            },
            Transform::from_xyz(position.x, position.y + 60.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(head_size.x / 2.0, head_size.y / 2.0),
            ColliderMassProperties::Density(1.2),
            Damping {
                linear_damping: 0.2,
                angular_damping: 0.5,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 100.0,
                max: 100.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let torso = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.4, 0.6, 0.8),
                custom_size: Some(torso_size),
                ..default()
            },
            Transform::from_xyz(position.x, position.y + 20.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(torso_size.x / 2.0, torso_size.y / 2.0),
            ColliderMassProperties::Density(1.5),
            Damping {
                linear_damping: 0.2,
                angular_damping: 0.5,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 150.0,
                max: 150.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let left_upper_arm = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.7, 0.6),
                custom_size: Some(upper_arm_size),
                ..default()
            },
            Transform::from_xyz(position.x - 25.0, position.y + 15.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(upper_arm_size.x / 2.0, upper_arm_size.y / 2.0),
            ColliderMassProperties::Density(1.0),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 80.0,
                max: 80.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let left_lower_arm = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.7, 0.6),
                custom_size: Some(lower_arm_size),
                ..default()
            },
            Transform::from_xyz(position.x - 25.0, position.y - 10.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(lower_arm_size.x / 2.0, lower_arm_size.y / 2.0),
            ColliderMassProperties::Density(1.0),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 70.0,
                max: 70.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let right_upper_arm = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.7, 0.6),
                custom_size: Some(upper_arm_size),
                ..default()
            },
            Transform::from_xyz(position.x + 25.0, position.y + 15.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(upper_arm_size.x / 2.0, upper_arm_size.y / 2.0),
            ColliderMassProperties::Density(1.0),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 80.0,
                max: 80.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let right_lower_arm = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.7, 0.6),
                custom_size: Some(lower_arm_size),
                ..default()
            },
            Transform::from_xyz(position.x + 25.0, position.y - 10.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(lower_arm_size.x / 2.0, lower_arm_size.y / 2.0),
            ColliderMassProperties::Density(1.0),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 70.0,
                max: 70.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let left_upper_leg = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.7),
                custom_size: Some(upper_leg_size),
                ..default()
            },
            Transform::from_xyz(position.x - 10.0, position.y - 15.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(upper_leg_size.x / 2.0, upper_leg_size.y / 2.0),
            ColliderMassProperties::Density(1.2),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 90.0,
                max: 90.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let left_lower_leg = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.7),
                custom_size: Some(lower_leg_size),
                ..default()
            },
            Transform::from_xyz(position.x - 10.0, position.y - 45.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(lower_leg_size.x / 2.0, lower_leg_size.y / 2.0),
            ColliderMassProperties::Density(1.2),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 80.0,
                max: 80.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let right_upper_leg = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.7),
                custom_size: Some(upper_leg_size),
                ..default()
            },
            Transform::from_xyz(position.x + 10.0, position.y - 15.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(upper_leg_size.x / 2.0, upper_leg_size.y / 2.0),
            ColliderMassProperties::Density(1.2),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 90.0,
                max: 90.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    let right_lower_leg = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.7),
                custom_size: Some(lower_leg_size),
                ..default()
            },
            Transform::from_xyz(position.x + 10.0, position.y - 45.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(lower_leg_size.x / 2.0, lower_leg_size.y / 2.0),
            ColliderMassProperties::Density(1.2),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.6,
            },
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: 80.0,
                max: 80.0,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
        ))
        .id();

    create_joint(
        commands,
        head,
        torso,
        Vec2::new(0.0, -head_size.y / 2.0),
        Vec2::new(0.0, torso_size.y / 2.0),
        -0.5,
        0.5,
    );

    create_joint(
        commands,
        torso,
        left_upper_arm,
        Vec2::new(-torso_size.x / 2.0, torso_size.y / 2.5),
        Vec2::new(0.0, upper_arm_size.y / 2.0),
        -2.0,
        2.0,
    );

    create_joint(
        commands,
        left_upper_arm,
        left_lower_arm,
        Vec2::new(0.0, -upper_arm_size.y / 2.0),
        Vec2::new(0.0, lower_arm_size.y / 2.0),
        0.0,
        2.5,
    );

    create_joint(
        commands,
        torso,
        right_upper_arm,
        Vec2::new(torso_size.x / 2.0, torso_size.y / 2.5),
        Vec2::new(0.0, upper_arm_size.y / 2.0),
        -2.0,
        2.0,
    );

    create_joint(
        commands,
        right_upper_arm,
        right_lower_arm,
        Vec2::new(0.0, -upper_arm_size.y / 2.0),
        Vec2::new(0.0, lower_arm_size.y / 2.0),
        0.0,
        2.5,
    );

    create_joint(
        commands,
        torso,
        left_upper_leg,
        Vec2::new(-8.0, -torso_size.y / 2.0),
        Vec2::new(0.0, upper_leg_size.y / 2.0),
        -1.5,
        1.0,
    );

    create_joint(
        commands,
        left_upper_leg,
        left_lower_leg,
        Vec2::new(0.0, -upper_leg_size.y / 2.0),
        Vec2::new(0.0, lower_leg_size.y / 2.0),
        -2.5,
        0.0,
    );

    create_joint(
        commands,
        torso,
        right_upper_leg,
        Vec2::new(8.0, -torso_size.y / 2.0),
        Vec2::new(0.0, upper_leg_size.y / 2.0),
        -1.5,
        1.0,
    );

    create_joint(
        commands,
        right_upper_leg,
        right_lower_leg,
        Vec2::new(0.0, -upper_leg_size.y / 2.0),
        Vec2::new(0.0, lower_leg_size.y / 2.0),
        -2.5,
        0.0,
    );
}

fn create_joint(
    commands: &mut Commands,
    parent: Entity,
    child: Entity,
    parent_anchor: Vec2,
    child_anchor: Vec2,
    min_angle: f32,
    max_angle: f32,
) {
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(parent_anchor)
        .local_anchor2(child_anchor)
        .limits([min_angle, max_angle]);

    commands.entity(child).insert(ImpulseJoint::new(parent, joint));
}
