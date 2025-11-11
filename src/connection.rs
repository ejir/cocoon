use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::get_cursor_world_position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintType {
    Fixed,
    Hinge,
    Spring,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionMode {
    Click,  // Mode 1: Click first, click second, press C to connect
    Drag,   // Mode 2: Drag from first to second to connect
}

#[derive(Resource)]
pub struct SelectionState {
    pub first_selected: Option<Entity>,
    pub second_selected: Option<Entity>,
    pub constraint_type: ConstraintType,
    pub is_enabled: bool,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            first_selected: None,
            second_selected: None,
            constraint_type: ConstraintType::Fixed,
            is_enabled: false,
        }
    }
}

/// Resource to track drag-based connection state (Mode 2)
#[derive(Resource, Default)]
pub struct DragConnectionState {
    pub is_dragging: bool,
    pub start_entity: Option<Entity>,
    pub start_position: Vec2,
}

/// Component for the visual line showing the connection being dragged
#[derive(Component)]
pub struct ConnectionDragLine;

#[derive(Component)]
pub struct SelectionIndicator {
    pub target_entity: Entity,
    pub is_first: bool,
}

/// Component for the hover indicator that shows when mouse is over a connectable object
#[derive(Component)]
pub struct HoverIndicator {
    pub target_entity: Entity,
}

#[derive(Component)]
pub struct Connectable;

#[derive(Component)]
pub struct UserCreatedJoint;

pub fn handle_object_selection(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    drag_state: Res<crate::drag::DragState>,
    drag_conn_state: Res<DragConnectionState>,
) {
    if !selection_state.is_enabled {
        return;
    }

    // Don't handle selection if dragging or drag-connecting
    if drag_state.dragging_entity.is_some() || drag_conn_state.is_dragging {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            let mut closest_entity = None;
            let mut closest_distance = f32::INFINITY;

            for (entity, transform) in connectable_query.iter() {
                let object_pos = transform.translation.truncate();
                let distance = object_pos.distance(world_pos);

                let max_radius = 50.0;

                if distance < max_radius && distance < closest_distance {
                    closest_distance = distance;
                    closest_entity = Some(entity);
                }
            }

            if let Some(entity) = closest_entity {
                if selection_state.first_selected.is_none() {
                    selection_state.first_selected = Some(entity);
                    spawn_selection_indicator(&mut commands, entity, true);
                } else if selection_state.first_selected == Some(entity) {
                    clear_selection(&mut commands, &mut selection_state, &indicator_query);
                } else if selection_state.second_selected.is_none() {
                    if selection_state.first_selected != Some(entity) {
                        selection_state.second_selected = Some(entity);
                        spawn_selection_indicator(&mut commands, entity, false);
                    }
                } else {
                    clear_selection(&mut commands, &mut selection_state, &indicator_query);
                    selection_state.first_selected = Some(entity);
                    spawn_selection_indicator(&mut commands, entity, true);
                }
            }
        }
    }
}

fn spawn_selection_indicator(commands: &mut Commands, target_entity: Entity, is_first: bool) {
    let color = if is_first {
        Color::srgba(0.0, 1.0, 0.0, 0.6)
    } else {
        Color::srgba(0.0, 0.5, 1.0, 0.6)
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        SelectionIndicator {
            target_entity,
            is_first,
        },
    ));
}

fn clear_selection(
    commands: &mut Commands,
    selection_state: &mut SelectionState,
    indicator_query: &Query<Entity, With<SelectionIndicator>>,
) {
    selection_state.first_selected = None;
    selection_state.second_selected = None;

    for entity in indicator_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_selection_indicators(
    mut indicator_query: Query<(&mut Transform, &SelectionIndicator)>,
    transform_query: Query<&Transform, Without<SelectionIndicator>>,
) {
    for (mut indicator_transform, indicator) in indicator_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(indicator.target_entity) {
            indicator_transform.translation = target_transform.translation + Vec3::new(0.0, 0.0, 1.0);
            indicator_transform.rotation = target_transform.rotation;
        }
    }
}

