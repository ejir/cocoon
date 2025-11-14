use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::components::{Flammable, Health, RagdollPart};
use crate::systems::damage::connection::Connectable;
use crate::systems::damage::damage::JointHealth;
use crate::systems::input::drag::Draggable;

pub struct BodyPartConfig {
    pub size: Vec2,
    pub position: Vec2,
    pub color: Color,
    pub density: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub health: f32,
}

impl Default for BodyPartConfig {
    fn default() -> Self {
        Self {
            size: Vec2::new(20.0, 20.0),
            position: Vec2::ZERO,
            color: Color::srgb(0.9, 0.7, 0.6),
            density: 1.0,
            linear_damping: 0.2,
            angular_damping: 0.5,
            health: 100.0,
        }
    }
}

pub fn spawn_body_part(commands: &mut Commands, config: BodyPartConfig) -> Entity {
    commands
        .spawn((
            Sprite {
                color: config.color,
                custom_size: Some(config.size),
                ..default()
            },
            Transform::from_xyz(config.position.x, config.position.y, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(config.size.x / 2.0, config.size.y / 2.0),
            ColliderMassProperties::Density(config.density),
            Damping {
                linear_damping: config.linear_damping,
                angular_damping: config.angular_damping,
            },
            Velocity::zero(),  // Initialize with zero velocity for stable joint creation
            ExternalImpulse::default(),
            RagdollPart,
            Health {
                current: config.health,
                max: config.health,
            },
            Flammable {
                ignition_threshold: 0.5,
            },
            Draggable,
            Connectable,
        ))
        .id()
}

pub struct JointConfig {
    pub parent: Entity,
    pub child: Entity,
    pub parent_anchor: Vec2,
    pub child_anchor: Vec2,
    pub min_angle: f32,
    pub max_angle: f32,
}

pub fn create_joint(commands: &mut Commands, config: JointConfig) {
    // Anti-vibration ragdoll joint configuration:
    // - Apply high damping (> 3) for soft tissue simulation via motor force
    // - Local anchors must be precisely at joint connection points
    // - Motor max force provides damping to prevent oscillation
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(config.parent_anchor)
        .local_anchor2(config.child_anchor)
        .limits([config.min_angle, config.max_angle])
        .motor_model(MotorModel::ForceBased)
        .motor_max_force(350.0);  // High damping for soft tissue (damping > 3)

    // CRITICAL: Zero out velocities at joint creation to prevent explosion
    // This is essential for PPG (People Playground) style physics stability
    commands.entity(config.parent).insert(Velocity::zero());
    commands.entity(config.child).insert(Velocity::zero());

    commands
        .entity(config.child)
        .insert((
            ImpulseJoint::new(config.parent, joint),
            JointHealth {
                current: 100.0,
                max: 100.0,
                parent_entity: config.parent,
            },
        ));
}
