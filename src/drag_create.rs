use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Flammable, Health};
use crate::connection::Connectable;
use crate::drag::{Draggable, DragState};
use crate::iron_block::IronBlock;
use crate::ui_topbar::{ObjectType, SelectedObject};
use crate::utils::get_cursor_world_position;
use crate::wooden_box::WoodenBox;

/// Resource to track the state of drag-to-create
#[derive(Resource, Default)]
pub struct CreateDragState {
    pub is_creating: bool,
    pub start_position: Vec2,
    pub preview_entity: Option<Entity>,
}

/// Component marker for preview entities
#[derive(Component)]
pub struct PreviewEntity;

const MIN_SIZE: f32 = 20.0;
const MAX_SIZE: f32 = 500.0;

/// Start drag-to-create when clicking on empty space
pub fn start_create_drag_system(
    mut create_drag_state: ResMut<CreateDragState>,
    drag_state: Res<DragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_object: Res<SelectedObject>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    // Only activate if not already dragging an existing object
    if mouse_button.just_pressed(MouseButton::Left) 
        && drag_state.dragging_entity.is_none() 
        && !create_drag_state.is_creating 
    {
        // Only create for WoodenBox and IronBlock types
        if matches!(selected_object.object_type, ObjectType::WoodenBox | ObjectType::IronBlock) {
            if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
                create_drag_state.is_creating = true;
                create_drag_state.start_position = world_pos;
                
                // Spawn initial preview entity
                let color = match selected_object.object_type {
                    ObjectType::WoodenBox => Color::srgba(0.6, 0.4, 0.2, 0.5),
                    ObjectType::IronBlock => Color::srgba(0.6, 0.6, 0.65, 0.5),
                    _ => Color::srgba(0.5, 0.5, 0.5, 0.5),
                };
                
                let preview = commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(MIN_SIZE, MIN_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
                    PreviewEntity,
                )).id();
                
                create_drag_state.preview_entity = Some(preview);
            }
        }
    }
}

/// Update preview entity size based on drag distance
pub fn update_create_drag_system(
    create_drag_state: Res<CreateDragState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut preview_query: Query<(&mut Transform, &mut Sprite), With<PreviewEntity>>,
) {
    if create_drag_state.is_creating {
        if let Some(preview_entity) = create_drag_state.preview_entity {
            if let Ok((mut transform, mut sprite)) = preview_query.get_mut(preview_entity) {
                if let Some(current_pos) = get_cursor_world_position(&windows, &camera_q) {
                    // Calculate the rectangle from start to current position
                    let start = create_drag_state.start_position;
                    let width = (current_pos.x - start.x).abs().max(MIN_SIZE).min(MAX_SIZE);
                    let height = (current_pos.y - start.y).abs().max(MIN_SIZE).min(MAX_SIZE);
                    
                    // Update sprite size
                    sprite.custom_size = Some(Vec2::new(width, height));
                    
                    // Center the preview between start and current position
                    let center_x = (start.x + current_pos.x) / 2.0;
                    let center_y = (start.y + current_pos.y) / 2.0;
                    transform.translation.x = center_x;
                    transform.translation.y = center_y;
                }
            }
        }
    }
}

/// End drag-to-create and spawn the actual entity
pub fn end_create_drag_system(
    mut create_drag_state: ResMut<CreateDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_object: Res<SelectedObject>,
    preview_query: Query<(&Transform, &Sprite), With<PreviewEntity>>,
    mut commands: Commands,
) {
    if mouse_button.just_released(MouseButton::Left) && create_drag_state.is_creating {
        // Get the final size from the preview entity
        if let Some(preview_entity) = create_drag_state.preview_entity {
            if let Ok((transform, sprite)) = preview_query.get(preview_entity) {
                let position = transform.translation.truncate();
                let size = sprite.custom_size.unwrap_or(Vec2::new(MIN_SIZE, MIN_SIZE));
                
                // Spawn the actual entity based on the selected type
                match selected_object.object_type {
                    ObjectType::WoodenBox => {
                        spawn_wooden_box_with_size(&mut commands, position, size);
                    }
                    ObjectType::IronBlock => {
                        spawn_iron_block_with_size(&mut commands, position, size);
                    }
                    _ => {}
                }
            }
            
            // Despawn the preview entity
            commands.entity(preview_entity).despawn();
        }
        
        // Reset state
        create_drag_state.is_creating = false;
        create_drag_state.preview_entity = None;
        create_drag_state.start_position = Vec2::ZERO;
    }
}

fn spawn_wooden_box_with_size(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        ColliderMassProperties::Density(0.8),
        Restitution::coefficient(0.3),
        Friction::coefficient(0.7),
        ExternalImpulse::default(),
        Velocity::default(),
        WoodenBox,
        Draggable,
        Connectable,
        Health {
            current: 100.0 * (size.x * size.y) / (60.0 * 60.0), // Scale health with size
            max: 100.0 * (size.x * size.y) / (60.0 * 60.0),
        },
        Flammable {
            ignition_threshold: 0.5,
        },
    ));
}

fn spawn_iron_block_with_size(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.6, 0.65),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        ColliderMassProperties::Density(7.8),
        Restitution::coefficient(0.2),
        Friction::coefficient(0.5),
        ExternalImpulse::default(),
        Velocity::default(),
        IronBlock,
        Draggable,
        Connectable,
    ));
}
