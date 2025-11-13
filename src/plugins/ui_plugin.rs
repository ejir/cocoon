//! Plugin for user interface systems

use bevy::prelude::*;
use crate::core::setup::setup;
use crate::ui::{
    handle_button_clicks, setup_ui_topbar, spawn_selected_object_on_click,
    sync_selection_with_connection_system, SelectedObject,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedObject>()
            .add_systems(Startup, (setup, setup_ui_topbar))
            .add_systems(
                Update,
                (
                    handle_button_clicks,
                    spawn_selected_object_on_click,
                    sync_selection_with_connection_system,
                ),
            );
    }
}
