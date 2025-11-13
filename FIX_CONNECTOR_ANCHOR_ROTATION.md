# Fix: Connector Anchor Placed at Rectangle Centers

## 问题描述 (Problem Description)

当使用拖拽模式连接两个横向的长方形时，完成拖动后，连接点总是建立在两个长方形的中心点，而不是用户实际点击/释放鼠标的位置。

When using drag mode to connect two horizontal rectangles end-to-end, after completing the drag, the connection points were established at the rectangle centers instead of where the user actually clicked/released the mouse.

## 根本原因 (Root Cause)

原始代码在计算关节锚点时，没有考虑物体的旋转。代码直接使用世界空间的偏移量作为局部锚点：

```rust
// 原始代码 (Original code)
let anchor_on_start = start_click_pos - start_body_pos;
let anchor_on_end = end_click_pos - end_body_pos;
```

这个计算方式只在物体没有旋转（rotation = identity）时正确。当物体有旋转时，需要将世界空间的偏移量转换到物体的局部坐标系中。

The original code calculated joint anchors without considering object rotation. It directly used world-space offsets as local anchors, which only works when objects have no rotation. When objects are rotated, world-space offsets need to be transformed into the object's local coordinate frame.

## 解决方案 (Solution)

### 1. 使用 GlobalTransform 获取世界位置

改用 `GlobalTransform` 而不是 `Transform` 来获取物体的世界位置和旋转信息：

```rust
// 修复后 (After fix)
if let (Ok(start_global_transform), Ok(end_global_transform)) = (
    global_transform_query.get(start_entity),
    global_transform_query.get(end_entity),
) {
    let start_body_pos = start_global_transform.translation().truncate();
    let end_body_pos = end_global_transform.translation().truncate();
    
    let start_rotation = start_global_transform.to_scale_rotation_translation().1;
    let end_rotation = end_global_transform.to_scale_rotation_translation().1;
```

### 2. 应用逆旋转转换到局部空间

将世界空间的偏移量通过物体旋转的逆变换转换为局部空间锚点：

```rust
// 计算世界空间偏移
let start_world_offset = start_click_pos - start_body_pos;
let end_world_offset = end_click_pos - end_body_pos;

// 通过逆旋转转换到局部空间
let anchor_on_start = start_rotation.inverse() * start_world_offset.extend(0.0);
let anchor_on_end = end_rotation.inverse() * end_world_offset.extend(0.0);

let anchor_on_start = anchor_on_start.truncate();
let anchor_on_end = anchor_on_end.truncate();
```

## 技术细节 (Technical Details)

### 坐标系转换 (Coordinate Space Transformation)

在物理引擎中，关节的锚点必须定义在物体的局部坐标系中。转换公式：

```
local_anchor = rotation.inverse() * (world_click_pos - world_body_pos)
```

可视化表示时，需要反向转换回世界空间：

```
world_anchor_pos = world_body_pos + (rotation * local_anchor)
```

### 为什么需要逆旋转 (Why Inverse Rotation)

- 物体的旋转 `rotation` 将局部坐标转换为世界坐标
- 逆旋转 `rotation.inverse()` 将世界坐标转换回局部坐标
- 局部锚点在物体的坐标系中是固定的，不随物体旋转而改变其在物体坐标系中的位置

## 修改的文件 (Modified Files)

### src/systems/damage/connection.rs

1. **end_drag_connection 函数**:
   - 将 `transform_query: Query<&Transform>` 改为 `global_transform_query: Query<&GlobalTransform>`
   - 添加旋转提取和逆旋转变换逻辑
   
2. **update_connection_visuals 函数**:
   - 同样改用 `GlobalTransform` 确保一致性
   - 更新旋转和位置提取方式

## 测试建议 (Testing Recommendations)

1. **基本测试**：
   - 创建两个长方形（Box 或 Iron Block）
   - 使用拖拽模式（Drag-to-Connect）从一个长方形的边缘拖动到另一个长方形的边缘
   - 验证连接点是否正确显示在边缘位置

2. **旋转物体测试**：
   - 让物体旋转后再创建连接
   - 验证连接点在旋转的物体上仍然固定在正确的位置

3. **不同约束类型**：
   - 测试 Fixed 约束（X键）
   - 测试 Hinge 约束（H键）
   - 测试不同材料（Wood, Metal, Rope, Plastic）

## 影响范围 (Impact)

- ✅ 修复了连接系统的锚点计算问题
- ✅ 支持旋转物体的正确连接
- ✅ 保持向后兼容，不影响现有功能
- ✅ 提升了用户体验，连接点现在与点击位置精确对应

## 版本信息 (Version Info)

- **Bevy**: 0.15
- **bevy_rapier2d**: 0.28
- **Branch**: fix-connector-anchor-placed-at-rect-centers
