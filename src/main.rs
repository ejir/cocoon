use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
mod blood;
mod body_parts;
mod bomb;
mod combustion;
mod components;
mod constants;
mod drag;
mod entity_finder;
mod explosion;
mod particles;
mod physics;
mod physics_utils;
mod ragdoll;
mod setup;
mod utils;
mod wooden_box;

use animation::{animate_explosion_flash, animate_explosion_shockwave, animate_smoke_particles};
use blood::animate_blood_particles;
use bomb::{bomb_timer_system, spawn_bomb_on_keypress};
use combustion::{animate_fire_particles, apply_fire_damage, ignite_ragdoll_on_keypress, spread_fire};
use drag::{end_drag_system, start_drag_system, update_drag_system, DragState};
use physics::{apply_explosion, cleanup_debris};
use ragdoll::spawn_ragdoll_on_keypress;
use setup::setup;
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_ragdoll_on_keypress,
                spawn_bomb_on_keypress,
                spawn_wooden_box_on_keypress,
                bomb_timer_system,
                apply_explosion,
                cleanup_debris,
                animate_explosion_flash,
                animate_explosion_shockwave,
                animate_smoke_particles,
                animate_blood_particles,
                ignite_ragdoll_on_keypress,
                apply_fire_damage,
                spread_fire,
                animate_fire_particles,
            ),
        )
        .add_systems(
            Update,
            (
                start_drag_system,
                update_drag_system,
                end_drag_system,
            )
                .chain(),
        )
        .run();
}
