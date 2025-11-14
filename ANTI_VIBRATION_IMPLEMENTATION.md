# Anti-Vibration / Anti-Explosion Complete Implementation

## 问题 / Problem
Physics joints were experiencing vibration and explosion issues due to:
1. Insufficient damping causing oscillation
2. Missing velocity zeroing at connection moment
3. Incorrect stiffness values
4. Potential anchor misalignment

## 解决方案 / Solution

### A. 合理的 Stiffness + Damping (Core Stability)

#### Material Damping Values (Updated)
All materials now have damping > 2.5 for stability:

- **Rope**: 5.0 (soft material, needs very high damping > 3)
- **Wood**: 3.5 (soft material, needs high damping > 3)  
- **Plastic**: 3.0 (soft material, needs high damping > 3)
- **Metal**: 2.5 (rigid material, moderate damping)

#### Stiffness Values (via Compliance)
All stiffness values kept under 600 for stability:

- **Metal**: 500 (compliance = 1/500 = 0.002)
- **Wood**: 400 (compliance = 1/400 = 0.0025)
- **Plastic**: 300 (compliance = 1/300 = 0.00333)
- **Rope**: 150 (compliance = 1/150 = 0.00667)

### B. 锚点对准接触点 (Local Anchor Alignment)

✅ **Already Correctly Implemented**

The code properly calculates local anchors by:
1. Getting world-space click positions
2. Calculating world offset from body center
3. Applying inverse rotation to convert to local space
4. Using precise 2D rotation math (cos/sin)

**Key Code** (connection.rs lines 369-396):
```rust
// Extract 2D rotation angles
let start_angle = start_rotation.to_euler(bevy::math::EulerRot::XYZ).2;
let end_angle = end_rotation.to_euler(bevy::math::EulerRot::XYZ).2;

// Apply inverse 2D rotation to transform world offsets to local space
let cos_start = (-start_angle).cos();
let sin_start = (-start_angle).sin();
let anchor_on_start = Vec2::new(
    start_world_offset.x * cos_start - start_world_offset.y * sin_start,
    start_world_offset.x * sin_start + start_world_offset.y * cos_start,
);
```

### C. 连接瞬间清零速度 (Zero Velocity at Connection)

✅ **CRITICAL - Now Implemented**

**Why Essential**: Hard alignment when bodies have velocity causes explosive forces.

**Implementation** (connection.rs lines 399-408):
```rust
// CRITICAL: Zero out velocities at connection moment to prevent explosion
// This is essential for PPG (People Playground) style physics stability
if let Ok(mut vel) = velocity_query.get_mut(start_entity) {
    vel.linvel = Vec2::ZERO;
    vel.angvel = 0.0;
}
if let Ok(mut vel) = velocity_query.get_mut(end_entity) {
    vel.linvel = Vec2::ZERO;
    vel.angvel = 0.0;
}
```

**Also Applied**:
- All entity spawning now uses `Velocity::zero()` instead of `Velocity::default()`
- Ensures stable initial state for all dynamic bodies

## 修改的文件 / Modified Files

### 1. `src/systems/damage/connection.rs`
- ✅ Updated material damping values (lines 30-36)
- ✅ Updated material compliance/stiffness values (lines 19-26)
- ✅ Added velocity query parameter to `end_drag_connection`
- ✅ Added velocity zeroing before joint creation (lines 399-408)
- ✅ Added comprehensive comments explaining anti-vibration strategy

### 2. `src/entities/ragdoll/body_parts.rs`
- ✅ Added `Velocity::zero()` to body part spawning (line 49)
- ✅ Increased motor_max_force to 350.0 for high damping (line 84)
- ✅ Added comments about anti-vibration configuration

### 3. `src/entities/ragdoll/ragdoll.rs`
- ✅ Added `use bevy_rapier2d::prelude::*` for Velocity support

### 4. `src/entities/obstacles/wooden_box.rs`
- ✅ Changed `Velocity::default()` to `Velocity::zero()` (line 47)

### 5. `src/entities/obstacles/iron_block.rs`
- ✅ Changed `Velocity::default()` to `Velocity::zero()` (line 46)

### 6. `src/entities/weapons/bomb.rs`
- ✅ Added `Velocity::zero()` to bomb spawning (line 43)

### 7. `src/systems/input/drag_create.rs`
- ✅ Changed `Velocity::default()` to `Velocity::zero()` in both spawn functions (lines 153, 181)

## 核心原则总结 / Core Principles Summary

1. **Damping > 2.5**: 极大提高稳定性 / Greatly improves stability
2. **软材料 Damping > 3**: 橡胶、绳索、肉类必须 / Required for rubber, rope, flesh
3. **Stiffness < 600**: 防止过度刚性 / Prevents excessive rigidity
4. **Local Anchors**: 必须精确计算 / Must be precisely calculated
5. **Zero Velocity**: 连接瞬间清零速度 PPG 必做 / Essential PPG requirement
6. **Soft Constraints**: 使用带阻尼的软约束 / Use damped soft constraints

## 测试 / Testing

Run the game and test:
1. Create connections between ragdoll parts - should be stable
2. Create connections while objects are moving - velocity should zero out
3. Different materials should show different stiffness/damping behavior
4. No explosions or vibrations when creating joints
5. Ragdolls should be more stable with improved damping

## 技术细节 / Technical Details

### Motor-Based Damping
Since bevy_rapier2d 0.28 doesn't support direct compliance API, we use:
- `motor_model(MotorModel::ForceBased)` 
- `motor_max_force(damping * 100.0)`

This provides rotational damping for revolute joints, preventing oscillation.

### Why Velocity Zeroing Works
When creating a joint between two moving bodies:
1. Without zeroing: Joint tries to satisfy constraints instantly → explosive forces
2. With zeroing: Bodies start at rest → smooth constraint enforcement → stable joint

This is the **most critical** fix for preventing explosions.
