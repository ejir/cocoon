//! Plugin modules for organizing system registration

pub mod entities_plugin;
pub mod effects_plugin;
pub mod input_plugin;
pub mod damage_plugin;
pub mod ui_plugin;

pub use entities_plugin::EntitiesPlugin;
pub use effects_plugin::EffectsPlugin;
pub use input_plugin::InputPlugin;
pub use damage_plugin::DamagePlugin;
pub use ui_plugin::UiPlugin;