pub fn create_constraint_system(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    transform_query: Query<&Transform>,
) {
    if !selection_state.is_enabled {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyC) || keyboard.just_pressed(KeyCode::Enter) {
        if let (Some(first), Some(second)) = (selection_state.first_selected, selection_state.second_selected) {
            if let (Ok(first_transform), Ok(second_transform)) = (
                transform_query.get(first),
                transform_query.get(second),
            ) {
                let first_pos = first_transform.translation.truncate();
                let second_pos = second_transform.translation.truncate();

                let midpoint = (first_pos + second_pos) / 2.0;
                let anchor1 = midpoint - first_pos;
                let anchor2 = midpoint - second_pos;

                match selection_state.constraint_type {
                    ConstraintType::Fixed => {
                        let joint = FixedJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                        ));
                    }
                    ConstraintType::Hinge => {
                        let joint = RevoluteJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                        ));
                    }
                    ConstraintType::Spring => {
                        let rest_length = (first_pos - second_pos).length();
                        let joint = SpringJointBuilder::new(rest_length, 100.0, 5.0)
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                        ));
                    }
                }

                for entity in indicator_query.iter() {
                    commands.entity(entity).despawn();
                }

                selection_state.first_selected = None;
                selection_state.second_selected = None;
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        clear_selection(&mut commands, &mut selection_state, &indicator_query);
    }
}

pub fn handle_deleted_selections(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    query: Query<Entity, With<Connectable>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
) {
    let mut should_clear = false;

    if let Some(first) = selection_state.first_selected {
        if query.get(first).is_err() {
            should_clear = true;
        }
    }

    if let Some(second) = selection_state.second_selected {
        if query.get(second).is_err() {
            should_clear = true;
        }
    }

    if should_clear {
        clear_selection(&mut commands, &mut selection_state, &indicator_query);
    }
}

/// Update hover indicator to highlight connectable objects under cursor
pub fn update_hover_indicator(
    mut commands: Commands,
    selection_state: Res<SelectionState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    hover_query: Query<Entity, With<HoverIndicator>>,
    drag_conn_state: Res<DragConnectionState>,
) {
    // Only show hover indicator when in connect mode and not currently dragging
    if !selection_state.is_enabled || drag_conn_state.is_dragging {
        // Remove any existing hover indicators
        for entity in hover_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
        let mut closest_entity = None;
        let mut closest_distance = f32::INFINITY;

        for (entity, transform) in connectable_query.iter() {
            let object_pos = transform.translation.truncate();
            let distance = object_pos.distance(world_pos);

            let max_radius = 50.0;

            if distance < max_radius && distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(entity);
            }
        }

        // Remove old hover indicator
        for entity in hover_query.iter() {
            commands.entity(entity).despawn();
        }

        // Spawn new hover indicator if hovering over an object
        if let Some(entity) = closest_entity {
            spawn_hover_indicator(&mut commands, entity);
        }
    } else {
        // Cursor not in window, remove hover indicators
        for entity in hover_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_hover_indicator(commands: &mut Commands, target_entity: Entity) {
    let color = Color::srgba(1.0, 1.0, 0.3, 0.3); // Yellow with low alpha

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(65.0, 65.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.5),
        HoverIndicator { target_entity },
    ));
}

/// Update hover indicator position to follow target entity
pub fn update_hover_indicator_position(
    mut indicator_query: Query<(&mut Transform, &HoverIndicator)>,
    transform_query: Query<&Transform, Without<HoverIndicator>>,
) {
    for (mut indicator_transform, indicator) in indicator_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(indicator.target_entity) {
            indicator_transform.translation = target_transform.translation + Vec3::new(0.0, 0.0, 0.5);
            indicator_transform.rotation = target_transform.rotation;
        }
    }
}

// ========== Mode 2: Drag-based Connection Systems ==========

/// Start dragging a connection from a connectable object (Mode 2)
pub fn start_drag_connection(
    mut commands: Commands,
    mut drag_conn_state: ResMut<DragConnectionState>,
    selection_state: Res<SelectionState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    drag_state: Res<crate::drag::DragState>,
) {
    // Only work when connection mode is enabled
    if !selection_state.is_enabled {
        return;
    }

    // Don't start if already dragging an object or dragging a connection
    if drag_state.dragging_entity.is_some() || drag_conn_state.is_dragging {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            let mut closest_entity = None;
            let mut closest_distance = f32::INFINITY;

            for (entity, transform) in connectable_query.iter() {
                let object_pos = transform.translation.truncate();
                let distance = object_pos.distance(world_pos);

                let max_radius = 50.0;

                if distance < max_radius && distance < closest_distance {
                    closest_distance = distance;
                    closest_entity = Some((entity, object_pos));
                }
            }

            if let Some((entity, pos)) = closest_entity {
                // Start dragging connection from this entity
                drag_conn_state.is_dragging = true;
                drag_conn_state.start_entity = Some(entity);
                drag_conn_state.start_position = pos;

                // Spawn visual line
                spawn_connection_drag_line(&mut commands);
            }
        }
    }
}

