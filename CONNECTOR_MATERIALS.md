# Physical Connector Materials

## Overview
The connection system now creates **physical connector materials** between connected objects instead of directly joining them. This provides a more realistic simulation where the connecting material (wood, metal, rope, plastic) is visible and interactive.

## Key Features

### Physical Properties
Each connector is a real physics object with:
- **Collider**: Can interact with other objects
- **Mass**: Density varies by material type
- **Health**: Can be damaged and destroyed
- **Visual Appearance**: Color and size based on material

### Material Types

| Material | Width | Density | Health Multiplier | Visual Color |
|----------|-------|---------|-------------------|--------------|
| **Wood** | 6.0px | 0.6 | 1.0x | Brown (RGB: 0.6, 0.4, 0.2) |
| **Metal** | 4.0px | 7.8 | 2.0x | Gray/Silver (RGB: 0.7, 0.7, 0.8) |
| **Rope** | 3.0px | 0.3 | 0.5x | Tan (RGB: 0.8, 0.7, 0.5) |
| **Plastic** | 5.0px | 0.9 | 1.5x | Blue (RGB: 0.3, 0.6, 0.9) |

### How It Works

1. **Connection Creation**:
   - When user drags from object A to object B
   - A physical connector material is spawned between the two points
   - The connector is positioned at the midpoint
   - The connector is rotated to align with the connection direction

2. **Joint Configuration**:
   - Two joints are created instead of one:
     - Joint 1: Connects object A to the start of the connector
     - Joint 2: Connects object B to the end of the connector
   - Joint type (Fixed or Hinge) applies to both connections

3. **Physical Simulation**:
   - The connector has mass and inertia
   - It can collide with other objects
   - It can be damaged by explosions and impacts
   - When destroyed, both joints break and objects separate

### Benefits

1. **Visual Realism**: You can see the connecting material
2. **Physical Realism**: Connection has weight and takes up space
3. **Gameplay Depth**: Connections can be targeted and destroyed
4. **Interaction**: Connectors are themselves connectable objects
5. **Material Variety**: Different materials behave differently

### Technical Details

#### Connector Entity Components
```rust
- Sprite (visual representation)
- Transform (position and rotation)
- RigidBody::Dynamic
- Collider (thin rectangle)
- ColliderMassProperties::Density
- Health (destructible)
- ConnectorMaterial (material type)
- Connectable (can be connected to)
```

#### Anchor Point Calculation
- Anchors are calculated in local space for each object
- For the connector, anchors are at the ends: `(-length/2, 0)` and `(length/2, 0)`
- For connected objects, anchors are at the click positions
- This ensures proper alignment when objects rotate

### Implementation Files
- `src/connection.rs` - Contains `create_connector_material()` and connection logic
- `src/components.rs` - Health component definition
- `src/main.rs` - System registration

### Future Enhancements
- Material wear and tear over time
- Visual damage indicators on connectors
- Different connector shapes (springs, chains)
- Connector breaking sounds based on material
- Heat/fire propagation through connectors
