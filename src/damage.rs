use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::blood::spawn_blood_particles;
use crate::components::RagdollPart;

#[derive(Component)]
pub struct JointHealth {
    pub current: f32,
    pub max: f32,
    pub parent_entity: Entity,
}

#[derive(Component)]
pub struct Fractured {
    pub severity: f32,
}

#[derive(Component)]
pub struct PreviousVelocity {
    pub linvel: Vec2,
    pub angvel: f32,
}

pub fn check_joint_damage(
    mut commands: Commands,
    mut joint_query: Query<(Entity, &mut JointHealth, &Transform)>,
    velocity_query: Query<&Velocity, With<RagdollPart>>,
) {
    for (entity, mut joint_health, transform) in joint_query.iter_mut() {
        let child_velocity = velocity_query.get(entity).ok();
        let parent_velocity = velocity_query.get(joint_health.parent_entity).ok();
        
        if let (Some(child_vel), Some(parent_vel)) = (child_velocity, parent_velocity) {
            let velocity_diff = (child_vel.linvel - parent_vel.linvel).length();
            let angular_diff = (child_vel.angvel - parent_vel.angvel).abs();
            
            // More sensitive stress calculation
            let stress = velocity_diff * 0.015 + angular_diff * 0.15;
            
            // Lower threshold for joint stress damage
            if stress > 3.0 {
                let mut damage = (stress - 3.0) * 0.7;
                
                // Extra damage for extreme stress (violent movements)
                if stress > 15.0 {
                    let extreme_stress_multiplier = (stress / 15.0).min(2.0);
                    damage *= extreme_stress_multiplier;
                }
                
                joint_health.current -= damage;
                
                if joint_health.current <= 0.0 {
                    let position = transform.translation.truncate();
                    // Use velocity difference for blood spray direction
                    let blood_vel = (child_vel.linvel - parent_vel.linvel) * 0.3;
                    spawn_blood_particles(&mut commands, position, blood_vel);
                    
                    commands.entity(entity).remove::<ImpulseJoint>();
                    commands.entity(entity).remove::<JointHealth>();
                } else if joint_health.current < joint_health.max * 0.5 && joint_health.current > 0.0 {
                    if commands.get_entity(entity).is_some() {
                        commands.entity(entity).insert(Fractured {
                            severity: 1.0 - (joint_health.current / joint_health.max),
                        });
                    }
                }
            }
        }
    }
}

