# Realistic Shockwave Physics System

## Overview

The explosion system has been upgraded to use realistic shockwave physics instead of simple radial impulse forces. This provides more authentic behavior for bomb explosions, with proper propagation, pressure-based damage, and physically accurate force application.

## Physics Model

### Shockwave Propagation

- **Wave Speed**: 1200 pixels/second (equivalent to ~340 m/s at 100 pixels/meter scale, simulating sound speed)
- **Wave Thickness**: 80 pixels - represents the physical width of the pressure front
- **Peak Pressure**: 80,000 pressure units at the blast origin
- **Propagation**: The shockwave expands outward as a ring, affecting objects only when the wave front reaches them

### Pressure Decay

The pressure decreases with distance using a realistic decay model:

```
pressure = peak_pressure × wave_decay × distance_factor

where:
  wave_decay = 1.0 - (current_radius / max_radius)^1.5
  distance_factor = sqrt(max_radius / distance)
```

This implements an inverse square law approximation, accounting for:
- Energy dispersion over increasing area
- Air resistance and energy dissipation
- Non-linear decay at close range

### Force Application

Unlike simple radial force, the shockwave system considers physical properties:

1. **Mass-dependent acceleration**:
   ```
   impulse_magnitude = pressure × cross_section × delta_time
   ```

2. **Cross-sectional area**:
   - Larger objects experience more force (more area exposed to pressure)
   - Minimum area of 20 pixels to ensure small objects still respond

3. **Torque generation**:
   ```
   torque = random × impulse_magnitude × 0.1 × (1 - distance_factor)
   ```
   - Randomized for realistic tumbling
   - Decreases with distance for stability

### Damage Model

Damage is calculated based on both pressure and induced velocity:

```
base_damage = pressure × 0.0008
velocity_factor = min(impulse / mass, 1000) / 1000
total_damage = base_damage × (1 + velocity_factor × 2)
```

This accounts for:
- **Static pressure damage**: Direct tissue/material damage from overpressure
- **Dynamic pressure damage**: Additional damage from rapid acceleration
- **Mass scaling**: Light objects accelerate more but may cause less structural damage

## Visual Effects

### Shockwave Rings

5 overlapping shockwave rings are spawned with:
- Staggered delays (0.03s apart) for realistic wave front visualization
- Initial offset for wave progression effect
- Exponential expansion with easing function: `1 - (1 - t)³`
- Alpha fade: `(1 - t)^0.7 × 0.6` for realistic dissipation
- Color temperature: Bright white-yellow transitioning to orange

### Explosion Core

Central flash representing the detonation:
- Initial rapid expansion (0-30% of lifetime)
- Slower contraction (30-100% of lifetime)
- Peak scale of 12× for dramatic effect
- High temperature color (white → yellow-orange)

## Comparison with Previous System

| Aspect | Old System | New System |
|--------|-----------|------------|
| Force Application | Instant radial impulse | Time-based wave propagation |
| Pressure Model | Linear distance decay | Inverse square law with wave decay |
| Damage Calculation | Simple distance-based | Pressure + velocity-based |
| Mass Consideration | Same force for all | Scaled by cross-section and mass |
| Visual Accuracy | Basic expanding rings | Multi-layered realistic shockwave |
| Physics Realism | Arcade-style | Simulation-grade |

## Performance

- **Computational Cost**: Slightly higher than radial impulse (per-frame wave expansion check)
- **Memory**: Minimal - one ShockwaveRing component per explosion
- **Optimization**: Wave front culling - only objects in active wave zone are processed

## Configuration

Key parameters in `bomb.rs`:

```rust
let peak_pressure = 80000.0;  // Blast overpressure
spawn_shockwave(&mut commands, position, EXPLOSION_RADIUS, peak_pressure);
```

In `shockwave.rs`:

```rust
wave_speed: 1200.0,       // Propagation velocity
wave_thickness: 80.0,     // Wave front thickness
lifetime: 1.5 seconds,    // Total simulation time
```

## Future Enhancements

Possible improvements:
- Rarefaction phase (negative pressure following overpressure)
- Obstacle interaction (shockwave reflection/diffraction)
- Multiple shock fronts for shaped charges
- Temperature-based damage (thermal effects)
- Debris propulsion by shockwave (currently debris is spawned separately)
