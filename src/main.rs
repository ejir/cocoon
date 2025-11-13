//! Bevy 2D Ragdoll Sandbox - A physics simulation game with ragdolls and explosions

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod core;
mod entities;
mod plugins;
mod prelude;
mod systems;
mod ui;

use plugins::{DamagePlugin, EffectsPlugin, EntitiesPlugin, InputPlugin, UiPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy 2D Ragdoll Sandbox".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            UiPlugin,
            EntitiesPlugin,
            EffectsPlugin,
            InputPlugin,
            DamagePlugin,
        ))
        .run();
}
