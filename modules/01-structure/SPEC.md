# Module 01: Structure

**Status:** 🟡 In Progress
**Domain:** Structural Engineering, Materials Science

---

## Overview

The structural module defines the physical hull, internal framework, and material composition of the Cooper Station cylinder. This is the foundation upon which all other modules depend.

---

## Specifications

### Primary Hull

| Parameter | Value | Notes |
|---|---|---|
| Shape | Cylinder | Dual counter-rotating pair |
| Length | 32,000 m | Along central axis |
| Outer Diameter | 8,000 m | At hull exterior |
| Inner Diameter | 7,960 m | Habitable surface |
| Hull Thickness | 20 m | Multi-layer composite |
| Interior Surface Area | ~804 km² | Total cylindrical surface |
| Habitable Surface | ~500 km² | 3 of 6 longitudinal strips |

### Internal Layout

The cylinder interior is divided into 6 longitudinal strips running the full 32 km length:

```
Cross-section (looking down the axis):

        Window Strip 1
       ╱              ╲
      ╱                ╲
Land 3 ┃              ┃ Land 1
      ╲                ╱
       ╲              ╱
        Window Strip 2
       ╱              ╲
      ╱                ╲
Land 2 ┃    AXIS     ┃ Window Strip 3
      ╲                ╱
       ╲              ╱

Each strip: ~133 km² (32 km × ~4.2 km arc)
```

### End Caps

| Parameter | Value |
|---|---|
| Shape | Hemispherical dome |
| Diameter | 8,000 m |
| Material | Reinforced composite with viewport |
| Function | Structural cap + light source (one end) |

---

## Materials

### Hull Composition (Inside → Outside)

1. **Interior Surface Layer** (2m)
   - Soil substrate and vegetation support
   - Drainage and utility conduit layer
   - Material: Engineered regolith composite

2. **Structural Shell** (8m)
   - Primary load-bearing structure
   - Must withstand centrifugal stress: ~38.4 MPa for steel
   - Candidate materials:
     - **Steel alloys** — proven, heavy (TRL 9)
     - **Titanium alloys** — lighter, expensive (TRL 8)
     - **Carbon fiber composites** — best strength-to-weight (TRL 6 at scale)
     - **Carbon nanotube composites** — theoretical best, not yet at scale (TRL 3)

3. **Radiation Shielding Layer** (6m)
   - Lunar regolith fill (primary mass shielding)
   - Hydrogen-rich polymer layers (proton stopping)
   - Water ice layer (dual purpose: shielding + reserve)
   - Target: Reduce radiation to <20 mSv/year

4. **Micrometeorite Protection** (2m)
   - Whipple shield configuration
   - Multi-layer ceramic/kevlar composite
   - Self-healing polymer outer skin

5. **Thermal Control Surface** (2m)
   - Multi-layer insulation (MLI)
   - Radiator panels
   - Active thermal management channels

---

## Structural Analysis

### Centrifugal Loading

```
Hoop stress: σ = ρ × v²
where v = ω × r = 0.0495 × 4000 = 198 m/s

For steel hull (ρ_effective = 2000 kg/m³ average with regolith):
σ = 2000 × 198² = 78.4 MPa

Safety factor of 3: Required yield strength > 235 MPa
Steel: 250-690 MPa yield ✓
Ti-6Al-4V: 880 MPa yield ✓
CFRP: 1500+ MPa tensile ✓
```

### Mass Estimates

| Component | Mass (tonnes) | Notes |
|---|---|---|
| Structural shell | ~2.0 × 10⁹ | Steel baseline |
| Radiation shielding | ~5.0 × 10⁹ | Regolith fill |
| Interior soil/substrate | ~1.0 × 10⁹ | 2m average depth |
| Atmosphere | ~2.5 × 10⁸ | Half-pressure N₂/O₂ |
| Water reserves | ~5.0 × 10⁸ | Lakes, reservoirs, ice |
| **Total estimated** | **~8.75 × 10⁹** | ~8.75 billion tonnes |

### Material Source

At this scale, all materials must be sourced from off-Earth:
- **Lunar regolith**: Primary shielding mass (mass driver launch)
- **Asteroid mining**: Metals, carbon, volatiles
- **In-situ processing**: Refining and manufacturing in space

---

## Open Questions

- [ ] Optimal hull geometry: pure cylinder vs. slightly tapered?
- [ ] End cap structural design: hemisphere vs. flat with truss?
- [ ] Bearing system design for counter-rotating cylinder pair
- [ ] Self-repair mechanisms for micrometeorite damage
- [ ] Thermal expansion management across day/night cycle
- [ ] Seismic/vibration isolation between structural sections

---

## Contributing

If you want to contribute to this module, focus areas include:
1. Finite element analysis of hull under rotation loads
2. Material trade studies with mass/cost optimization
3. Bearing system design for counter-rotation
4. End cap structural analysis
5. Construction sequence planning

---

## References

- O'Neill, G.K. (1976). *The High Frontier: Human Colonies in Space*
- Johnson, R.D. & Holbrow, C. (1977). *Space Settlements: A Design Study* (NASA SP-413)
- ASM International — Materials properties database for Ti-6Al-4V, CFRP, steel alloys
- Toray Industries — Carbon fiber composite datasheets (T700, T1100)
- NASA TRL definitions (Technology Readiness Levels 1-9)
- Hoop stress derivation: σ = ρω²R² for rotating thin-walled cylinders