/// Update the visual line while dragging connection (Mode 2)
pub fn update_drag_connection(
    drag_conn_state: Res<DragConnectionState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
    transform_query: Query<&Transform>,
) {
    if !drag_conn_state.is_dragging {
        return;
    }

    if let Some(start_entity) = drag_conn_state.start_entity {
        if let Ok(start_transform) = transform_query.get(start_entity) {
            let start_pos = start_transform.translation.truncate();
            
            if let Some(cursor_pos) = get_cursor_world_position(&windows, &camera_q) {
                // Draw a line from start entity to cursor
                gizmos.line_2d(start_pos, cursor_pos, Color::srgb(0.2, 0.8, 0.2));
                
                // Draw a circle at the start point
                gizmos.circle_2d(start_pos, 8.0, Color::srgb(0.0, 1.0, 0.0));
                
                // Draw a circle at the cursor
                gizmos.circle_2d(cursor_pos, 8.0, Color::srgb(0.2, 0.8, 0.2));
            }
        }
    }
}

/// End drag connection and create constraint if over another object (Mode 2)
pub fn end_drag_connection(
    mut commands: Commands,
    mut drag_conn_state: ResMut<DragConnectionState>,
    selection_state: Res<SelectionState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    transform_query: Query<&Transform>,
    line_query: Query<Entity, With<ConnectionDragLine>>,
) {
    if !drag_conn_state.is_dragging {
        return;
    }

    if mouse_button.just_released(MouseButton::Left) {
        let mut connection_created = false;

        if let Some(start_entity) = drag_conn_state.start_entity {
            if let Some(cursor_pos) = get_cursor_world_position(&windows, &camera_q) {
                // Find if cursor is over another connectable object
                let mut target_entity = None;
                let mut closest_distance = f32::INFINITY;

                for (entity, transform) in connectable_query.iter() {
                    if entity == start_entity {
                        continue; // Skip the start entity
                    }

                    let object_pos = transform.translation.truncate();
                    let distance = object_pos.distance(cursor_pos);

                    let max_radius = 50.0;

                    if distance < max_radius && distance < closest_distance {
                        closest_distance = distance;
                        target_entity = Some(entity);
                    }
                }

                // If we found a target, create the connection
                if let Some(end_entity) = target_entity {
                    if let (Ok(start_transform), Ok(end_transform)) = (
                        transform_query.get(start_entity),
                        transform_query.get(end_entity),
                    ) {
                        let start_pos = start_transform.translation.truncate();
                        let end_pos = end_transform.translation.truncate();

                        let midpoint = (start_pos + end_pos) / 2.0;
                        let anchor1 = midpoint - start_pos;
                        let anchor2 = midpoint - end_pos;

                        match selection_state.constraint_type {
                            ConstraintType::Fixed => {
                                let joint = FixedJointBuilder::new()
                                    .local_anchor1(anchor1)
                                    .local_anchor2(anchor2);

                                commands.entity(end_entity).insert((
                                    ImpulseJoint::new(start_entity, joint),
                                    UserCreatedJoint,
                                ));
                            }
                            ConstraintType::Hinge => {
                                let joint = RevoluteJointBuilder::new()
                                    .local_anchor1(anchor1)
                                    .local_anchor2(anchor2);

                                commands.entity(end_entity).insert((
                                    ImpulseJoint::new(start_entity, joint),
                                    UserCreatedJoint,
                                ));
                            }
                            ConstraintType::Spring => {
                                let rest_length = (start_pos - end_pos).length();
                                let joint = SpringJointBuilder::new(rest_length, 100.0, 5.0)
                                    .local_anchor1(anchor1)
                                    .local_anchor2(anchor2);

                                commands.entity(end_entity).insert((
                                    ImpulseJoint::new(start_entity, joint),
                                    UserCreatedJoint,
                                ));
                            }
                        }

                        connection_created = true;
                    }
                }
            }
        }

        // Clean up drag line
        for entity in line_query.iter() {
            commands.entity(entity).despawn();
        }

        // Reset drag connection state
        drag_conn_state.is_dragging = false;
        drag_conn_state.start_entity = None;
        drag_conn_state.start_position = Vec2::ZERO;
    }
}

fn spawn_connection_drag_line(commands: &mut Commands) {
    // This is just a marker component - the actual line is drawn using Gizmos
    commands.spawn(ConnectionDragLine);
}