pub fn apply_explosive_joint_damage(
    mut commands: Commands,
    mut joint_query: Query<(Entity, &mut JointHealth, &Transform)>,
    explosion_force_query: Query<(Entity, &Transform, &ExternalImpulse), (With<RagdollPart>, Changed<ExternalImpulse>)>,
) {
    for (force_entity, force_transform, impulse) in explosion_force_query.iter() {
        let impulse_magnitude = impulse.impulse.length();
        
        if impulse_magnitude > 100.0 {
            for (joint_entity, mut joint_health, joint_transform) in joint_query.iter_mut() {
                let distance = force_transform.translation.truncate().distance(joint_transform.translation.truncate());
                
                if distance < 50.0 && joint_entity == force_entity {
                    let damage = impulse_magnitude * 0.02;
                    joint_health.current -= damage;
                    
                    if joint_health.current <= 0.0 {
                        let position = joint_transform.translation.truncate();
                        spawn_blood_particles(&mut commands, position, Vec2::ZERO);
                        
                        commands.entity(joint_entity).remove::<ImpulseJoint>();
                        commands.entity(joint_entity).remove::<JointHealth>();
                    } else if joint_health.current < joint_health.max * 0.5 {
                        if commands.get_entity(joint_entity).is_some() {
                            commands.entity(joint_entity).insert(Fractured {
                                severity: 1.0 - (joint_health.current / joint_health.max),
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn visualize_fractures(
    mut query: Query<(&mut Sprite, &Fractured), With<RagdollPart>>,
) {
    for (mut sprite, fractured) in query.iter_mut() {
        let base_color = sprite.color;
        let Srgba { red, green, blue, alpha } = base_color.to_srgba();
        
        let darken = fractured.severity * 0.4;
        sprite.color = Color::srgba(
            (red - darken).max(0.0),
            (green - darken).max(0.0),
            (blue - darken).max(0.0),
            alpha,
        );
    }
}

pub fn track_velocity(
    mut commands: Commands,
    query: Query<(Entity, &Velocity), With<RagdollPart>>,
) {
    for (entity, velocity) in query.iter() {
        commands.entity(entity).insert(PreviousVelocity {
            linvel: velocity.linvel,
            angvel: velocity.angvel,
        });
    }
}

pub fn detect_impact_damage(
    mut commands: Commands,
    mut joint_query: Query<(Entity, &mut JointHealth, &Transform), Without<Velocity>>,
    mut ragdoll_query: Query<(Entity, &Velocity, Option<&PreviousVelocity>), With<RagdollPart>>,
) {
    for (entity, velocity, prev_velocity) in ragdoll_query.iter_mut() {
        if let Some(prev) = prev_velocity {
            let velocity_change = (velocity.linvel - prev.linvel).length();
            let angular_change = (velocity.angvel - prev.angvel).abs();
            
            // Detect vertical impact (falling) for enhanced damage
            let vertical_impact = (prev.linvel.y - velocity.linvel.y).abs();
            let is_falling_impact = prev.linvel.y < -200.0 && velocity_change > 200.0;
            
            // Lower threshold for impact detection, more sensitive to impacts
            if velocity_change > 200.0 || angular_change > 8.0 || is_falling_impact {
                for (joint_entity, mut joint_health, joint_transform) in joint_query.iter_mut() {
                    if joint_entity == entity {
                        // Base impact damage calculation
                        let mut impact_damage = velocity_change * 0.05 + angular_change * 0.8;
                        
                        // Apply enhanced damage for high-velocity impacts (falling from height)
                        if velocity_change > 500.0 {
                            let high_impact_multiplier = (velocity_change / 500.0).min(3.0);
                            impact_damage *= high_impact_multiplier;
                        }
                        
                        // Extra damage for vertical falls to simulate ground impact
                        if is_falling_impact {
                            let fall_damage_bonus = vertical_impact * 0.08;
                            impact_damage += fall_damage_bonus;
                        }
                        
                        joint_health.current -= impact_damage;
                        
                        if joint_health.current <= 0.0 {
                            let position = joint_transform.translation.truncate();
                            // More dramatic blood spray for high impacts
                            let blood_velocity = if velocity_change > 500.0 {
                                velocity.linvel * 0.5
                            } else {
                                velocity.linvel * 0.3
                            };
                            spawn_blood_particles(&mut commands, position, blood_velocity);
                            
                            commands.entity(joint_entity).remove::<ImpulseJoint>();
                            commands.entity(joint_entity).remove::<JointHealth>();
                        } else if joint_health.current < joint_health.max * 0.5 {
                            if commands.get_entity(joint_entity).is_some() {
                                commands.entity(joint_entity).insert(Fractured {
                                    severity: 1.0 - (joint_health.current / joint_health.max),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn collision_joint_damage(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut joint_query: Query<(Entity, &mut JointHealth, &Transform, Option<&Velocity>), With<RagdollPart>>,
    mass_query: Query<Option<&ReadMassProperties>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            for entity in [entity1, entity2] {
                if let Ok((joint_entity, mut joint_health, transform, velocity_opt)) = joint_query.get_mut(*entity) {
                    let velocity = velocity_opt.map(|v| v.linvel.length()).unwrap_or(0.0);
                    
                    // Lower threshold and more aggressive damage for collisions
                    if velocity > 150.0 {
                        // Base collision damage with improved scaling
                        let mut collision_damage = (velocity - 150.0) * 0.08;
                        
                        // Check if colliding with a heavy object (iron block, etc)
                        let other_entity = if entity == entity1 { entity2 } else { entity1 };
                        if let Ok(other_mass_opt) = mass_query.get(*other_entity) {
                            if let Some(other_mass_props) = other_mass_opt {
                                // Extra damage if hit by heavy object
                                let mass = other_mass_props.mass;
                                if mass > 5.0 {
                                    let mass_multiplier = (mass / 5.0).min(2.5);
                                    collision_damage *= mass_multiplier;
                                }
                            }
                        }
                        
                        // Extra damage for very high velocity collisions
                        if velocity > 400.0 {
                            let high_velocity_bonus = (velocity - 400.0) * 0.06;
                            collision_damage += high_velocity_bonus;
                        }
                        
                        joint_health.current -= collision_damage;
                        
                        if joint_health.current <= 0.0 {
                            let position = transform.translation.truncate();
                            let vel_dir = velocity_opt.map(|v| v.linvel).unwrap_or(Vec2::ZERO);
                            // More blood for higher velocity collisions
                            let blood_velocity = if velocity > 400.0 {
                                vel_dir * 0.4
                            } else {
                                vel_dir * 0.2
                            };
                            spawn_blood_particles(&mut commands, position, blood_velocity);
                            
                            commands.entity(joint_entity).remove::<ImpulseJoint>();
                            commands.entity(joint_entity).remove::<JointHealth>();
                        } else if joint_health.current < joint_health.max * 0.5 {
                            if commands.get_entity(joint_entity).is_some() {
                                commands.entity(joint_entity).insert(Fractured {
                                    severity: 1.0 - (joint_health.current / joint_health.max),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
