# Bevy 2D Ragdoll Sandbox

A 2D physics sandbox game built with Bevy engine, inspired by People Playground. Features realistic ragdoll physics and explosive bombs powered by bevy_rapier2d.

## Features

- **Intuitive UI Top Bar**: Interactive object selection interface with visual feedback for easy spawning
- **Physics-Driven Ragdolls**: Fully articulated humanoid ragdolls with realistic joint constraints
- **Fracture & Dismemberment System**: Body parts can fracture and limbs can be torn off by explosions, impacts, and external forces
  - Joints have health and can be damaged by explosions, collisions, and excessive stress
  - Fractured body parts darken visually to show damage
  - When joint health reaches zero, limbs detach from the body with blood effects
- **Connection System**: Create physics constraints between any two objects
  - Fixed constraints for rigid connections
  - Hinge constraints for rotational joints
  - Visual selection indicators (green for first object, blue for second)
- **Realistic Shockwave Explosions**: Timed explosives with propagating shockwaves, pressure-based damage, and physics
- **Wooden Boxes**: Destructible wooden boxes that can be damaged and destroyed by explosions and fire
- **Iron Blocks**: Indestructible metal blocks that are affected by explosion forces but cannot be damaged or destroyed
- **Combustion System**: Set ragdolls and wooden boxes on fire with spreading flames and continuous damage
- **Destructible Objects**: Ragdoll parts and wooden boxes can be destroyed by explosions and fire
- **Blood Effects**: Realistic blood particle system with physics and fade-out animations
- **Fire Effects**: Dynamic fire particle system with realistic animations and spreading mechanics
- **Multi-Object Interactions**: Realistic collisions, impulses, and constraints between all objects
- **Realistic Physics**: Powered by Rapier2D physics engine with proper mass, damping, and gravity
- **Advanced Explosion Visuals**: Multi-layered shockwave rings with realistic expansion and fading

## Controls

### Object Selection UI
- **Top Bar**: Click buttons to select which object to spawn
  - **Ragdoll (R)**: Select ragdoll for spawning
  - **Bomb (B)**: Select bomb for spawning
  - **Box (W)**: Select wooden box for spawning (drag to create custom sizes)
  - **Iron (I)**: Select iron block for spawning (drag to create custom sizes)
  - **Fire (F)**: Select fire tool for spawning
  - **Fixed (X)**: Select fixed constraint tool for connecting objects
  - **Hinge (H)**: Select hinge constraint tool for connecting objects
- **Left Mouse Click**: Spawn the selected object at cursor position (when not dragging)
- **Left Mouse Drag** (Box/Iron): Click and drag to create boxes or iron blocks with custom sizes

### Keyboard Shortcuts (Alternative)
- **R**: Spawn a ragdoll at cursor position
- **B**: Spawn a bomb at cursor position (explodes after 2 seconds)
- **W**: Spawn a wooden box at cursor position
- **I**: Spawn an iron block at cursor position
- **F**: Ignite nearest flammable object near cursor (sets it on fire)

### Mouse Controls
- **Left Mouse Drag**: Click and drag to move ragdoll parts, bombs, wooden boxes, and iron blocks
- **Left Mouse Drag (Create Mode)**: When Box or Iron is selected, drag on empty space to create objects with custom sizes (20-200 pixels)

### Connection System Controls
When Fixed or Hinge constraint tool is selected, you can use either of two connection modes:

**Mode 1: Click-to-Connect** (Original method)
1. **First Click**: Select the first object (highlighted with green indicator)
2. **Second Click**: Select the second object (highlighted with blue indicator)
3. **C or Enter**: Create the constraint between the two selected objects
4. **Escape**: Cancel selection and clear indicators
- **Click on selected object again**: Deselect and clear all selections

**Mode 2: Drag-to-Connect** (New method)
1. **Click and Hold** on the first object
2. **Drag** to the second object (a green line shows the connection)
3. **Release** over the second object to create the connection instantly
- If released over empty space, the connection is cancelled
- Visual feedback: green circles at start/end points and connecting line

