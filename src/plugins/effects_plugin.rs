//! Plugin for visual effects systems

use bevy::prelude::*;
use crate::systems::effects::{
    animate_blood_particles, animate_explosion_flash, animate_explosion_shockwave,
    animate_fire_particles, animate_smoke_particles, apply_fire_damage,
    ignite_ragdoll_on_keypress, spread_fire,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_explosion_flash,
                animate_explosion_shockwave,
                animate_smoke_particles,
                animate_blood_particles,
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
        );
    }
}
