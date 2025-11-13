//! Plugin for input handling systems

use bevy::prelude::*;
use crate::systems::input::{
    end_create_drag_system, end_drag_system, start_create_drag_system, start_drag_system,
    update_create_drag_system, update_drag_system, CreateDragState, DragState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>()
            .init_resource::<CreateDragState>()
            .add_systems(
                Update,
                (
                    start_drag_system,
                    start_create_drag_system,
                    update_drag_system,
                    update_create_drag_system,
                    end_drag_system,
                    end_create_drag_system,
                )
                    .chain(),
            );
    }
}