**Both modes:**
- Constraints are created at the midpoint between the two objects
- Fixed constraints prevent any relative movement between objects
- Hinge constraints allow rotation around the connection point

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

### Fracture & Dismemberment System

Joints connecting body parts have health and can be damaged by various forces:
1. **Shockwave Damage**: Explosion shockwaves apply pressure-based damage to joints
2. **Collision Damage**: High-velocity impacts (>150 units/sec) damage joints, with extra damage from heavy objects
3. **Stress Damage**: Large velocity differences between connected parts cause joint stress
4. **Impact Detection**: Sudden velocity changes (>200 units/sec) indicate impacts
5. **Fall Damage**: Falling from heights causes enhanced impact damage when hitting surfaces
6. **High-Impact Multipliers**: Extreme velocities (>500 units/sec) apply up to 3x damage multiplier

Joint damage mechanics:
- Each joint starts with 100 health points
- When joint health drops below 50%, the body part becomes "fractured" (darkens visually)
- When joint health reaches 0, the joint breaks completely:
  - The ImpulseJoint constraint is removed
  - The limb detaches from the body
  - Blood particles spawn at the separation point with velocity-based spray
  - The detached limb continues as an independent physics object
- Body parts are more likely to detach from:
  - Falling from significant heights
  - Being hit by heavy objects (iron blocks, etc.)
  - Explosion impacts
  - High-velocity collisions

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

### Iron Block System

Iron blocks are indestructible physics objects:
1. Cannot be damaged or destroyed by any means (explosions, fire, collisions)
2. **Affected by impact forces**: Explosion shockwaves and collisions apply realistic physics forces
3. Heavy objects with steel-like density (7.8), so they move less than lighter objects
4. Can block explosion shockwaves, protecting objects behind them
5. Can be dragged and moved like other objects
6. Ideal for creating barriers, shields, and heavy projectiles

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
1. Click and hold left mouse button on any ragdoll part, bomb, wooden box, or iron block to start dragging
2. Object temporarily becomes kinematic (unaffected by physics) while dragging
3. Follows cursor position with offset from original click point
4. Release mouse button to drop object and restore dynamic physics behavior
5. Supports dragging individual ragdoll parts - joints remain connected

### Drag-to-Create System

Create custom-sized objects by dragging:
1. Select Box (W) or Iron (I) from the top bar
2. Click and hold left mouse button on empty space (not on an existing object)
3. Drag to define the size of the object - a semi-transparent preview shows the size
4. Release to spawn the object at the final size
5. Size constraints: minimum 20 pixels, maximum 500 pixels per dimension
6. Health and mass scale proportionally with object size

### Connection System

Create physics constraints between any two objects using two different modes:

**Mode 1: Click-to-Connect**
1. Select Fixed (X) or Hinge (H) constraint tool from the top bar
2. Click on the first object to select it (green selection indicator appears)
3. Click on a second object to select it (blue selection indicator appears)
4. Press C or Enter to create the constraint
5. Visual indicators follow the selected objects and can be cleared with Escape

**Mode 2: Drag-to-Connect**
1. Select Fixed (X) or Hinge (H) constraint tool from the top bar
2. Click and hold on the first object
3. Drag to the second object (green line and circles show the connection path)
4. Release the mouse button over the second object to instantly create the constraint
5. If released over empty space, the connection is cancelled

**Constraint Types:**
- **Fixed Constraints**: Rigidly attach two objects with no relative movement
- **Hinge Constraints**: Create a rotational joint allowing objects to pivot around the connection point
- Constraints are created at the midpoint between the two objects
- All objects (ragdoll parts, bombs, boxes, iron blocks) can be connected

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

- Additional object types (platforms, ropes)
- More explosion effects and particle systems
- Interactive tools (grab, pin, delete)
- Different bomb types with varying power
- Wound visualization on damaged body parts
- Wood debris particles when boxes are destroyed
- Save/load scene functionality
