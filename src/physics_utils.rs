use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct RigidBodyConfig {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub body_type: RigidBody,
    pub collider_type: ColliderType,
    pub density: f32,
    pub restitution: Option<f32>,
    pub friction: Option<f32>,
}

pub enum ColliderType {
    Cuboid,
    Ball,
}

impl Default for RigidBodyConfig {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(50.0, 50.0),
            color: Color::WHITE,
            body_type: RigidBody::Dynamic,
            collider_type: ColliderType::Cuboid,
            density: 1.0,
            restitution: None,
            friction: None,
        }
    }
}

pub fn spawn_physics_sprite(commands: &mut Commands, config: RigidBodyConfig) -> Entity {
    let collider = match config.collider_type {
        ColliderType::Cuboid => Collider::cuboid(config.size.x / 2.0, config.size.y / 2.0),
        ColliderType::Ball => Collider::ball(config.size.x / 2.0),
    };
    
    let mut entity_commands = commands.spawn((
        Sprite {
            color: config.color,
            custom_size: Some(config.size),
            ..default()
        },
        Transform::from_xyz(config.position.x, config.position.y, 0.0),
        config.body_type,
        collider,
        ColliderMassProperties::Density(config.density),
        ExternalImpulse::default(),
    ));
    
    if let Some(restitution) = config.restitution {
        entity_commands.insert(Restitution::coefficient(restitution));
    }
    
    if let Some(friction) = config.friction {
        entity_commands.insert(Friction::coefficient(friction));
    }
    
    entity_commands.id()
}

pub fn apply_radial_impulse(
    impulse: &mut ExternalImpulse,
    object_pos: Vec2,
    explosion_pos: Vec2,
    radius: f32,
    force: f32,
    apply_torque: bool,
) -> f32 {
    let delta = object_pos - explosion_pos;
    let distance = delta.length();
    
    if distance < radius && distance > 0.1 {
        let direction = delta.normalize();
        let strength = (1.0 - distance / radius) * force;
        let force_vec = direction * strength;
        
        impulse.impulse += force_vec;
        
        if apply_torque {
            let torque = rand::random::<f32>() * 10000.0 - 5000.0;
            let torque_scaled = torque * (1.0 - distance / radius);
            impulse.torque_impulse += torque_scaled;
        }
        
        strength
    } else {
        0.0
    }
}
