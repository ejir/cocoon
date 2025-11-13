pub mod connection;
pub mod damage;

pub use connection::{
    SelectionState, DragConnectionState,
    start_drag_connection, update_drag_connection, end_drag_connection,
    update_hover_indicator, update_hover_indicator_position,
    break_joints_on_force_limit, handle_despawned_connected_entities, update_connection_visuals,
};
pub use damage::{apply_explosive_joint_damage, check_joint_damage, collision_joint_damage, detect_impact_damage, track_velocity, visualize_fractures};
