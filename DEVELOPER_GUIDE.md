# Developer Quick Reference Guide

## Project Structure

```
src/
├── main.rs              # App entry point and system registration
├── components.rs        # ECS component definitions
├── constants.rs         # Game constants
│
├── Utilities (Use these!)
├── utils.rs            # Input, rendering, color helpers
├── particles.rs        # Generic particle system
├── body_parts.rs       # Ragdoll component factory
├── physics_utils.rs    # Physics helper functions
├── entity_finder.rs    # Entity query utilities
│
├── Game Systems
├── ragdoll.rs          # Ragdoll spawning
├── bomb.rs             # Bomb mechanics
├── explosion.rs        # Explosion effects
├── wooden_box.rs       # Wooden box objects
├── combustion.rs       # Fire system
├── blood.rs            # Blood particles
├── physics.rs          # Physics application
├── animation.rs        # Visual animations
├── drag.rs             # Drag interaction
└── setup.rs            # Scene initialization
```

## Common Tasks

### 1. Handle Cursor Input
```rust
use crate::utils::get_cursor_world_position;

fn my_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(world_pos) = get_cursor_world_position(&windows, &camera_q) {
        // Use world_pos
    }
}
```

### 2. Manipulate Sprite Colors
```rust
use crate::utils::{set_sprite_alpha, fade_sprite_alpha};

// Set alpha directly
set_sprite_alpha(&mut sprite, 0.5);

// Fade by amount
fade_sprite_alpha(&mut sprite, time.delta_secs() * 0.3);
```

### 3. Create a Body Part
```rust
use crate::body_parts::{spawn_body_part, BodyPartConfig};

let part = spawn_body_part(&mut commands, BodyPartConfig {
    size: Vec2::new(20.0, 30.0),
    position: Vec2::new(100.0, 200.0),
    color: Color::srgb(0.9, 0.7, 0.6),
    density: 1.2,
    health: 100.0,
    ..Default::default()
});
```

### 4. Create a Joint
```rust
use crate::body_parts::{create_joint, JointConfig};

create_joint(&mut commands, JointConfig {
    parent: parent_entity,
    child: child_entity,
    parent_anchor: Vec2::new(0.0, -10.0),
    child_anchor: Vec2::new(0.0, 10.0),
    min_angle: -1.5,
    max_angle: 1.5,
});
```

### 5. Spawn Particles
```rust
use crate::particles::{spawn_particles, ParticleSpawnConfig, FadeMode, ScaleMode};

spawn_particles(&mut commands, ParticleSpawnConfig {
    count: 30,
    position: Vec2::new(0.0, 0.0),
    z_index: 0.5,
    size_range: (4.0, 10.0),
    speed_range: (50.0, 150.0),
    lifetime_range: (1.0, 2.0),
    color_fn: Box::new(|rng| {
        Color::srgba(1.0, rng.gen_range(0.0..0.5), 0.0, 0.8)
    }),
    velocity_fn: Box::new(|rng, speed| {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        Vec2::new(angle.cos(), angle.sin()) * speed
    }),
    gravity: -400.0,
    drag: 0.98,
    fade_mode: FadeMode::Linear,
    scale_mode: ScaleMode::GrowLinear(2.0),
});
```

### 6. Create a Physics Object
```rust
use crate::physics_utils::{spawn_physics_sprite, RigidBodyConfig, ColliderType};

let entity = spawn_physics_sprite(&mut commands, RigidBodyConfig {
    position: Vec2::new(0.0, 100.0),
    size: Vec2::new(50.0, 50.0),
    color: Color::srgb(0.8, 0.4, 0.2),
    body_type: RigidBody::Dynamic,
    collider_type: ColliderType::Cuboid,
    density: 1.0,
    restitution: Some(0.5),
    friction: Some(0.7),
});
```

### 7. Apply Radial Force (Explosion-like)
```rust
use crate::physics_utils::apply_radial_impulse;

let strength = apply_radial_impulse(
    &mut impulse,           // ExternalImpulse component
    object_pos,             // Object position
    explosion_center,       // Explosion center
    200.0,                  // Radius
    50000.0,                // Force magnitude
    true,                   // Apply random torque?
);
```

### 8. Find Closest Entity
```rust
use crate::entity_finder::find_closest_entity;

let query_iter = my_query.iter();
if let Some(entity) = find_closest_entity(query_iter, target_pos, 100.0) {
    // Use entity
}
```

