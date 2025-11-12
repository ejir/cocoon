# Connection System Materials Refactor

## Overview
This document describes the refactoring of the connection system to support different materials and the removal of the Spring constraint type. The system now focuses on two core connection types (Fixed and Hinge) with material properties that affect joint behavior.

## Changes Made

### 1. Removed Spring Constraint Type
The Spring constraint has been completely removed from the system:
- Removed `ConstraintType::Spring` enum variant
- Removed Spring button from UI (`ObjectType::SpringConstraint`)
- Removed all Spring joint creation logic from connection systems

### 2. Added Material System
Introduced a material-based system for connections with four material types:

#### ConnectionMaterial Enum
```rust
pub enum ConnectionMaterial {
    Wood,      // Weak, moderate flexibility
    Metal,     // Strong, rigid
    Rope,      // Weak, high flexibility
    Plastic,   // Moderate strength and flexibility
}
```

#### Material Properties
Each material has unique physical properties:

| Material | Compliance | Damping | Description |
|----------|-----------|---------|-------------|
| Wood | 0.00001 | 0.5 | Moderate strength, good for general construction |
| Metal | 0.000001 | 0.1 | Very strong and rigid, best for solid connections |
| Rope | 0.0001 | 2.0 | Flexible and weak, allows more movement |
| Plastic | 0.00005 | 1.0 | Balanced properties between wood and metal |

**Compliance**: Lower values = stiffer connection (less give under stress)
**Damping**: Higher values = less oscillation/bouncing

### 3. Updated Connection Types
The system now has two connection types with clear behavior:

#### Fixed Connection (Non-Rotatable)
- Like a nail or weld
- No rotation allowed between connected objects
- Rigid connection at the point of contact
- Material affects joint strength and flexibility

#### Hinge Connection (Rotatable)  
- Like a bearing or hinge
- Allows free rotation around the connection point
- Connected objects can pivot relative to each other
- Material affects rotational friction and stability

### 4. Enhanced UI System
Added material selection buttons to the top bar:

**New UI Buttons:**
- "Wood" - Select wood material for connections
- "Metal" - Select metal material for connections (default)
- "Rope" - Select rope material for connections
- "Plastic" - Select plastic material for connections

**Connection Type Buttons (retained):**
- "Fixed (X)" - Non-rotatable connection
- "Hinge (H)" - Rotatable connection

### 5. Material-Aware Joint Storage
Added `JointMaterial` component to store material information:
```rust
#[derive(Component, Clone, Copy)]
pub struct JointMaterial(pub ConnectionMaterial);
```

This allows querying and inspecting joint materials after creation, useful for:
- Visual representation of different material connections
- Damage systems that consider material strength
- Future UI for editing existing connections

## Usage Examples

### Creating a Metal Fixed Connection
1. Click "Fixed (X)" button in top bar
2. Click "Metal" button (or skip if already selected)
3. Click first object - green ring appears
4. Click second object - blue ring appears  
5. Press C or Enter - rigid metal connection created

### Creating a Rope Hinge Connection
1. Click "Hinge (H)" button in top bar
2. Click "Rope" button to select rope material
3. Use Click Mode: Click two objects, then press C
   OR
   Use Drag Mode: Drag from first to second object
4. Flexible hinged connection with rope properties created

### Switching Materials Mid-Connection
1. Click "Fixed (X)" to enter connection mode
2. Click "Wood" for wooden connections
3. Create some connections...
4. Click "Metal" to switch to metal
5. Continue creating connections with metal properties
6. Connection mode remains active when switching materials

## Technical Implementation

### SelectionState Resource
Updated to include material selection:
```rust
pub struct SelectionState {
    pub constraint_type: ConstraintType,  // Fixed or Hinge
    pub material: ConnectionMaterial,      // Wood, Metal, Rope, or Plastic
    pub is_enabled: bool,
    // ... other fields
}
```

### Connection Creation
Both Click and Drag modes now:
1. Read the selected material from `SelectionState`
2. Create the appropriate joint type (Fixed or Hinge)
3. Attach `JointMaterial` component with selected material
4. Material properties can influence joint behavior

### File Changes

**Modified Files:**
- `src/connection.rs` - Added material system, removed Spring
- `src/ui_topbar.rs` - Updated UI buttons and material selection
- `src/main.rs` - No changes needed (systems remain the same)

## Migration Notes

### For Existing Code
- Any references to `ConstraintType::Spring` must be removed
- UI code referencing `ObjectType::SpringConstraint` must be updated
- Spring joint creation logic is no longer available

### Future Enhancements
Possible improvements:
1. Visual distinction for different materials (colored connection lines)
2. Material-based joint breaking forces
3. UI to change material of existing connections
4. Sound effects based on material type
5. Additional materials (chain, carbon fiber, etc.)
6. Material durability/wear system

## Compatibility

- **Bevy Version**: 0.15
- **bevy_rapier2d Version**: 0.28
- **Breaking Changes**: 
  - Spring constraint type removed
  - Material system is new addition
  - Default material is Metal (maintains rigid behavior similar to previous Fixed joints)

## Testing Checklist

- [x] Fixed joints can be created with all material types
- [x] Hinge joints can be created with all material types
- [x] Material buttons change the active material
- [x] Connection mode remains active when switching materials
- [x] Both Click and Drag modes work with materials
- [x] JointMaterial component is attached to created joints
- [x] Spring constraint is completely removed
- [x] Code compiles without errors

## Benefits of This Refactor

1. **Simplified System**: Two clear connection types instead of three
2. **More Flexible**: Material properties allow fine-tuning without new constraint types
3. **Extensible**: Easy to add new materials without changing core logic
4. **Intuitive**: "Fixed vs Rotatable" is clearer than "Fixed, Hinge, and Spring"
5. **Game-like**: Material selection adds gameplay depth
6. **Realistic**: Materials behave according to real-world properties
