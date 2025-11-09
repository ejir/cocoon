use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animation;
mod bomb;
mod components;
mod constants;
mod explosion;
mod physics;
mod ragdoll;
mod setup;

use animation::{animate_explosion_flash, animate_explosion_shockwave, animate_smoke_particles};
use bomb::{bomb_timer_system, spawn_bomb_on_keypress};
use physics::{apply_explosion, cleanup_debris};
use ragdoll::spawn_ragdoll_on_keypress;
use setup::setup;

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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_ragdoll_on_keypress,
                spawn_bomb_on_keypress,
                bomb_timer_system,
                apply_explosion,
                cleanup_debris,
                animate_explosion_flash,
                animate_explosion_shockwave,
                animate_smoke_particles,
            ),
        )
        .run();
}