## Adding New Features

### New Spawnable Object
1. Add spawn key to `constants.rs`
2. Create spawn system in appropriate module
3. Use `get_cursor_world_position()` for input
4. Use `spawn_physics_sprite()` or custom spawn function
5. Register system in `main.rs`

### New Particle Effect
1. Use `ParticleSpawnConfig` with custom configuration
2. Define color and velocity functions
3. Choose appropriate `FadeMode` and `ScaleMode`
4. Call `spawn_particles()` when triggered

### New Ragdoll Type
1. Define size constants
2. Create body parts with `BodyPartConfig` variations
3. Connect with `JointConfig`
4. Adjust health, density, and other properties

## Best Practices

### ✅ DO
- Use utility functions from utils.rs
- Use configuration structs for entity creation
- Extract common patterns to utilities
- Follow existing module organization
- Add components to components.rs
- Add constants to constants.rs

### ❌ DON'T
- Duplicate cursor position conversion code
- Manually manipulate sprite RGBA when utilities exist
- Copy-paste entity creation code
- Put multiple concerns in one module
- Hardcode magic numbers (use constants)

## System Organization

### Bevy Update Schedule
```rust
.add_systems(Update, (
    // Input systems (spawning)
    spawn_ragdoll_on_keypress,
    spawn_bomb_on_keypress,
    spawn_wooden_box_on_keypress,
    
    // Timer systems
    bomb_timer_system,
    
    // Physics systems
    apply_explosion,
    cleanup_debris,
    
    // Animation systems
    animate_explosion_flash,
    animate_explosion_shockwave,
    animate_smoke_particles,
    animate_blood_particles,
    
    // Fire systems
    ignite_ragdoll_on_keypress,
    apply_fire_damage,
    spread_fire,
    animate_fire_particles,
))
```

### Chained Systems (Order Matters)
```rust
.add_systems(Update, (
    start_drag_system,
    update_drag_system,
    end_drag_system,
).chain())
```

## Component Patterns

### Standard Entity with Health
```rust
commands.spawn((
    Sprite { /* ... */ },
    Transform { /* ... */ },
    RigidBody::Dynamic,
    Collider::cuboid(width / 2.0, height / 2.0),
    ColliderMassProperties::Density(density),
    ExternalImpulse::default(),
    Health { current: 100.0, max: 100.0 },
    Draggable,
));
```

### Flammable Entity
Add these components:
```rust
Flammable { ignition_threshold: 0.5 }
// Add OnFire when ignited:
OnFire { 
    intensity: 1.0, 
    duration: Timer::from_seconds(8.0, TimerMode::Once) 
}
```

## Debugging Tips

### Enable Physics Debug Rendering
Already enabled in main.rs:
```rust
.add_plugins(RapierDebugRenderPlugin::default())
```

### Common Issues
- **Entities not colliding**: Check collider sizes and RigidBody type
- **Explosions not working**: Verify ExternalImpulse component exists
- **Input not working**: Check if cursor is in window bounds
- **Particles not appearing**: Check z_index ordering

## Performance Considerations

### Optimization Flags
Already configured in Cargo.toml:
```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

### Entity Cleanup
- Despawn entities when they go off-screen (see cleanup_debris)
- Use timers to auto-despawn particle effects
- Remove entities when health reaches 0

## File Naming Conventions

- **Systems**: `verb_noun` (e.g., `spawn_ragdoll`, `apply_explosion`)
- **Components**: `Noun` or `NounState` (e.g., `Bomb`, `OnFire`)
- **Configs**: `NounConfig` (e.g., `BodyPartConfig`)
- **Modules**: `snake_case` matching content (e.g., `body_parts`)

## Git Workflow

Current branch: `refactor-modularize-improve-reusability`

Commit messages should describe the improvement made.

## Documentation

- `README.md` - User-facing documentation
- `REFACTORING.md` - Original refactoring notes
- `REFACTORING_V2.md` - Latest refactoring details
- `MODULARIZATION_IMPROVEMENTS.md` - Detailed improvements
- `DEVELOPER_GUIDE.md` - This file

## Questions?

Refer to:
1. This guide for common patterns
2. Existing code for examples
3. Module documentation in each .rs file
4. Bevy documentation: https://bevyengine.org/learn/

Happy coding! 🦀🎮
