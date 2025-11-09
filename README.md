# Bevy 2D Ragdoll Sandbox

A 2D physics sandbox game built with Bevy engine, inspired by People Playground. Features realistic ragdoll physics and explosive bombs powered by bevy_rapier2d.

## Features

- **Physics-Driven Ragdolls**: Fully articulated humanoid ragdolls with realistic joint constraints
- **Explosive Bombs**: Timed explosives with radial force application
- **Destructible Ragdolls**: Ragdoll parts can be destroyed by explosions with blood particle effects
- **Blood Effects**: Realistic blood particle system with physics and fade-out animations
- **Multi-Object Interactions**: Realistic collisions, impulses, and constraints between all objects
- **Realistic Physics**: Powered by Rapier2D physics engine with proper mass, damping, and gravity

## Controls

- **R**: Spawn a ragdoll at cursor position
- **B**: Spawn a bomb at cursor position (explodes after 2 seconds)

## Technical Details

### Ragdoll System

Each ragdoll consists of 9 body parts connected with revolute joints:
- Head
- Torso
- Upper and lower arms (left and right)
- Upper and lower legs (left and right)

Joints have realistic angle limits to simulate anatomical constraints.

Each body part has its own health system:
- Parts can be damaged and destroyed by explosions
- Health values vary by body part (torso has the most health)
- When destroyed, parts spawn blood particle effects and are removed from the scene

### Bomb System

Bombs use a timer-based explosion system:
1. Bomb spawns as a dynamic rigid body
2. After 2 seconds, it explodes
3. Applies radial impulse force to all nearby objects
4. Damages ragdoll parts based on distance and explosion strength
5. Spawns visual debris, smoke, and blood particles
6. Force and damage decrease with distance from explosion center

### Physics Configuration

- Gravity: 981 pixels/second² (Earth-like gravity)
- Pixels per meter: 100 (for proper physics scale)
- Collision detection and response handled by Rapier2D
- Debug rendering enabled for visualizing colliders and joints

## Building and Running

```bash
# Build the project
cargo build --release

# Run the game
cargo run --release
```

## Dependencies

- `bevy` 0.15 - Game engine
- `bevy_rapier2d` 0.28 - 2D physics plugin
- `rand` 0.8 - Random number generation for explosions

## Performance

The game uses optimized compilation settings for fast development builds:
- Dev dependencies are compiled with optimizations
- Debug builds use minimal optimization level 1

## Future Enhancements

- Additional object types (boxes, platforms, ropes)
- More explosion effects and particle systems
- Interactive tools (grab, pin, delete)
- Different bomb types with varying power
- Wound visualization on damaged body parts
- Save/load scene functionality
