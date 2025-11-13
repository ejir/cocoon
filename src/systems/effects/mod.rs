pub mod animation;
pub mod blood;
pub mod combustion;
pub mod particles;

pub use animation::{animate_explosion_flash, animate_explosion_shockwave, animate_smoke_particles};
pub use blood::animate_blood_particles;
pub use combustion::{
    animate_fire_particles, apply_fire_damage, ignite_ragdoll_on_keypress, spread_fire, spawn_fire_from_ui,
};
