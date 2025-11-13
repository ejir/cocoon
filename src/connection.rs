use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::get_cursor_world_position;
use crate::components::{Connection, ConnectionKind};

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
    
    /// Get the color for visual representation
    pub fn color(&self) -> Color {
        match self {
            ConnectionMaterial::Wood => Color::srgb(0.6, 0.4, 0.2),      // Brown
            ConnectionMaterial::Metal => Color::srgb(0.7, 0.7, 0.8),     // Gray/Silver
            ConnectionMaterial::Rope => Color::srgb(0.8, 0.7, 0.5),      // Tan
            ConnectionMaterial::Plastic => Color::srgb(0.3, 0.6, 0.9),   // Blue
        }
    }
    
    /// Get the thickness for visual representation
    pub fn thickness(&self) -> f32 {
        match self {
            ConnectionMaterial::Wood => 4.0,
            ConnectionMaterial::Metal => 3.0,
            ConnectionMaterial::Rope => 2.0,
            ConnectionMaterial::Plastic => 3.5,
        }
    }
    
    /// Get the break force threshold for this material
    pub fn break_force(&self) -> f32 {
        match self {
            ConnectionMaterial::Wood => 5000.0,
            ConnectionMaterial::Metal => 15000.0,
            ConnectionMaterial::Rope => 2000.0,
            ConnectionMaterial::Plastic => 7000.0,
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

// Connection mode removed - only drag mode is supported now

#[derive(Resource)]
pub struct SelectionState {
    pub constraint_type: ConstraintType,
    pub material: ConnectionMaterial,
    pub is_enabled: bool,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            constraint_type: ConstraintType::Fixed,
            material: ConnectionMaterial::Metal,
            is_enabled: false,
        }
    }
}

/// Resource to track drag-based connection state
#[derive(Resource, Default)]
pub struct DragConnectionState {
    pub is_dragging: bool,
    pub start_entity: Option<Entity>,
    pub start_position: Vec2,
}

/// Component for the visual line showing the connection being dragged
#[derive(Component)]
pub struct ConnectionDragLine;

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

/// Component for visual connection line between connected objects
#[derive(Component)]
pub struct ConnectionVisual {
    pub entity1: Entity,
    pub entity2: Entity,
    pub anchor1: Vec2,
    pub anchor2: Vec2,
    pub material: ConnectionMaterial,
}

// Click mode removed - only drag mode is supported

// create_constraint_system removed - only drag mode is supported

// handle_deleted_selections removed - not needed for drag mode

/// Update hover indicator to highlight connectable objects under cursor
pub fn update_hover_indicator(
    mut commands: Commands,
    selection_state: Res<SelectionState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    hover_query: Query<Entity, With<HoverIndicator>>,
    drag_conn_state: Res<DragConnectionState>,
    rapier_context: Query<&RapierContext>,
) {
    // Only show hover indicator when in connect mode and not currently dragging
    if !selection_state.is_enabled || drag_conn_state.is_dragging {
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

// ========== Drag-based Connection Systems ==========

/// Start dragging a connection from a connectable object
pub fn start_drag_connection(
    mut commands: Commands,
    mut drag_conn_state: ResMut<DragConnectionState>,
    selection_state: Res<SelectionState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    connectable_query: Query<(Entity, &Transform), With<Connectable>>,
    drag_state: Res<crate::drag::DragState>,
    rapier_context: Query<&RapierContext>,
) {
    // Only work when connection mode is enabled
    if !selection_state.is_enabled {
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

/// Update the visual line while dragging connection
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

/// End drag connection and create constraint if over another object
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

                        let material = selection_state.material;

                        // Calculate anchors on the connected objects
                        let anchor_on_start = start_click_pos - start_body_pos;
                        let anchor_on_end = end_click_pos - end_body_pos;

                        let break_force = material.break_force();
                        let connection_kind = match selection_state.constraint_type {
                            ConstraintType::Fixed => ConnectionKind::Fixed,
                            ConstraintType::Hinge => ConnectionKind::Hinge,
                        };

                        // Create appropriate joint type based on constraint type
                        let joint = match selection_state.constraint_type {
                            ConstraintType::Fixed => {
                                // Fixed joint: non-rotatable, rigid connection like a nail or weld
                                let fixed_joint = FixedJointBuilder::new()
                                    .local_anchor1(anchor_on_start)
                                    .local_anchor2(anchor_on_end);
                                
                                ImpulseJoint::new(end_entity, fixed_joint)
                            }
                            ConstraintType::Hinge => {
                                // Revolute joint: rotatable connection like a bearing or hinge
                                // Apply material damping for rotational friction
                                let revolute_joint = RevoluteJointBuilder::new()
                                    .local_anchor1(anchor_on_start)
                                    .local_anchor2(anchor_on_end)
                                    .motor_model(MotorModel::ForceBased)
                                    .motor_max_force(material.damping() * 100.0);
                                
                                ImpulseJoint::new(end_entity, revolute_joint)
                            }
                        };

                        commands.entity(start_entity).with_children(|parent| {
                            parent.spawn((
                                joint,
                                UserCreatedJoint,
                                JointMaterial(material),
                                Connection {
                                    a: start_entity,
                                    b: end_entity,
                                    anchor_a: anchor_on_start,
                                    anchor_b: anchor_on_end,
                                    kind: connection_kind,
                                    break_force,
                                    current_force: 0.0,
                                },
                                ConnectionVisual {
                                    entity1: start_entity,
                                    entity2: end_entity,
                                    anchor1: anchor_on_start,
                                    anchor2: anchor_on_end,
                                    material,
                                },
                            ));
                        });
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
    }
}


fn spawn_connection_drag_line(commands: &mut Commands) {
    commands.spawn(ConnectionDragLine);
}

/// System to despawn joint entities if one of the connected bodies is despawned
pub fn handle_despawned_connected_entities(
    mut commands: Commands,
    joint_query: Query<(Entity, &Connection)>,
    transform_query: Query<&Transform>, // Used to check for existence
) {
    for (joint_entity, connection) in joint_query.iter() {
        if transform_query.get(connection.a).is_err() || transform_query.get(connection.b).is_err() {
            // One of the connected entities is despawned, so despawn the joint entity
            if let Some(mut entity_commands) = commands.get_entity(joint_entity) {
                entity_commands.despawn();
            }
        }
    }
}

/// System to update the visual representation of connections (the lines)
pub fn update_connection_visuals(
    mut gizmos: Gizmos,
    visual_query: Query<&ConnectionVisual>,
    transform_query: Query<&Transform>,
) {
    for visual in visual_query.iter() {
        if let (Ok(transform1), Ok(transform2)) = (
            transform_query.get(visual.entity1),
            transform_query.get(visual.entity2),
        ) {
            let start_pos = transform1.translation.truncate() + (transform1.rotation * visual.anchor1.extend(0.0)).truncate();
            let end_pos = transform2.translation.truncate() + (transform2.rotation * visual.anchor2.extend(0.0)).truncate();
            
            gizmos.line_2d(start_pos, end_pos, visual.material.color());
        }
    }
}

/// System to check for and break joints that exceed their force limit
pub fn break_joints_on_force_limit(
    mut commands: Commands,
    joint_query: Query<(Entity, &ImpulseJoint, &Connection)>,
    _rapier_context: Query<&RapierContext>,
) {
    for (entity, _joint, connection) in joint_query.iter() {
        if connection.current_force > connection.break_force {
            commands.entity(entity).despawn_recursive();
        }
    }
}
