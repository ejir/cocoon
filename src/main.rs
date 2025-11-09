use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const EXPLOSION_RADIUS: f32 = 400.0;
const EXPLOSION_FORCE: f32 = 150000.0;
const RAGDOLL_SPAWN_KEY: KeyCode = KeyCode::KeyR;
const BOMB_SPAWN_KEY: KeyCode = KeyCode::KeyB;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D Ragdoll Sandbox".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_ragdoll_on_keypress,
                spawn_bomb_on_keypress,
                bomb_timer_system,
                apply_explosion,
                cleanup_debris,
                animate_explosion_flash,
                animate_explosion_shockwave,
                animate_smoke_particles,
            ),
        )
        .run();
}

#[derive(Component)]
struct Bomb {
    timer: Timer,
}

#[derive(Component)]
struct Explosion {
    position: Vec2,
    radius: f32,
    force: f32,
}

#[derive(Component)]
struct RagdollPart;

#[derive(Component)]
struct Debris;

#[derive(Component)]
struct ExplosionFlash {
    timer: Timer,
    max_scale: f32,
}

#[derive(Component)]
struct ExplosionShockwave {
    timer: Timer,
    max_radius: f32,
    start_radius: f32,
}

#[derive(Component)]
struct SmokeParticle {
    lifetime: Timer,
    velocity: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    spawn_ground(&mut commands);

    commands.spawn((
        Text::new("R: Spawn Ragdoll | B: Spawn Bomb"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn spawn_ground(commands: &mut Commands) {
    let ground_thickness = 20.0;
    let ground_width = 2000.0;
    let ground_y = -300.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(ground_width, ground_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, ground_y, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(ground_width / 2.0, ground_thickness / 2.0),
    ));

    let wall_thickness = 20.0;
    let wall_height = 1000.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(wall_thickness, wall_height)),
            ..default()
        },
        Transform::from_xyz(-640.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(wall_thickness, wall_height)),
            ..default()
        },
        Transform::from_xyz(640.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
    ));
}

fn spawn_ragdoll_on_keypress(
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

fn spawn_bomb_on_keypress(
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

fn bomb_timer_system(
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

fn spawn_explosion_visuals(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.9, 0.3, 1.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 1.0),
        ExplosionFlash {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            max_scale: 15.0,
        },
    ));

    for i in 0..3 {
        let delay = i as f32 * 0.05;
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.6, 0.1, 0.8),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.5),
            ExplosionShockwave {
                timer: Timer::from_seconds(0.5 + delay, TimerMode::Once),
                max_radius: EXPLOSION_RADIUS * 1.2,
                start_radius: 20.0,
            },
        ));
    }
}

fn spawn_explosion_debris(commands: &mut Commands, position: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..30 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(150.0..500.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(3.0..12.0);
        let color = Color::srgb(
            rng.gen_range(0.8..1.0),
            rng.gen_range(0.3..0.7),
            rng.gen_range(0.0..0.2),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            RigidBody::Dynamic,
            Collider::ball(size / 2.0),
            Velocity::linear(velocity),
            ColliderMassProperties::Density(0.5),
            ExternalImpulse::default(),
            Debris,
        ));
    }

    for _ in 0..20 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..200.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(2.0..6.0);
        let color = Color::srgba(1.0, 0.8, 0.0, 1.0);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            RigidBody::Dynamic,
            Collider::ball(size / 2.0),
            Velocity::linear(velocity),
            ColliderMassProperties::Density(0.3),
            ExternalImpulse::default(),
            Debris,
        ));
    }
}

fn spawn_smoke_particles(commands: &mut Commands, position: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..40 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(30.0..120.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = rng.gen_range(8.0..20.0);
        let gray = rng.gen_range(0.2..0.5);
        let color = Color::srgba(gray, gray, gray, rng.gen_range(0.4..0.8));

        let offset = Vec2::new(
            rng.gen_range(-20.0..20.0),
            rng.gen_range(-20.0..20.0),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(position.x + offset.x, position.y + offset.y, -0.5),
            SmokeParticle {
                lifetime: Timer::from_seconds(rng.gen_range(1.0..2.5), TimerMode::Once),
                velocity: velocity + Vec2::new(0.0, rng.gen_range(30.0..80.0)),
            },
        ));
    }
}

fn apply_explosion(
    mut commands: Commands,
    explosion_query: Query<(Entity, &Explosion)>,
    mut physics_query: Query<(&Transform, &mut ExternalImpulse), With<RigidBody>>,
) {
    for (explosion_entity, explosion) in explosion_query.iter() {
        for (transform, mut impulse) in physics_query.iter_mut() {
            let pos = transform.translation.truncate();
            let delta = pos - explosion.position;
            let distance = delta.length();

            if distance < explosion.radius && distance > 0.1 {
                let direction = delta.normalize();
                let strength = (1.0 - distance / explosion.radius) * explosion.force;
                let force = direction * strength;

                impulse.impulse += force;

                let torque = rand::thread_rng().gen_range(-5000.0..5000.0)
                    * (1.0 - distance / explosion.radius);
                impulse.torque_impulse += torque;
            }
        }

        commands.entity(explosion_entity).despawn();
    }
}

fn cleanup_debris(
    mut commands: Commands,
    time: Res<Time>,
    mut debris_query: Query<(Entity, &Transform, &mut Sprite), With<Debris>>,
) {
    for (entity, transform, mut sprite) in debris_query.iter_mut() {
        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }

        let Srgba { red, green, blue, alpha } = sprite.color.to_srgba();
        let new_alpha = alpha - time.delta_secs() * 0.3;
        if new_alpha <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            sprite.color = Color::srgba(red, green, blue, new_alpha);
        }
    }
}

fn animate_explosion_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionFlash, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut flash, mut transform, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());
        
        let progress = flash.timer.fraction();
        let scale = 1.0 + (flash.max_scale - 1.0) * progress;
        transform.scale = Vec3::splat(scale);
        
        let alpha = 1.0 - progress;
        let Srgba { red, green, blue, .. } = sprite.color.to_srgba();
        sprite.color = Color::srgba(red, green, blue, alpha);
        
        if flash.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_explosion_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionShockwave, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut shockwave, mut transform, mut sprite) in query.iter_mut() {
        shockwave.timer.tick(time.delta());
        
        let progress = shockwave.timer.fraction();
        let current_radius = shockwave.start_radius 
            + (shockwave.max_radius - shockwave.start_radius) * progress;
        let thickness = 8.0 * (1.0 - progress);
        
        transform.scale = Vec3::new(current_radius / 5.0, current_radius / 5.0, 1.0);
        
        let alpha = (1.0 - progress) * 0.8;
        let brightness = 1.0 - progress * 0.5;
        sprite.color = Color::srgba(
            brightness * 1.0,
            brightness * 0.6,
            brightness * 0.1,
            alpha
        );
        
        if shockwave.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_smoke_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SmokeParticle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut smoke, mut transform, mut sprite) in query.iter_mut() {
        smoke.lifetime.tick(time.delta());
        
        transform.translation += smoke.velocity.extend(0.0) * time.delta_secs();
        
        smoke.velocity.y += 30.0 * time.delta_secs();
        smoke.velocity *= 0.98;
        
        let progress = smoke.lifetime.fraction();
        let scale = 1.0 + progress * 2.0;
        transform.scale = Vec3::splat(scale);
        
        let alpha = (1.0 - progress) * 0.6;
        let Srgba { red, green, blue, .. } = sprite.color.to_srgba();
        sprite.color = Color::srgba(red, green, blue, alpha);
        
        if smoke.lifetime.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
