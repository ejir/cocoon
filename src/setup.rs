use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup(mut commands: Commands) {
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
