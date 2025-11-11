# Ragdoll High-Impact Detachment System

## Overview
Enhanced the ragdoll physics system to make body parts detach more reliably from high-impact forces such as falling from heights and being hit by objects.

## Changes Made

### 1. Enhanced Impact Damage Detection (`detect_impact_damage`)
**Location**: `src/damage.rs`

**Improvements**:
- **Lowered detection threshold**: From 300 to 200 units/sec for more sensitive impact detection
- **Fall damage system**: Added specific detection for vertical falls with downward velocity > 200 units/sec
- **High-velocity multiplier**: Impacts above 500 units/sec apply up to 3x damage multiplier
- **Increased base damage**: Raised from 0.03 to 0.05 per velocity unit
- **Fall damage bonus**: Extra damage (0.08 per unit) for vertical impacts
- **Dynamic blood effects**: More dramatic blood spray for high-velocity impacts (0.5x instead of 0.3x velocity)

**Result**: Body parts now reliably detach when ragdolls fall from significant heights or experience sudden impacts.

### 2. Improved Collision Damage (`collision_joint_damage`)
**Location**: `src/damage.rs`

**Improvements**:
- **Lowered threshold**: From 200 to 150 units/sec for earlier damage triggering
- **Increased base damage**: From 0.05 to 0.08 per velocity unit
- **Mass-based damage**: Heavy objects (mass > 5.0) apply up to 2.5x damage multiplier
- **High-velocity bonus**: Extra damage for collisions above 400 units/sec (0.06 per unit)
- **Enhanced blood effects**: More blood particles for high-velocity collisions (0.4x instead of 0.2x velocity)
- **Added mass query**: Now considers the mass of colliding objects for realistic damage

**Result**: Being hit by heavy objects (like iron blocks) or high-velocity projectiles causes significantly more joint damage.

### 3. More Sensitive Stress Damage (`check_joint_damage`)
**Location**: `src/damage.rs`

**Improvements**:
- **Increased sensitivity**: Stress calculation multipliers increased (0.01→0.015 for linear, 0.1→0.15 for angular)
- **Lowered threshold**: From 5.0 to 3.0 stress units for damage to begin
- **Increased base damage**: From 0.5 to 0.7 per stress unit
- **Extreme stress multiplier**: Stress above 15.0 applies up to 2x damage multiplier
- **Directional blood spray**: Blood particles now use velocity difference for realistic spray direction

**Result**: Violent movements, explosions, and ragdoll flailing cause more joint stress and faster dismemberment.

## Damage Mechanics Summary

### Thresholds
| Damage Source | Old Threshold | New Threshold | Improvement |
|--------------|---------------|---------------|-------------|
| Impact Detection | 300 units/sec | 200 units/sec | 33% more sensitive |
| Collision Damage | 200 units/sec | 150 units/sec | 25% more sensitive |
| Stress Damage | 5.0 stress | 3.0 stress | 40% more sensitive |

### Damage Multipliers
- **High-velocity impacts (>500 u/s)**: Up to 3.0x damage
- **Heavy object collisions (mass >5)**: Up to 2.5x damage  
- **Extreme stress (>15)**: Up to 2.0x damage
- **Fall damage bonus**: +0.08 per unit of vertical velocity change

### Visual Feedback
- Joints fracture (darken) when health drops below 50%
- Blood particle velocity scales with impact severity
- Detached limbs continue as independent physics objects

## Testing Scenarios

To test the enhanced detachment system:

1. **Fall Test**: Spawn a ragdoll high above the ground - limbs should detach on impact
2. **Iron Block Test**: Drop or throw heavy iron blocks at ragdolls - should cause dismemberment
3. **Explosion Test**: Bombs should tear apart nearby ragdolls more reliably
4. **Drag Test**: Violently drag and throw ragdolls - rapid movements should stress joints

## Technical Details

### Blood Particle Improvements
Blood spray now adapts to impact type:
- Standard impacts: 0.3x velocity
- High impacts (>500 u/s): 0.5x velocity  
- Collisions: 0.2x to 0.4x based on velocity
- Stress damage: Uses velocity difference between parts

### Physics Considerations
- All calculations maintain compatibility with Rapier2D physics
- Damage scales realistically with mass and velocity
- System respects existing joint health (100 HP starting value)
- Compatible with existing explosion and combustion systems

## Future Enhancements

Potential improvements for later:
- Directional damage (side impacts vs. head-on)
- Joint-specific health multipliers (weak points)
- Cumulative damage over time (fatigue)
- Visual damage indicators (wounds, bruises)
- Sound effects for limb detachment
