# 连接锚点重合常见问题 (Anchor Overlap FAQ)

## ❓ 问题：连接之后，连接材料与连接物体之间重合会不会出问题？

## ✅ 答案：**不会有问题！**

### 简短解释 (Short Explanation)

在物理引擎中，关节（Joints）是**约束条件**，不是实际的物理对象。锚点只是数学上的点，不会占用空间，也不会产生碰撞。

In physics engines, joints are **constraints**, not actual physical objects. Anchors are just mathematical points that don't occupy space or cause collisions.

### 为什么安全？ (Why Is It Safe?)

1. **关节不是物理实体**
   - 关节只定义两个物体之间的约束关系
   - 不会与其他物体碰撞
   
2. **锚点可以在任何位置**
   - 物体表面 ✅
   - 物体内部 ✅
   - 物体边缘 ✅
   - 所有位置都是合法的

3. **Rapier2D 自动处理**
   - 物理引擎会智能管理连接物体之间的碰撞
   - 关节约束优先于碰撞检测

### 代码中的说明 (Code Documentation)

我们在代码中添加了注释：

```rust
// Note: Anchors can be anywhere on/in the objects - the physics engine
// handles this correctly. Joints are constraints, not physical entities,
// so anchor overlap with object geometry is not a problem.
// Rapier2D automatically manages collision between jointed bodies.
```

### 详细分析 (Detailed Analysis)

完整的技术分析请参阅：
- [CONNECTION_ANCHOR_OVERLAP_ANALYSIS.md](CONNECTION_ANCHOR_OVERLAP_ANALYSIS.md)

### 总结 (Summary)

用户可以放心地在物体的任何位置创建连接：
- ✅ 不会出现碰撞问题
- ✅ 不会导致物理不稳定
- ✅ 连接会正常工作

Users can confidently create connections anywhere on objects:
- ✅ No collision issues
- ✅ No physics instability
- ✅ Connections work normally
