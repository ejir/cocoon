//! Plugin for entity spawning and management systems

use bevy::prelude::*;
use crate::entities::obstacles::{spawn_iron_block_on_keypress, spawn_wooden_box_on_keypress};
use crate::entities::ragdoll::spawn_ragdoll_on_keypress;
use crate::entities::weapons::{
    animate_explosion_core, animate_shockwave_visual, bomb_timer_system,
    shockwave_joint_damage, spawn_bomb_on_keypress, update_shockwave,
};
use crate::systems::physics::{apply_explosion, cleanup_debris};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
                cleanup_debris,
                animate_shockwave_visual,
                animate_explosion_core,
            ),
        );
    }
}
