use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
) {
    if mouse_button.just_pressed(MouseButton::Left) && drag_state.dragging_entity.is_none() {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let mut closest_entity = None;
                let mut closest_distance = f32::INFINITY;

                for (entity, transform, body) in draggable_query.iter() {
                    let object_pos = transform.translation.truncate();
                    let distance = object_pos.distance(world_pos);
                    
                    let max_radius = 50.0;

                    if distance < max_radius && distance < closest_distance {
                        closest_distance = distance;
                        closest_entity = Some((entity, object_pos, *body));
                    }
                }

                if let Some((entity, object_pos, body)) = closest_entity {
                    let offset = object_pos - world_pos;
                    drag_state.dragging_entity = Some(entity);
                    drag_state.original_body_type = Some(body);
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
            let window = windows.single();
            let (camera, camera_transform) = camera_q.single();

            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
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
