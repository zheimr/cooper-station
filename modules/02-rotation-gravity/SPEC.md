# Module 02: Rotation & Gravity

**Status:** 🟡 In Progress
**Domain:** Classical Mechanics, Human Factors Engineering

---

## Overview

This module covers the rotational dynamics of Cooper Station, the artificial gravity system, gravity gradient mapping, Coriolis effects on inhabitants, and the counter-rotation bearing system connecting the twin cylinders.

---

## Rotational Parameters

| Parameter | Value | Formula |
|---|---|---|
| Target gravity (rim) | 9.81 m/s² | Design requirement |
| Radius (inner surface) | 3,980 m | R_inner |
| Angular velocity (ω) | 0.04964 rad/s | ω = √(g/r) |
| Rotation rate | 0.474 RPM | RPM = ω × 60/(2π) |
| Period | 126.6 seconds | T = 2π/ω |
| Rim velocity | 197.6 m/s | v = ωr (711 km/h) |
| Rotational energy | ~2.8 × 10¹⁸ J | E = ½Iω² |

---

## Gravity Gradient Map

Gravity scales linearly with radius. This creates distinct gravity zones:

```
Radius (m)    Gravity (g)    Zone Designation
─────────────────────────────────────────────
4,000         1.00 g         Rim — Primary residential
3,500         0.88 g         Sub-rim — Parks, recreation
3,000         0.75 g         Mid-level — Light commercial
2,500         0.63 g         Upper-mid — Sports facilities
2,000         0.50 g         Transit level
1,500         0.38 g         Low-g industrial
1,000         0.25 g         Manufacturing
500           0.13 g         Assembly/construction
100           0.025 g        Near-zero docking prep
0             0.00 g         Central axis — transport corridor
```

### Architectural Implications

- **Buildings at rim**: Max ~50m tall (gravity varies <1.3% across height)
- **Vertical transport**: Elevators experience changing "weight" — needs variable braking
- **Water behavior**: Surface curves visibly in large reservoirs (>100m)
- **Sports**: Unique physics — thrown objects curve, jump height varies
- **Agriculture**: Different crops optimized for different gravity levels

---

## Coriolis Effects

### The Math

In a rotating reference frame, moving objects experience Coriolis acceleration:

```
a_coriolis = -2(ω × v)

Magnitude: a_c = 2ωv (for radial motion)
```

### Practical Impact at Various Speeds

| Activity | Speed | Coriolis Accel | Deflection per 10m | Noticeable? |
|---|---|---|---|---|
| Walking radially | 1.5 m/s | 0.149 m/s² | 0.33 m | Yes |
| Running radially | 4 m/s | 0.397 m/s² | 0.88 m | Very |
| Elevator (slow) | 2 m/s | 0.199 m/s² | 0.44 m | Yes |
| Elevator (fast) | 10 m/s | 0.993 m/s² | 2.2 m | Significant |
| Thrown ball (up) | 15 m/s | 1.489 m/s² | 3.3 m | Dramatic |
| Dropped object | 4.4 m/s (after 1s) | 0.437 m/s² | — | Visible curve |

### Directional Effects

```
Standing at rim, facing along cylinder axis (spinward to your left):

- Walk "up" (toward axis): Deflected spinward (left)
- Walk "down" (toward rim): Deflected anti-spinward (right)
- Walk spinward: Feel slightly lighter
- Walk anti-spinward: Feel slightly heavier

Spinward/anti-spinward weight variation at walking speed:
Δg ≈ ±2ωv/g ≈ ±3% — subtle but detectable
```

### Human Adaptation

Research suggests:
- Rotation rates < 2 RPM cause minimal motion sickness (0.474 RPM is well within)
- Most people adapt to Coriolis effects within 2-3 days
- Head movements in the rotation plane may cause brief disorientation initially
- At this radius, effects are subtle enough for comfortable daily life

---

## Counter-Rotation System

### Why Two Cylinders?

A single rotating cylinder acts as a gyroscope — it resists changes in orientation. With two counter-rotating cylinders, the angular momenta cancel, allowing the station to be reoriented freely (e.g., to track the Sun).

### Bearing System

```
Cylinder A ←──── Bearing Assembly ────→ Cylinder B
(+ω)              (stationary hub)              (-ω)

Configuration:
- Central non-rotating hub connects both cylinders
- Magnetic bearings (frictionless, no wear)
- Hub houses: docking ports, solar collector optics, communications
- Rod/truss connection at each end cap
```

| Parameter | Value |
|---|---|
| Bearing type | Superconducting magnetic |
| Bearing diameter | ~200 m (hub) |
| Angular momentum per cylinder | ~1.4 × 10²² kg⋅m²/s |
| Net angular momentum | 0 (counter-rotating) |
| Bearing load | Axial + radial combined |

---

## Spin-Up Procedure

The station cannot start at full rotation — it must be gradually spun up:

```
Phase 1: Structural completion (0 RPM)
Phase 2: Atmosphere filling (0 RPM)
Phase 3: Gradual spin-up over ~6 months
  - Rate: +0.003 RPM/day
  - Allows structural settling
  - Personnel can adapt incrementally
Phase 4: Fine-tuning to target RPM
Phase 5: Counter-rotation synchronization
```

### Spin-Up Energy Requirements

```
Moment of inertia (cylinder): I ≈ ½MR²
I ≈ ½ × 8.75 × 10¹² × 4000² = 7.0 × 10¹⁹ kg⋅m²

Rotational KE = ½Iω² = ½ × 7.0 × 10¹⁹ × 0.04964²
             = 8.6 × 10¹⁶ J per cylinder

At 1 GW continuous thrust: ~2.7 years to spin up
At 10 GW: ~100 days
```

---

## Open Questions

- [ ] Bearing lubrication/cooling in vacuum
- [ ] Vibration damping between counter-rotating sections
- [ ] Emergency de-spin procedure
- [ ] Attitude control thrusters placement and fuel
- [ ] Gravity wave effects on large liquid bodies
- [ ] Long-term health effects of 0.9g vs 1.0g
- [ ] Optimal gravity for agriculture vs human habitation

---

## Contributing

Focus areas:
1. Detailed bearing system engineering
2. Coriolis compensation in architecture/infrastructure design
3. Spin-up simulation (structural stress during acceleration)
4. Human factors research on rotation adaptation
5. Sports and recreation physics in rotating frame

---

## References

- O'Neill, G.K. (1976). *The High Frontier: Human Colonies in Space*
- Johnson, R.D. & Holbrow, C. (1977). *Space Settlements: A Design Study* (NASA SP-413)
- Hall, T.W. (1999). *Artificial Gravity and the Architecture of Orbital Habitats* — rotation comfort thresholds
- Lackner, J.R. & DiZio, P. (2005). *Vestibular, Proprioceptive, and Haptic Contributions to Spatial Orientation* — Coriolis sickness research
- Young, L.R. (2000). *Artificial Gravity Considerations for a Mars Exploration Mission* — MIT Man Vehicle Lab
- Stone, R.W. (1973). *An Overview of Artificial Gravity* (NASA SP-314) — rotation rate limits (<2 RPM recommendation)
