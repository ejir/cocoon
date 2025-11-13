//! Weapon entities including bombs, explosions, and shockwaves

pub mod bomb;
pub mod explosion;
pub mod shockwave;

pub use bomb::{bomb_timer_system, spawn_bomb_on_keypress, spawn_bomb_from_ui};
pub use shockwave::{animate_explosion_core, animate_shockwave_visual, shockwave_joint_damage, update_shockwave};
