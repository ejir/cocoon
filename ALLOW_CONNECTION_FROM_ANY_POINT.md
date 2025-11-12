# Allow Connection from Any Block Point

## 问题描述 (Problem Description)
之前的实现只能从物块的中心点（坐标原点）进行连接。现在修改后，可以从物块的任意一点进行连接。

Previously, connections could only be made from the center point (coordinate origin) of blocks. After this modification, connections can be made from any point on a block.

## 实现方案 (Implementation)

### 1. 点击模式 (Click Mode) 修改

#### SelectionState 资源扩展
添加了两个新字段来存储实际的点击位置：
- `first_click_position: Option<Vec2>` - 第一次点击的世界坐标
- `second_click_position: Option<Vec2>` - 第二次点击的世界坐标

#### handle_object_selection 函数修改
当用户点击物块时，不仅保存选中的实体，还保存鼠标实际点击的世界坐标位置：
```rust
selection_state.first_click_position = Some(world_pos);
selection_state.second_click_position = Some(world_pos);
```

#### create_constraint_system 函数修改
创建关节时，使用实际的点击位置而不是物体中心：
```rust
// 之前 (Before)
let midpoint = (first_pos + second_pos) / 2.0;
let anchor1 = midpoint - first_pos;
let anchor2 = midpoint - second_pos;

// 之后 (After)
let anchor1 = first_click_pos - first_pos;
let anchor2 = second_click_pos - second_pos;
```

### 2. 拖拽模式 (Drag Mode) 修改

#### start_drag_connection 函数修改
开始拖拽时，保存鼠标实际点击的世界坐标：
```rust
// 之前 (Before)
let pos = transform.translation.truncate();
drag_conn_state.start_position = pos;

// 之后 (After)
drag_conn_state.start_position = world_pos;
```

#### update_drag_connection 函数修改
更新可视化连接线时，从保存的点击位置绘制：
```rust
// 之前 (Before)
let start_pos = start_transform.translation.truncate();

// 之后 (After)
let start_pos = drag_conn_state.start_position;
```

#### end_drag_connection 函数修改
结束拖拽并创建连接时，使用实际的点击位置计算锚点：
```rust
// 之前 (Before)
let midpoint = (start_pos + end_pos) / 2.0;
let anchor1 = midpoint - start_pos;
let anchor2 = midpoint - end_pos;

// 之后 (After)
let start_click_pos = drag_conn_state.start_position;
let end_click_pos = cursor_pos;
let anchor1 = start_click_pos - start_body_pos;
let anchor2 = end_click_pos - end_body_pos;
```

### 3. 清理函数修改

所有清理选择状态的函数都已更新，确保也清理点击位置信息：
- `clear_selection()` - 清理点击位置字段
- `clear_selections_on_mode_change()` - 切换模式时清理点击位置

## 技术细节 (Technical Details)

### 局部锚点计算 (Local Anchor Calculation)
物理引擎中的关节需要使用局部坐标系的锚点。计算公式：
```rust
local_anchor = world_click_position - body_center_position
```

这样，无论物体如何移动或旋转，关节点都会保持在正确的相对位置上。

### 弹簧关节长度计算 (Spring Joint Length)
对于弹簧关节，静止长度使用两个实际点击位置之间的距离：
```rust
let rest_length = (first_click_pos - second_click_pos).length();
```

## 测试建议 (Testing Recommendations)

1. **点击模式测试**：
   - 点击第一个物块的边缘
   - 点击第二个物块的边缘
   - 按 C 键创建连接
   - 验证连接点是否在点击的位置

2. **拖拽模式测试**：
   - 从第一个物块的边缘开始拖拽
   - 拖拽到第二个物块的边缘
   - 释放鼠标创建连接
   - 验证连接点是否在正确的位置

3. **不同约束类型测试**：
   - 测试固定关节 (Fixed Joint)
   - 测试铰链关节 (Hinge Joint)
   - 测试弹簧关节 (Spring Joint)

## 兼容性 (Compatibility)
此修改完全向后兼容，不会破坏现有功能。所有现有的连接逻辑都已相应更新。
