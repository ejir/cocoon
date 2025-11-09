use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::blood::spawn_blood_particles;
use crate::components::{Health, RagdollPart, ShockwaveRing};
use crate::damage::{Fractured, JointHealth};
use crate::explosion::spawn_object_fragments;
use crate::wooden_box::WoodenBox;

pub fn spawn_shockwave(commands: &mut Commands, position: Vec2, max_radius: f32, peak_pressure: f32) {
    commands.spawn(ShockwaveRing {
        origin: position,
        current_radius: 0.0,
        max_radius,
        peak_pressure,
        wave_speed: 1200.0,
        wave_thickness: 80.0,
        lifetime: Timer::from_seconds(1.5, TimerMode::Once),
    });
}

pub fn update_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    mut shockwave_query: Query<(Entity, &mut ShockwaveRing)>,
    mut physics_query: Query<(
        Entity,
        &Transform,
        &mut ExternalImpulse,
        Option<&mut Health>,
        Option<&RagdollPart>,
        Option<&WoodenBox>,
        Option<&ReadMassProperties>,
        Option<&Sprite>,
        Option<&Velocity>,
    ), With<RigidBody>>,
) {
    for (shockwave_entity, mut shockwave) in shockwave_query.iter_mut() {
        shockwave.lifetime.tick(time.delta());
        
        let old_radius = shockwave.current_radius;
        shockwave.current_radius += shockwave.wave_speed * time.delta_secs();
        
        let wave_decay = 1.0 - (shockwave.current_radius / shockwave.max_radius).powf(1.5);
        
        if shockwave.current_radius >= shockwave.max_radius || shockwave.lifetime.finished() {
            commands.entity(shockwave_entity).despawn();
            continue;
        }
        
        for (entity, transform, mut impulse, health_opt, ragdoll_opt, wooden_box_opt, mass_props_opt, sprite_opt, velocity_opt) in physics_query.iter_mut() {
            let pos = transform.translation.truncate();
            let distance = pos.distance(shockwave.origin);
            
            if distance >= old_radius && distance < shockwave.current_radius + shockwave.wave_thickness {
                let direction = if distance > 1.0 {
                    (pos - shockwave.origin).normalize()
                } else {
                    Vec2::new(1.0, 0.0)
                };
                
                let distance_factor = if distance < 1.0 { 
                    1.0 
                } else { 
                    (shockwave.max_radius / distance).sqrt()
                };
                
                let pressure = shockwave.peak_pressure * wave_decay * distance_factor;
                
                let mass = if let Some(mass_props) = mass_props_opt {
                    mass_props.mass.max(0.1)
                } else {
                    1.0
                };
                
                let cross_section = transform.scale.x.max(transform.scale.y).max(20.0);
                
                let impulse_magnitude = pressure * cross_section * time.delta_secs();
                let impulse_vec = direction * impulse_magnitude;
                
                impulse.impulse += impulse_vec;
                
                let torque_factor = (distance / shockwave.max_radius).clamp(0.0, 1.0);
                let random_torque = (rand::random::<f32>() - 0.5) * 2.0;
                let torque = random_torque * impulse_magnitude * 0.1 * (1.0 - torque_factor);
                impulse.torque_impulse += torque;
                
                if let Some(mut health) = health_opt {
                    if ragdoll_opt.is_some() || wooden_box_opt.is_some() {
                        let base_damage = pressure * 0.0008;
                        
                        let velocity_factor = (impulse_magnitude / mass).min(1000.0) / 1000.0;
                        let damage = base_damage * (1.0 + velocity_factor * 2.0);
                        
                        health.current -= damage;
                        
                        if health.current <= 0.0 {
                            let current_velocity = velocity_opt
                                .map(|v| v.linvel)
                                .unwrap_or(direction * (pressure * 0.3).min(500.0));
                            
                            if let Some(sprite) = sprite_opt {
                                let size = sprite.custom_size.unwrap_or(Vec2::new(20.0, 20.0));
                                let color = sprite.color;
                                
                                spawn_object_fragments(
                                    &mut commands,
                                    pos,
                                    size,
                                    color,
                                    current_velocity,
                                    wooden_box_opt.is_some(),
                                );
                            }
                            
                            if ragdoll_opt.is_some() {
                                let blood_velocity = direction * (pressure * 0.5).min(800.0);
                                spawn_blood_particles(&mut commands, pos, blood_velocity);
                            }
                            
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_shockwave_visuals(commands: &mut Commands, position: Vec2, max_radius: f32) {
    for i in 0..5 {
        let delay = i as f32 * 0.03;
        let initial_offset = i as f32 * 15.0;
        
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.9, 0.7, 0.6),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.6 + i as f32 * 0.1),
            ShockwaveVisual {
                timer: Timer::from_seconds(0.6 + delay, TimerMode::Once),
                max_radius: max_radius * 0.9,
                start_radius: initial_offset,
                start_delay: Timer::from_seconds(delay, TimerMode::Once),
            },
        ));
    }
    
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.95, 0.8, 0.9),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.0),
        ExplosionCore {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
            max_scale: 12.0,
        },
    ));
}

