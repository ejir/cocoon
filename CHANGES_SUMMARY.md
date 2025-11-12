# Connection System Changes Summary

## Changes Made

### 1. Removed Click Mode - Only Drag Mode Now

**Rationale**: Simplified the connection system to use only drag-based connections, which are more intuitive and fix the connection starting point issue.

**Changes**:
- Removed `ConnectionMode` enum and `ConnectionModeState` resource
- Removed all Click mode systems:
  - `handle_object_selection()`
  - `create_constraint_system()`
  - `handle_deleted_selections()`
  - `update_selection_indicators()`
  - `clear_selections_on_mode_change()`
- Removed `SelectionIndicator` component (only used by Click mode)
- Removed mode toggle button and `handle_connection_mode_button()` system
- Simplified `SelectionState` resource to only track:
  - `constraint_type` (Fixed or Hinge)
  - `material` (Wood, Metal, Rope, or Plastic)
  - `is_enabled` (whether connection mode is active)

### 2. Material Selection as Sub-Options

**Rationale**: Materials should appear as sub-options only when Fixed or Hinge constraint is selected, not always visible in the top bar.

**Changes**:
- Removed material buttons from main top bar in `setup_ui_topbar()`
- Added `MaterialButton` component marker
- Created `spawn_material_buttons()` function that creates a panel below the top bar with material selection
- Updated `sync_selection_with_connection_system()` to:
  - Show material buttons when Fixed or Hinge is selected
  - Hide material buttons when switching to other modes
- Material buttons now appear in a separate panel positioned at top: 70px, showing "Material:" label followed by Wood, Metal, Rope, and Plastic buttons

### 3. Fixed Connection Starting Point

**Rationale**: Connections should start from where the user clicks on the object, not from the object's origin.

**Solution**: The drag mode already correctly implements this:
- In `start_drag_connection()`: Captures the world position where user clicks (`world_pos`) and stores it in `drag_conn_state.start_position`
- In `end_drag_connection()`: 
  - Calculates `anchor1 = start_click_pos - start_body_pos` (local offset from object center to click position)
  - Calculates `anchor2 = end_click_pos - end_body_pos` (local offset for end object)
  - These anchors are used to create the joint at the exact click positions

## Files Modified

1. **src/connection.rs**:
   - Removed ConnectionMode enum and ConnectionModeState resource
   - Removed Click mode systems and components
   - Simplified SelectionState resource
   - Updated system signatures to remove connection_mode parameter
   - Cleaned up comments referencing Mode 1/Mode 2

2. **src/ui_topbar.rs**:
   - Removed material buttons from main setup
   - Added MaterialButton component
   - Removed ConnectionModeButton component
   - Replaced connection mode button with material buttons panel
   - Updated sync_selection_with_connection_system() to spawn/despawn material buttons
   - Removed unused imports (spawn_iron_block_from_ui, spawn_wooden_box_from_ui)

3. **src/main.rs**:
   - Removed ConnectionModeState resource initialization
   - Removed Click mode systems from Update schedule
   - Removed handle_connection_mode_button system
   - Cleaned up imports

## How to Use the New System

1. Select "Fixed (X)" or "Hinge (H)" from the top bar
2. A material selection panel appears below showing: Wood, Metal, Rope, Plastic
3. Select desired material (default: Metal)
4. Drag from one object to another to create the connection
   - Connection starts exactly where you click on the first object
   - Connection ends exactly where you release on the second object
5. To exit connection mode, select any other tool from the top bar

## Material Properties

- **Wood**: Moderate flexibility, moderate strength (compliance: 0.00001, damping: 0.5)
- **Metal**: High strength, rigid (compliance: 0.000001, damping: 0.1) - Default
- **Rope**: High flexibility, weak (compliance: 0.0001, damping: 2.0)
- **Plastic**: Moderate strength and flexibility (compliance: 0.00005, damping: 1.0)

## Testing

- Build successful with `cargo build --release`
- All type checks pass
- Only minor unused code warnings remain (for utility functions)
