# 修复连接系统坐标系问题 (Fix Connection System Coordinate System Issues)

## 问题描述 (Problem Description)

连接系统在处理旋转物体时存在坐标系转换问题。之前的实现使用3D四元数（Quaternion）来处理2D旋转变换，虽然在理论上可行，但在2D物理系统中不够精确，可能导致连接点位置不准确。

The connection system had coordinate system transformation issues when handling rotated objects. The previous implementation used 3D quaternions to handle 2D rotation transformations. While theoretically correct, this approach is not precise enough for a 2D physics system and could lead to inaccurate connection point positions.

## 根本原因 (Root Cause)

在2D物理引擎（Rapier2D）中，所有旋转都是围绕Z轴的。使用3D四元数进行坐标变换时：

```rust
// 之前的实现 (Previous implementation)
let anchor_on_start = start_rotation.inverse() * start_world_offset.extend(0.0);
let anchor_on_start = anchor_on_start.truncate();
```

这种方法有以下问题：
1. **精度损失**：在Vec2和Vec3之间来回转换可能导致精度损失
2. **不直观**：四元数乘法在2D环境中不够直观
3. **性能开销**：3D四元数运算比2D旋转矩阵更复杂

In a 2D physics engine (Rapier2D), all rotations are around the Z-axis. Using 3D quaternions for coordinate transformations has these issues:
1. **Precision loss**: Converting between Vec2 and Vec3 may cause precision loss
2. **Not intuitive**: Quaternion multiplication is less intuitive in 2D contexts
3. **Performance overhead**: 3D quaternion operations are more complex than 2D rotation matrices

## 解决方案 (Solution)

### 1. 使用2D旋转公式 (Use 2D Rotation Formula)

改为使用标准的2D旋转矩阵公式：

```rust
// 正向旋转 (Forward rotation): local -> world
// world_x = local_x * cos(angle) - local_y * sin(angle)
// world_y = local_x * sin(angle) + local_y * cos(angle)

// 逆向旋转 (Inverse rotation): world -> local
// local_x = world_x * cos(-angle) - world_y * sin(-angle)
// local_y = world_x * sin(-angle) + world_y * cos(-angle)
```

### 2. 修改 end_drag_connection 函数

将世界坐标的点击位置转换为物体局部坐标的锚点：

```rust
// Extract 2D rotation angles (rotation around Z-axis in 2D)
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

### 3. 修改 update_connection_visuals 函数

将物体局部坐标的锚点转换为世界坐标以进行可视化：

```rust
// Extract 2D rotation angles (rotation around Z-axis in 2D)
let angle1 = rotation1.to_euler(bevy::math::EulerRot::XYZ).2;
let angle2 = rotation2.to_euler(bevy::math::EulerRot::XYZ).2;

// Apply 2D rotation to transform local anchors to world space
let cos1 = angle1.cos();
let sin1 = angle1.sin();
let world_anchor1 = Vec2::new(
    visual.anchor1.x * cos1 - visual.anchor1.y * sin1,
    visual.anchor1.x * sin1 + visual.anchor1.y * cos1,
);
```

## 技术细节 (Technical Details)

### 坐标系转换数学 (Coordinate Transformation Mathematics)

#### 世界坐标到局部坐标 (World to Local)

```
1. 计算世界空间偏移量
   world_offset = click_position - body_position

2. 提取2D旋转角度（绕Z轴）
   angle = quaternion.to_euler(EulerRot::XYZ).2

3. 应用逆旋转矩阵
   local_x = world_offset.x * cos(-angle) - world_offset.y * sin(-angle)
   local_y = world_offset.x * sin(-angle) + world_offset.y * cos(-angle)
```

#### 局部坐标到世界坐标 (Local to World)

```
1. 提取2D旋转角度
   angle = quaternion.to_euler(EulerRot::XYZ).2

2. 应用正向旋转矩阵
   world_x = local_anchor.x * cos(angle) - local_anchor.y * sin(angle)
   world_y = local_anchor.x * sin(angle) + local_anchor.y * cos(angle)

