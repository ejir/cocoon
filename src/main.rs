use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
mod blood;
mod body_parts;
mod bomb;
mod combustion;
mod components;
mod connection;
mod constants;
mod damage;
mod drag;
mod drag_create;
mod entity_finder;
mod explosion;
mod iron_block;
mod particles;
mod physics;
mod physics_utils;
mod ragdoll;
mod setup;
mod shockwave;
mod ui_topbar;
mod utils;
mod wooden_box;

use animation::{animate_explosion_flash, animate_explosion_shockwave, animate_smoke_particles};
use blood::animate_blood_particles;
use bomb::{bomb_timer_system, spawn_bomb_on_keypress};
use combustion::{
    animate_fire_particles, apply_fire_damage, ignite_ragdoll_on_keypress, spread_fire,
};
use connection::{
    create_constraint_system, handle_deleted_selections, handle_object_selection,
    update_selection_indicators, SelectionState,
};
use damage::{apply_explosive_joint_damage, check_joint_damage, collision_joint_damage, detect_impact_damage, track_velocity, visualize_fractures};
use drag::{end_drag_system, start_drag_system, update_drag_system, DragState};
use drag_create::{end_create_drag_system, start_create_drag_system, update_create_drag_system, CreateDragState};
use iron_block::spawn_iron_block_on_keypress;
use physics::{apply_explosion, cleanup_debris};
use ragdoll::spawn_ragdoll_on_keypress;
use setup::setup;
use shockwave::{animate_explosion_core, animate_shockwave_visual, shockwave_joint_damage, update_shockwave};
use ui_topbar::{handle_button_clicks, setup_ui_topbar, spawn_selected_object_on_click, sync_selection_with_connection_system, SelectedObject};
use wooden_box::spawn_wooden_box_on_keypress;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D Ragdoll Sandbox".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<DragState>()
        .init_resource::<CreateDragState>()
        .init_resource::<SelectedObject>()
        .init_resource::<SelectionState>()
        .add_systems(Startup, (setup, setup_ui_topbar))
        .add_systems(
            Update,
            (
                spawn_ragdoll_on_keypress,
                spawn_bomb_on_keypress,
                spawn_wooden_box_on_keypress,
                spawn_iron_block_on_keypress,
                bomb_timer_system,
                apply_explosion,
                update_shockwave,
                shockwave_joint_damage,
            ),
        )
        .add_systems(
            Update,
            (
                cleanup_debris,
                animate_explosion_flash,
                animate_explosion_shockwave,
                animate_smoke_particles,
                animate_blood_particles,
                animate_shockwave_visual,
                animate_explosion_core,
            ),
        )
        .add_systems(
            Update,
            (
                ignite_ragdoll_on_keypress,
                apply_fire_damage,
                spread_fire,
                animate_fire_particles,
            ),
        )
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
                start_drag_system,
                start_create_drag_system,
                update_drag_system,
                update_create_drag_system,
                end_drag_system,
                end_create_drag_system,
            ).chain(),
        )
        .add_systems(
            Update,
            (handle_button_clicks, spawn_selected_object_on_click, sync_selection_with_connection_system),
        )
        .add_systems(
            Update,
            (
                handle_object_selection,
                update_selection_indicators,
                create_constraint_system,
                handle_deleted_selections,
            ),
        )
        .run();
}
