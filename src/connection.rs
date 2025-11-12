use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::get_cursor_world_position;

/// Material type for connections, affecting joint strength and behavior
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionMaterial {
    Wood,      // Weak, moderate flexibility
    Metal,     // Strong, rigid
    Rope,      // Weak, high flexibility
    Plastic,   // Moderate strength and flexibility
}

impl ConnectionMaterial {
    /// Get the compliance (inverse stiffness) for this material
    pub fn compliance(&self) -> f32 {
        match self {
            ConnectionMaterial::Wood => 0.00001,
            ConnectionMaterial::Metal => 0.000001,
            ConnectionMaterial::Rope => 0.0001,
            ConnectionMaterial::Plastic => 0.00005,
        }
    }
    
    /// Get the damping coefficient for this material
    pub fn damping(&self) -> f32 {
        match self {
            ConnectionMaterial::Wood => 0.5,
            ConnectionMaterial::Metal => 0.1,
            ConnectionMaterial::Rope => 2.0,
            ConnectionMaterial::Plastic => 1.0,
        }
    }
}

impl Default for ConnectionMaterial {
    fn default() -> Self {
        ConnectionMaterial::Metal
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintType {
    Fixed,  // Non-rotatable, like a nail
    Hinge,  // Rotatable, like a bearing
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionMode {
    Click,  // Mode 1: Click first, click second, press C to connect
    Drag,   // Mode 2: Drag from first to second to connect
}

#[derive(Resource)]
pub struct ConnectionModeState {
    pub mode: ConnectionMode,
}

impl Default for ConnectionModeState {
    fn default() -> Self {
        Self {
            mode: ConnectionMode::Drag,
        }
    }
}

#[derive(Resource)]
pub struct SelectionState {
    pub first_selected: Option<Entity>,
    pub second_selected: Option<Entity>,
    pub first_click_position: Option<Vec2>,
    pub second_click_position: Option<Vec2>,
    pub constraint_type: ConstraintType,
    pub material: ConnectionMaterial,
    pub is_enabled: bool,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            first_selected: None,
            second_selected: None,
            first_click_position: None,
            second_click_position: None,
            constraint_type: ConstraintType::Fixed,
            material: ConnectionMaterial::Metal,
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

/// Component to store the material of a connection
#[derive(Component, Clone, Copy)]
pub struct JointMaterial(pub ConnectionMaterial);

pub fn handle_object_selection(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    connection_mode: Res<ConnectionModeState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    drag_state: Res<crate::drag::DragState>,
    drag_conn_state: Res<DragConnectionState>,
    rapier_context: Query<&RapierContext>,
) {
    if !selection_state.is_enabled {
        return;
    }

    // Only handle selection in Click mode
    if connection_mode.mode != ConnectionMode::Click {
        return;
    }

    // Don't handle selection if dragging or drag-connecting
    if drag_state.dragging_entity.is_some() || drag_conn_state.is_dragging {
        return;
    }

    let Ok(context) = rapier_context.get_single() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            // Use raycast to detect the object under cursor
            let filter = QueryFilter::default();
            
            if let Some((entity, _toi)) = context.cast_ray(
                world_pos,
                Vec2::new(0.0, -1.0),
                0.1,
                true,
                filter,
            ) {
                // Check if the hit entity is connectable
                if connectable_query.get(entity).is_ok() {
                    if selection_state.first_selected.is_none() {
                        selection_state.first_selected = Some(entity);
                        selection_state.first_click_position = Some(world_pos);
                        spawn_selection_indicator(&mut commands, entity, true);
                    } else if selection_state.first_selected == Some(entity) {
                        clear_selection(&mut commands, &mut selection_state, &indicator_query);
                    } else if selection_state.second_selected.is_none() {
                        if selection_state.first_selected != Some(entity) {
                            selection_state.second_selected = Some(entity);
                            selection_state.second_click_position = Some(world_pos);
                            spawn_selection_indicator(&mut commands, entity, false);
                        }
                    } else {
                        clear_selection(&mut commands, &mut selection_state, &indicator_query);
                        selection_state.first_selected = Some(entity);
                        selection_state.first_click_position = Some(world_pos);
                        spawn_selection_indicator(&mut commands, entity, true);
                    }
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
    selection_state.first_click_position = None;
    selection_state.second_click_position = None;

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
    connection_mode: Res<ConnectionModeState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
    transform_query: Query<&Transform>,
) {
    if !selection_state.is_enabled {
        return;
    }

    // Only work in Click mode
    if connection_mode.mode != ConnectionMode::Click {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyC) || keyboard.just_pressed(KeyCode::Enter) {
        if let (Some(first), Some(second), Some(first_click_pos), Some(second_click_pos)) = (
            selection_state.first_selected,
            selection_state.second_selected,
            selection_state.first_click_position,
            selection_state.second_click_position,
        ) {
            if let (Ok(first_transform), Ok(second_transform)) = (
                transform_query.get(first),
                transform_query.get(second),
            ) {
                let first_pos = first_transform.translation.truncate();
                let second_pos = second_transform.translation.truncate();

                let anchor1 = first_click_pos - first_pos;
                let anchor2 = second_click_pos - second_pos;

                let material = selection_state.material;
                let compliance = material.compliance();
                let damping = material.damping();

                match selection_state.constraint_type {
                    ConstraintType::Fixed => {
                        let joint = FixedJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                            JointMaterial(material),
                        ));
                    }
                    ConstraintType::Hinge => {
                        let joint = RevoluteJointBuilder::new()
                            .local_anchor1(anchor1)
                            .local_anchor2(anchor2);

                        commands.entity(second).insert((
                            ImpulseJoint::new(first, joint),
                            UserCreatedJoint,
                            JointMaterial(material),
                        ));
                    }
                }

                for entity in indicator_query.iter() {
                    commands.entity(entity).despawn();
                }

                selection_state.first_selected = None;
                selection_state.second_selected = None;
                selection_state.first_click_position = None;
                selection_state.second_click_position = None;
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
    connection_mode: Res<ConnectionModeState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    hover_query: Query<Entity, With<HoverIndicator>>,
    drag_conn_state: Res<DragConnectionState>,
    rapier_context: Query<&RapierContext>,
) {
    // Only show hover indicator when in connect mode (Drag mode) and not currently dragging
    if !selection_state.is_enabled || connection_mode.mode != ConnectionMode::Drag || drag_conn_state.is_dragging {
        // Remove any existing hover indicators
        for entity in hover_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    let Ok(context) = rapier_context.get_single() else {
        return;
    };

    if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
        // Use raycast to detect the object under cursor
        let filter = QueryFilter::default();
        let mut hover_entity = None;
        
        if let Some((entity, _toi)) = context.cast_ray(
            world_pos,
            Vec2::new(0.0, -1.0),
            0.1,
            true,
            filter,
        ) {
            // Check if the hit entity is connectable
            if connectable_query.get(entity).is_ok() {
                hover_entity = Some(entity);
            }
        }

        // Remove old hover indicator
        for entity in hover_query.iter() {
            commands.entity(entity).despawn();
        }

        // Spawn new hover indicator if hovering over an object
        if let Some(entity) = hover_entity {
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
    connection_mode: Res<ConnectionModeState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    drag_state: Res<crate::drag::DragState>,
    rapier_context: Query<&RapierContext>,
) {
    // Only work when connection mode is enabled and in Drag mode
    if !selection_state.is_enabled {
        return;
    }

    if connection_mode.mode != ConnectionMode::Drag {
        return;
    }

    // Don't start if already dragging an object or dragging a connection
    if drag_state.dragging_entity.is_some() || drag_conn_state.is_dragging {
        return;
    }

    let Ok(context) = rapier_context.get_single() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
            // Use raycast to detect the object under cursor
            let filter = QueryFilter::default();
            
            if let Some((entity, _toi)) = context.cast_ray(
                world_pos,
                Vec2::new(0.0, -1.0),
                0.1,
                true,
                filter,
            ) {
                // Check if the hit entity is connectable
                if let Ok((entity, _transform)) = connectable_query.get(entity) {
                    // Start dragging connection from this entity
                    drag_conn_state.is_dragging = true;
                    drag_conn_state.start_entity = Some(entity);
                    drag_conn_state.start_position = world_pos;

                    // Spawn visual line
                    spawn_connection_drag_line(&mut commands);
                }
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
) {
    if !drag_conn_state.is_dragging {
        return;
    }

    if let Some(_start_entity) = drag_conn_state.start_entity {
        let start_pos = drag_conn_state.start_position;
        
        if let Some(cursor_pos) = get_cursor_world_position(&windows, &camera_q) {
            // Draw a line from start position to cursor
            gizmos.line_2d(start_pos, cursor_pos, Color::srgb(0.2, 0.8, 0.2));
            
            // Draw a circle at the start point
            gizmos.circle_2d(start_pos, 8.0, Color::srgb(0.0, 1.0, 0.0));
            
            // Draw a circle at the cursor
            gizmos.circle_2d(cursor_pos, 8.0, Color::srgb(0.2, 0.8, 0.2));
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
    rapier_context: Query<&RapierContext>,
) {
    if !drag_conn_state.is_dragging {
        return;
    }

    let Ok(context) = rapier_context.get_single() else {
        return;
    };

    if mouse_button.just_released(MouseButton::Left) {
        let mut connection_created = false;

        if let Some(start_entity) = drag_conn_state.start_entity {
            if let Some(cursor_pos) = get_cursor_world_position(&windows, &camera_q) {
                // Use raycast to detect the object under cursor
                let filter = QueryFilter::default();
                let mut target_entity = None;
                
                if let Some((entity, _toi)) = context.cast_ray(
                    cursor_pos,
                    Vec2::new(0.0, -1.0),
                    0.1,
                    true,
                    filter,
                ) {
                    // Check if the hit entity is connectable and not the start entity
                    if entity != start_entity && connectable_query.get(entity).is_ok() {
                        target_entity = Some(entity);
                    }
                }

                // If we found a target, create the connection
                if let Some(end_entity) = target_entity {
                    if let (Ok(start_transform), Ok(end_transform)) = (
                        transform_query.get(start_entity),
                        transform_query.get(end_entity),
                    ) {
                        let start_body_pos = start_transform.translation.truncate();
                        let end_body_pos = end_transform.translation.truncate();

                        let start_click_pos = drag_conn_state.start_position;
                        let end_click_pos = cursor_pos;

                        let anchor1 = start_click_pos - start_body_pos;
                        let anchor2 = end_click_pos - end_body_pos;

                        let material = selection_state.material;
                        let compliance = material.compliance();
                        let damping = material.damping();

                        match selection_state.constraint_type {
                            ConstraintType::Fixed => {
                                let joint = FixedJointBuilder::new()
                                    .local_anchor1(anchor1)
                                    .local_anchor2(anchor2);

                                commands.entity(end_entity).insert((
                                    ImpulseJoint::new(start_entity, joint),
                                    UserCreatedJoint,
                                    JointMaterial(material),
                                ));
                            }
                            ConstraintType::Hinge => {
                                let joint = RevoluteJointBuilder::new()
                                    .local_anchor1(anchor1)
                                    .local_anchor2(anchor2);

                                commands.entity(end_entity).insert((
                                    ImpulseJoint::new(start_entity, joint),
                                    UserCreatedJoint,
                                    JointMaterial(material),
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

/// Clear selections when switching between connection modes
pub fn clear_selections_on_mode_change(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    connection_mode: Res<ConnectionModeState>,
    indicator_query: Query<Entity, With<SelectionIndicator>>,
) {
    if connection_mode.is_changed() {
        // Clear click-mode selections when switching modes
        for entity in indicator_query.iter() {
            commands.entity(entity).despawn();
        }
        selection_state.first_selected = None;
        selection_state.second_selected = None;
        selection_state.first_click_position = None;
        selection_state.second_click_position = None;
    }
}
