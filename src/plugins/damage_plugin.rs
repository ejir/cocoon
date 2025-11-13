//! Plugin for damage and connection systems

use bevy::prelude::*;
use crate::systems::damage::{
    apply_explosive_joint_damage, break_joints_on_force_limit, check_joint_damage,
    collision_joint_damage, detect_impact_damage, end_drag_connection,
    handle_despawned_connected_entities, start_drag_connection, track_velocity,
    update_connection_visuals, update_drag_connection, update_hover_indicator,
    update_hover_indicator_position, visualize_fractures, DragConnectionState, SelectionState,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .init_resource::<DragConnectionState>()
            .add_systems(
                Update,
                (
                    check_joint_damage,
                    apply_explosive_joint_damage,
                    detect_impact_damage,
                    collision_joint_damage,
                    track_velocity,
                    visualize_fractures,
                ),
            )
            .add_systems(
                Update,
                (
                    update_hover_indicator,
                    update_hover_indicator_position,
                    start_drag_connection,
                    update_drag_connection,
                    end_drag_connection,
                    break_joints_on_force_limit,
                    update_connection_visuals,
                    handle_despawned_connected_entities,
                )
                    .chain(),
            );
    }
}
