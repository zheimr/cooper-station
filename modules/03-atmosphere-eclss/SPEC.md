# Module 03: Atmosphere & ECLSS

**Status:** 🔴 Open
**Domain:** Environmental Engineering, Chemical Engineering, Biology

---

## Overview

The Environmental Control and Life Support System (ECLSS) maintains a breathable atmosphere, manages water cycling, controls temperature and humidity, and handles waste processing. At the scale of Cooper Station, the ECLSS operates more like a planetary ecosystem than a spacecraft system.

---

## Atmospheric Specifications

| Parameter | Value | Rationale |
|---|---|---|
| Total pressure | 50.65 kPa (0.5 atm) | Reduces hull stress, sufficient for life |
| O₂ partial pressure | 20.26 kPa (40%) | Sea-level equivalent O₂ availability |
| N₂ partial pressure | 30.39 kPa (60%) | Fire suppression, prevents O₂ toxicity |
| Temperature range | 15-30°C | Zoned by area function |
| Relative humidity | 40-70% | Varies by zone |
| CO₂ limit | <1000 ppm | Below health concern threshold |
| Atmospheric mass | ~2.5 × 10⁸ tonnes | Significant shielding contribution |

### Pressure Gradient

Due to rotation, atmospheric pressure increases toward the rim:

```
P(r) = P_axis × exp(ω²r² / (2RT/M))

At axis:  ~40 kPa (thinner air)
At rim:   ~51 kPa (nominal)

Difference is small (~20%) — manageable for transit
```

---

## Air Revitalization System

### CO₂ Removal & O₂ Generation

For 1 million inhabitants:
- CO₂ production: ~1 kg/person/day = 1,000 tonnes/day
- O₂ consumption: ~0.84 kg/person/day = 840 tonnes/day

### Methods (Layered Approach)

1. **Biological (Primary — 80%)**
   - Forests and green spaces: ~100 km² dedicated
   - Agricultural photosynthesis (Module 04 integration)
   - Algae bioreactors for high-efficiency conversion
   - Target: 800 tonnes O₂/day biological generation

2. **Electrolysis (Secondary — 15%)**
   - Water electrolysis: 2H₂O → 2H₂ + O₂
   - Powered by solar arrays (Module 05)
   - Emergency backup capacity

3. **Chemical Scrubbers (Emergency — 5%)**
   - Lithium hydroxide or amine-based scrubbers
   - Activated only during biological system failures
   - 72-hour standalone capacity

---

## Water Recovery System

### Water Budget (1M population)

| Use | Daily Volume | Annual Volume |
|---|---|---|
| Drinking/cooking | 3 L/person = 3,000 m³ | 1.1 × 10⁶ m³ |
| Hygiene | 50 L/person = 50,000 m³ | 1.8 × 10⁷ m³ |
| Agriculture | ~200,000 m³ | 7.3 × 10⁷ m³ |
| Industrial | ~100,000 m³ | 3.6 × 10⁷ m³ |
| **Total daily** | **~353,000 m³** | |

### Recovery Target: 98%+ Closure

```
Recovery Chain:
  Wastewater → Primary filtration → Biological treatment
    → Membrane distillation → UV sterilization → Mineral addition
    → Potable water

  Agricultural runoff → Constructed wetlands → Reservoir return

  Humidity condensate → Collection → Treatment → Reuse

  Industrial water → Specialized treatment per contaminant → Reuse
```

### Water Reserves

- Surface water (lakes, rivers): ~10⁸ m³
- Underground reserves: ~5 × 10⁷ m³
- Ice shield reserves: ~2 × 10⁸ m³ (radiation shielding dual-use)
- Total reserve: ~360 days at full consumption without recycling

---

## Temperature & Climate Control

### Zone-Based Climate

```
Zone              Temperature    Humidity    Purpose
────────────────────────────────────────────────────
Residential       20-25°C        45-55%     Living spaces
Agricultural-A    25-30°C        60-70%     Tropical crops
Agricultural-B    15-20°C        40-50%     Temperate crops
Industrial        18-22°C        30-40%     Manufacturing
Parks/Recreation  18-28°C        50-60%     Outdoor spaces
Medical           21-23°C        45-55%     Healthcare
Central Axis      15-18°C        30-40%     Transport/zero-g
```

### Heat Management

```
Internal heat sources:
- Solar input (through windows): ~2.0 × 10¹² W (controlled by mirrors)
- Metabolic heat (1M people): ~1.0 × 10⁸ W
- Industrial waste heat: ~5.0 × 10⁹ W
- Electrical systems: ~2.0 × 10⁹ W

Heat rejection:
- Radiator panels on exterior: sized for total thermal load
- Operating temperature: exterior radiators at 300-400K
- Required radiator area: ~50 km²
```

---

## Waste Processing

### Solid Waste

```
Input: ~2 kg/person/day = 2,000 tonnes/day

Processing chain:
  Organic → Composting → Agricultural soil amendment (70%)
  Recyclable → Sorting → Reprocessing → Raw materials (20%)
  Non-recyclable → Plasma gasification → Syngas + slag (10%)

Target: 99.5% material recovery
Zero-waste-to-void policy
```

### Trace Contaminant Control

- Catalytic oxidation for VOCs
- Activated carbon filtration
- Photocatalytic air purification
- Continuous atmospheric monitoring (real-time mass spectrometry)

---

## Open Questions

- [ ] Atmospheric stability with weather systems at this scale
- [ ] Microbiome management — preventing pathogenic evolution
- [ ] Fire suppression in half-atmosphere (different flame behavior)
- [ ] Odor management across climate zones
- [ ] Atmospheric leakage rate and makeup requirements
- [ ] Integration of natural water cycle with engineered systems

---

## Contributing

Focus areas:
1. Detailed mass/energy balance calculations
2. Bioreactor design for CO₂/O₂ cycling
3. Water treatment system engineering
4. Climate modeling inside a rotating cylinder
5. Fire safety engineering in modified atmosphere
