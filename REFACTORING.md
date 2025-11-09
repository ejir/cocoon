# Code Refactoring Summary

## Overview
The original `main.rs` (784 lines) has been refactored into 11 well-organized modules following Rust best practices.

## Module Structure

### `main.rs` (56 lines)
- App configuration and plugin setup
- System registration
- Entry point for the application

### `constants.rs` (11 lines)
- `EXPLOSION_RADIUS`, `EXPLOSION_FORCE`
- Fire constants: `FIRE_DAMAGE_PER_SEC`, `FIRE_SPREAD_RADIUS`, `FIRE_DURATION`
- Keyboard constants: `RAGDOLL_SPAWN_KEY`, `BOMB_SPAWN_KEY`, `FIRE_SPAWN_KEY`

### `components.rs` (68 lines)
- All ECS component definitions:
  - `Bomb`, `Explosion`, `RagdollPart`, `Debris`
  - `Health` - Health system for ragdoll parts
  - `ExplosionFlash`, `ExplosionShockwave`, `SmokeParticle`
  - `BloodParticle` - Blood particle system
  - `OnFire` - Marks entities that are burning with intensity and duration
  - `Flammable` - Marks entities that can catch fire
  - `FireParticle` - Fire particle system

### `setup.rs` (65 lines)
- Initial scene setup
- Ground and wall spawning
- UI text setup

### `ragdoll.rs` (389 lines)
- Ragdoll spawning system
- Body part creation logic with health values and flammable properties
- Joint creation and configuration

### `bomb.rs` (72 lines)
- Bomb spawning on keypress
- Bomb timer system
- Explosion triggering logic

### `explosion.rs` (127 lines)
- Visual explosion effects
- Debris particle spawning
- Smoke particle generation

### `physics.rs` (71 lines)
- Explosion force application
- Ragdoll damage calculation
- Health-based destruction system
- Debris cleanup system

### `blood.rs` (80 lines)
- Blood particle spawning with physics
- Blood particle animation and fade-out
- Gravity and velocity simulation for blood

### `animation.rs` (81 lines)
- Flash animation system
- Shockwave animation system
- Smoke particle animation system

### `combustion.rs` (171 lines)
- Ragdoll ignition system on keypress
- Fire damage application over time
- Fire spreading to nearby flammable parts
- Fire particle spawning and animation
- Fire particle effects with realistic physics

## Benefits

1. **Separation of Concerns**: Each module has a clear, single responsibility
2. **Maintainability**: Easier to locate and modify specific functionality
3. **Readability**: Smaller, focused files are easier to understand
4. **Testability**: Modules can be tested independently
5. **Scalability**: New features can be added without cluttering main.rs

## Design Principles Applied

- **Modular Architecture**: Related functionality grouped together
- **Encapsulation**: Private helper functions within modules
- **Clear Interfaces**: Public functions for cross-module communication
- **Logical Organization**: Systems organized by domain (ragdoll, bomb, explosion, etc.)