#[derive(Component)]
pub struct ShockwaveVisual {
    pub timer: Timer,
    pub max_radius: f32,
    pub start_radius: f32,
    pub start_delay: Timer,
}

#[derive(Component)]
pub struct ExplosionCore {
    pub timer: Timer,
    pub max_scale: f32,
}

pub fn animate_shockwave_visual(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ShockwaveVisual, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut visual, mut transform, mut sprite) in query.iter_mut() {
        if !visual.start_delay.finished() {
            visual.start_delay.tick(time.delta());
            continue;
        }
        
        visual.timer.tick(time.delta());
        
        let progress = visual.timer.fraction();
        
        let eased_progress = 1.0 - (1.0 - progress).powi(3);
        
        let current_radius = visual.start_radius + (visual.max_radius - visual.start_radius) * eased_progress;
        
        transform.scale = Vec3::new(
            current_radius / 5.0,
            current_radius / 5.0,
            1.0,
        );
        
        let alpha = (1.0 - progress).powf(0.7) * 0.6;
        let intensity = 1.0 - progress * 0.3;
        
        sprite.color = Color::srgba(
            intensity * 1.0,
            intensity * 0.9,
            intensity * 0.7,
            alpha,
        );
        
        if visual.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_explosion_core(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionCore, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut core, mut transform, mut sprite) in query.iter_mut() {
        core.timer.tick(time.delta());
        
        let progress = core.timer.fraction();
        
        let scale = if progress < 0.3 {
            let expand = progress / 0.3;
            1.0 + (core.max_scale - 1.0) * expand
        } else {
            let contract = (progress - 0.3) / 0.7;
            core.max_scale * (1.0 - contract * 0.6)
        };
        
        transform.scale = Vec3::splat(scale);
        
        let alpha = 1.0 - progress;
        let temp = 1.0 - progress * 0.4;
        
        sprite.color = Color::srgba(
            1.0,
            temp * 0.95,
            temp * 0.8,
            alpha,
        );
        
        if core.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn shockwave_joint_damage(
    mut commands: Commands,
    shockwave_query: Query<&ShockwaveRing>,
    mut joint_query: Query<(Entity, &mut JointHealth, &Transform), With<RagdollPart>>,
) {
    for shockwave in shockwave_query.iter() {
        for (entity, mut joint_health, transform) in joint_query.iter_mut() {
            let pos = transform.translation.truncate();
            let distance = pos.distance(shockwave.origin);
            
            if distance < shockwave.current_radius + shockwave.wave_thickness 
                && distance >= shockwave.current_radius - shockwave.wave_thickness {
                
                let distance_factor = if distance < 1.0 { 
                    1.0 
                } else { 
                    (shockwave.max_radius / distance).sqrt()
                };
                
                let pressure = shockwave.peak_pressure * distance_factor;
                let joint_damage = pressure * 0.004;
                
                joint_health.current -= joint_damage;
                
                if joint_health.current <= 0.0 {
                    spawn_blood_particles(&mut commands, pos, Vec2::ZERO);
                    commands.entity(entity).remove::<ImpulseJoint>();
                    commands.entity(entity).remove::<JointHealth>();
                } else if joint_health.current < joint_health.max * 0.5 {
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