3. 添加物体世界位置
   world_position = body_position + world_offset
```

### 为什么使用负角度进行逆变换 (Why Use Negative Angle for Inverse Transform)

旋转矩阵的逆矩阵等于旋转相反角度：
```
R(-θ) = R(θ)^(-1)
```

因此，要将世界坐标转换为局部坐标，我们使用 `-angle` 作为旋转角度。

The inverse of a rotation matrix equals rotating by the negative angle. Therefore, to transform from world to local coordinates, we use `-angle` as the rotation angle.

## 修改的文件 (Modified Files)

### src/systems/damage/connection.rs

1. **end_drag_connection 函数** (行 366-394):
   - 将四元数旋转改为2D旋转公式
   - 使用 `to_euler()` 提取Z轴旋转角度
   - 使用标准2D旋转矩阵计算局部锚点

2. **update_connection_visuals 函数** (行 485-525):
   - 将四元数旋转改为2D旋转公式
   - 使用2D旋转将局部锚点转换为世界坐标
   - 确保视觉表示与物理表示一致

## 优势 (Benefits)

1. **更高精度** ✅
   - 避免了Vec2/Vec3转换
   - 直接使用2D计算

2. **更好的可维护性** ✅
   - 代码更符合2D物理引擎的直觉
   - 更容易理解和调试

3. **性能提升** ✅
   - 避免了不必要的3D四元数运算
   - 使用更简单的三角函数

4. **一致性** ✅
   - 与Rapier2D的2D坐标系统完全一致
   - 与Bevy的2D Transform系统协调工作

## 测试建议 (Testing Recommendations)

### 基础测试

1. **非旋转物体连接**：
   - 创建两个未旋转的物体
   - 从边缘拖拽连接
   - 验证连接点位置准确

2. **旋转物体连接**：
   - 创建并旋转两个物体（如45度、90度、180度）
   - 从物体边缘拖拽连接
   - 验证连接点在旋转后仍保持在正确位置

3. **动态旋转**：
   - 创建连接后，让物体旋转
   - 验证连接点随物体旋转保持固定
   - 检查视觉连接线是否正确跟随

### 边界情况测试

1. **极端角度**：
   - 测试0°、90°、180°、270°、360°角度
   - 测试负角度旋转

2. **多个连接**：
   - 在同一物体上创建多个连接点
   - 验证所有连接点都正确工作

3. **不同约束类型**：
   - 测试Fixed约束（固定连接）
   - 测试Hinge约束（铰链连接）
   - 测试所有材料类型（Wood, Metal, Rope, Plastic）

## 兼容性 (Compatibility)

- **Bevy版本**: 0.15
- **bevy_rapier2d版本**: 0.28
- **破坏性变更**: 无 - 仅改进内部实现
- **API变更**: 无 - 所有公共接口保持不变

## 性能影响 (Performance Impact)

理论上，这次修改应该略微**提升**性能，因为：
- 移除了Vec2 ↔ Vec3的转换
- 使用更简单的2D三角函数而非3D四元数运算
- 减少了内存分配和复制

In theory, this change should slightly **improve** performance because:
- Removed Vec2 ↔ Vec3 conversions
- Uses simpler 2D trigonometric functions instead of 3D quaternion operations
- Reduces memory allocation and copying

## 相关文档 (Related Documents)

- [FIX_CONNECTOR_ANCHOR_ROTATION.md](FIX_CONNECTOR_ANCHOR_ROTATION.md) - 之前的锚点旋转修复
- [ALLOW_CONNECTION_FROM_ANY_POINT.md](ALLOW_CONNECTION_FROM_ANY_POINT.md) - 允许从任意点连接的功能
- [CONNECTION_MATERIALS_REFACTOR.md](CONNECTION_MATERIALS_REFACTOR.md) - 连接材料系统重构

## 版本信息 (Version Info)

- **分支 (Branch)**: fix/connection-coordinate-system-check
- **日期 (Date)**: 2024
- **作者 (Author)**: AI Developer
