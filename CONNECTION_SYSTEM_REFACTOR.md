# Connection System Refactoring

> **⚠️ NOTE: This document is OUTDATED**  
> Spring constraint support described in this document has been **removed** in a later refactor.  
> See [CONNECTION_MATERIALS_REFACTOR.md](CONNECTION_MATERIALS_REFACTOR.md) for the current system which uses:
> - **Fixed** (non-rotatable, like nails) and **Hinge** (rotatable, like bearings) constraints only
> - **Material selection** (Wood, Metal, Rope, Plastic) for different joint properties
> - Spring has been completely removed from the system

## Overview
This document describes the refactoring of the connection system to provide better visual feedback and expanded joint type support.

## New Features

### 1. Hover Highlighting
When in Connect mode (Fixed, Hinge, or Spring constraint selected), hovering the mouse over any connectable object displays a visual highlight.

**Implementation:**
- New component: `HoverIndicator` - tracks the hover highlight sprite
- New system: `update_hover_indicator()` - detects mouse position and spawns/removes hover indicator
- New system: `update_hover_indicator_position()` - follows the target entity

**Visual Design:**
- Color: Yellow with low alpha `srgba(1.0, 1.0, 0.3, 0.3)`
- Size: 65x65 pixels (slightly larger than selection indicators)
- Z-position: 0.5 (between selection indicators at 1.0 and objects at 0.0)

**Behavior:**
- Only active when `selection_state.is_enabled == true`
- Disabled during drag operations
- Updates every frame to follow cursor
- Automatically removed when leaving Connect mode

### 2. Spring Joint Support
Added Spring joint type alongside Fixed (Weld) and Hinge joints.

**Joint Types:**
1. **Fixed** - Rigid connection, no relative movement
2. **Hinge** - Rotational joint, allows pivoting
3. **Spring** - Elastic connection, allows stretching/compression

**Spring Joint Parameters:**
- `rest_length`: Distance between objects when created
- `stiffness`: 100.0 (how strongly it resists deformation)
- `damping`: 5.0 (how quickly oscillations settle)

**API Usage:**
```rust
let rest_length = (pos1 - pos2).length();
let joint = SpringJointBuilder::new(rest_length, 100.0, 5.0)
    .local_anchor1(anchor1)
    .local_anchor2(anchor2);
```

### 3. Enhanced UI
Added Spring constraint button to the top bar.

**UI Elements:**
- Button label: "Spring (S)"
- Keyboard shortcut: S key (future enhancement)
- Button position: After Hinge button

## System Architecture

### Connection Mode Flow

```
1. User selects Fixed/Hinge/Spring button in UI
   ↓
2. sync_selection_with_connection_system() enables connection mode
   ↓
3. update_hover_indicator() shows highlights on mouse hover
   ↓
4. User can choose either connection method:
   
   Mode 1 (Click-to-Connect):
   - Click first object (green indicator)
   - Click second object (blue indicator)
   - Press C or Enter to create joint
   
   Mode 2 (Drag-to-Connect):
   - Click and hold on first object
   - Drag to second object (rubber band line visible)
   - Release to create joint
```

### System Execution Order

```rust
.add_systems(
    Update,
    (
        // Hover highlighting (runs first)
        update_hover_indicator,
        update_hover_indicator_position,
        
        // Mode 1: Click-based connection
        handle_object_selection,
        update_selection_indicators,
        create_constraint_system,
        handle_deleted_selections,
        
        // Mode 2: Drag-based connection
        start_drag_connection,
        update_drag_connection,
        end_drag_connection,
    ).chain(),
)
```

## Modified Files

### src/connection.rs
- Added `ConstraintType::Spring` enum variant
- Added `HoverIndicator` component
- Added `update_hover_indicator()` system
- Added `spawn_hover_indicator()` helper function
- Added `update_hover_indicator_position()` system
- Updated `create_constraint_system()` to handle Spring joints
- Updated `end_drag_connection()` to handle Spring joints

### src/ui_topbar.rs
- Added `ObjectType::SpringConstraint` enum variant
- Added Spring button in `setup_ui_topbar()`
- Updated `spawn_selected_object_on_click()` to handle SpringConstraint
- Updated `sync_selection_with_connection_system()` to enable Spring mode

### src/main.rs
- Imported new hover systems
- Added hover systems to connection system chain

## Visual Feedback Summary

| State | Indicator | Color | Size | Z-Index |
|-------|-----------|-------|------|---------|
| Hover | Yellow ring | `srgba(1.0, 1.0, 0.3, 0.3)` | 65x65 | 0.5 |
| First selected | Green ring | `srgba(0.0, 1.0, 0.0, 0.6)` | 60x60 | 1.0 |
| Second selected | Blue ring | `srgba(0.0, 0.5, 1.0, 0.6)` | 60x60 | 1.0 |
| Drag line | Green line + circles | `srgb(0.2, 0.8, 0.2)` | 8px radius | - |

## Usage Examples

### Creating a Spring Connection (Click Mode)
1. Click "Spring (S)" button in top bar
2. Hover over objects - yellow highlights appear
3. Click first object - green ring appears
4. Click second object - blue ring appears
5. Press C or Enter - spring joint created

### Creating a Spring Connection (Drag Mode)
1. Click "Spring (S)" button in top bar
2. Hover over first object - yellow highlight appears
3. Click and hold on first object
4. Drag to second object - green line follows cursor
5. Release over second object - spring joint created instantly

## Technical Notes

### Spring Joint Tuning
The default spring parameters can be adjusted:
- **Stiffness (100.0)**: Higher = stiffer spring (less stretching)
- **Damping (5.0)**: Higher = quicker settling (less bouncing)

### Hover Detection
- Detection radius: 50.0 pixels
- Uses closest entity if multiple objects in range
- Respects physics collision bounds

### Performance Considerations
- Hover system runs every frame when in Connect mode
- Efficient entity queries with `With<Connectable>` filter
- Indicators despawned immediately when mode changes

## Future Enhancements

Possible improvements:
1. Configurable spring parameters via UI slider
2. Visual spring rendering (coil graphic)
3. Spring stress visualization (color changes under tension)
4. Keyboard shortcut for Spring mode (S key)
5. Breaking force for springs (joint breaks if over-stressed)
6. Different spring types (tension-only, compression-only)

## Testing Checklist

- [x] Hover indicator appears when mouse over connectable objects
- [x] Hover indicator follows objects as they move
- [x] Hover indicator disappears when leaving Connect mode
- [x] Spring joints can be created with click mode
- [x] Spring joints can be created with drag mode
- [x] Spring joints behave elastically (stretch and compress)
- [x] UI button for Spring constraint works
- [x] All existing functionality preserved (Fixed, Hinge)

## Compatibility

- **Bevy Version**: 0.15
- **bevy_rapier2d Version**: 0.28
- **Breaking Changes**: None - all existing features preserved
