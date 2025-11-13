use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::utils::get_cursor_world_position;

#[derive(Component)]
pub struct Draggable;

#[derive(Resource, Default)]
pub struct DragState {
    pub dragging_entity: Option<Entity>,
    pub original_body_type: Option<RigidBody>,
    pub drag_offset: Vec2,
}

pub fn start_drag_system(
    mut drag_state: ResMut<DragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    draggable_query: Query<(Entity, &Transform, &RigidBody), With<Draggable>>,
    rapier_context: Query<&RapierContext>,
    selection_state: Res<crate::connection::SelectionState>,
) {
    // Don't start drag if connection mode is enabled
    if selection_state.is_enabled {
        return;
    }

    let Ok(context) = rapier_context.get_single() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) && drag_state.dragging_entity.is_none() {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            // Use raycast to detect the object under cursor
            let filter = QueryFilter::default();
            
            // Use point intersection to find entities at cursor position
            if let Some((entity, _toi)) = context.cast_ray(
                world_pos,
                Vec2::new(0.0, -1.0), // Direction doesn't matter for point detection
                0.1, // Very small distance, essentially a point query
                true,
                filter,
            ) {
                // Check if the hit entity is draggable
                if let Ok((entity, transform, body)) = draggable_query.get(entity) {
                    let object_pos = transform.translation.truncate();
                    let offset = object_pos - world_pos;
                    drag_state.dragging_entity = Some(entity);
                    drag_state.original_body_type = Some(*body);
                    drag_state.drag_offset = offset;
                }
            }
        }
    }
}

pub fn update_drag_system(
    drag_state: Res<DragState>,
    mut draggable_query: Query<(&mut Transform, &mut RigidBody, &mut Velocity), With<Draggable>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(entity) = drag_state.dragging_entity {
        if let Ok((mut transform, mut body, mut velocity)) = draggable_query.get_mut(entity) {
            if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
                *body = RigidBody::KinematicPositionBased;

                let target_pos = world_pos + drag_state.drag_offset;
                transform.translation.x = target_pos.x;
                transform.translation.y = target_pos.y;

                velocity.linvel = Vec2::ZERO;
                velocity.angvel = 0.0;
            }
        }
    }
}

pub fn end_drag_system(
    mut drag_state: ResMut<DragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut draggable_query: Query<&mut RigidBody, With<Draggable>>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        if let Some(entity) = drag_state.dragging_entity {
            if let Some(original_body) = drag_state.original_body_type {
                if let Ok(mut body) = draggable_query.get_mut(entity) {
                    *body = original_body;
                }
            }

            drag_state.dragging_entity = None;
            drag_state.original_body_type = None;
            drag_state.drag_offset = Vec2::ZERO;
        }
    }
}
