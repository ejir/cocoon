# Bevy 2D Ragdoll Sandbox

A 2D physics sandbox game built with Bevy engine, inspired by People Playground. Features realistic ragdoll physics and explosive bombs powered by bevy_rapier2d.

## Features

- **Physics-Driven Ragdolls**: Fully articulated humanoid ragdolls with realistic joint constraints
- **Realistic Shockwave Explosions**: Timed explosives with propagating shockwaves, pressure-based damage, and physics
- **Wooden Boxes**: Destructible wooden boxes that can be damaged and destroyed by explosions and fire
- **Combustion System**: Set ragdolls and wooden boxes on fire with spreading flames and continuous damage
- **Destructible Objects**: Ragdoll parts and wooden boxes can be destroyed by explosions and fire
- **Blood Effects**: Realistic blood particle system with physics and fade-out animations
- **Fire Effects**: Dynamic fire particle system with realistic animations and spreading mechanics
- **Multi-Object Interactions**: Realistic collisions, impulses, and constraints between all objects
- **Realistic Physics**: Powered by Rapier2D physics engine with proper mass, damping, and gravity
- **Advanced Explosion Visuals**: Multi-layered shockwave rings with realistic expansion and fading

## Controls

- **R**: Spawn a ragdoll at cursor position
- **B**: Spawn a bomb at cursor position (explodes after 2 seconds)
- **W**: Spawn a wooden box at cursor position
- **F**: Ignite nearest flammable object near cursor (sets it on fire)
- **Left Mouse Button**: Click and drag to move ragdoll parts, bombs, and wooden boxes

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

Bombs use a realistic shockwave-based explosion system:
1. Bomb spawns as a dynamic rigid body
2. After 2 seconds, it explodes and generates a shockwave
3. **Realistic Shockwave Physics**:
   - Shockwave propagates outward at 1200 pixels/second (simulating ~340 m/s sound speed)
   - Peak pressure of 80,000 units applied to objects in the wave front
   - Pressure decays realistically with distance using inverse square law
   - Wave has physical thickness (80 pixels) for realistic interaction
4. **Dynamic Force Application**:
   - Force considers object mass and cross-sectional area
   - Larger objects experience more force (pressure × area)
   - Torque applied based on distance and randomization for realistic tumbling
5. **Advanced Damage Model**:
   - Damage based on pressure and induced velocity
   - Accounts for mass-dependent acceleration effects
   - Higher velocity changes cause additional damage
6. Spawns visual debris, smoke, blood particles, and shockwave rings
7. Multiple overlapping shockwave visual effects for realistic appearance

### Wooden Box System

Wooden boxes are destructible physics objects:
1. Each box has 100 health points
2. Can be damaged and destroyed by explosions
3. Flammable - can be set on fire and will burn for 8 seconds
4. Fire spreads from boxes to nearby flammable objects
5. Realistic physics with wood-like density (0.8) and friction (0.7)
6. Can be dragged and moved like other objects
7. When destroyed by fire or explosions, the box is removed from the scene

### Combustion System

The fire system provides realistic burning mechanics:
1. Flammable objects (ragdoll parts and wooden boxes) can be ignited by clicking near them with the F key
2. Fire continuously damages objects over time (15 HP/sec)
3. Fire spreads to nearby flammable objects within 50 pixels
4. Fire burns for 8 seconds by default before extinguishing
5. Dynamic fire particle effects with varied colors (yellow, orange, red)
6. Fire particles rise and fade out realistically
7. Burning ragdoll parts spawn blood particles when destroyed

### Drag System

Interactive mouse-based object manipulation:
1. Click and hold left mouse button on any ragdoll part, bomb, or wooden box to start dragging
2. Object temporarily becomes kinematic (unaffected by physics) while dragging
3. Follows cursor position with offset from original click point
4. Release mouse button to drop object and restore dynamic physics behavior
5. Supports dragging individual ragdoll parts - joints remain connected

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

- Additional object types (platforms, ropes, metal boxes)
- More explosion effects and particle systems
- Interactive tools (grab, pin, delete)
- Different bomb types with varying power
- Wound visualization on damaged body parts
- Wood debris particles when boxes are destroyed
- Save/load scene functionality
