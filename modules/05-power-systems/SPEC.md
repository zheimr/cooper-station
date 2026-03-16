# Module 05: Power Systems

**Status:** 🟡 In Progress
**Domain:** Electrical Engineering, Nuclear Physics, Solar Energy

---

## Overview

The Power Systems module provides continuous, reliable electrical power to all station systems. With a population of 1 million and intensive life-support requirements, Cooper Station requires multiple redundant power sources: primary solar arrays with external mirrors, nuclear fission backup, and energy storage systems. This module details generation capacity, distribution, storage, and integrated power management.

---

## Power Demand Budget

### Baseline Load by System

| System | Peak Power | Average Power | Notes |
|---|---|---|---|
| ECLSS (Module 03) | 8.0 × 10¹² W | 5.0 × 10¹² W | Air/water/waste processing |
| Agriculture (Module 04) | 3.2 × 10¹³ W | 1.8 × 10¹³ W | Grow lights + climate control |
| Habitat Systems (Module 07) | 2.0 × 10¹² W | 1.2 × 10¹² W | Lighting, heating, HVAC |
| Transportation (Module 08) | 5.0 × 10¹¹ W | 2.0 × 10¹¹ W | Maglev trains, elevators |
| Manufacturing (Module 10) | 4.0 × 10¹² W | 1.5 × 10¹² W | Industrial processes |
| Communications (Module 09) | 5.0 × 10¹⁰ W | 3.0 × 10¹⁰ W | Networking, deep-space |
| Residential (lighting) | 3.0 × 10¹² W | 1.5 × 10¹² W | Home power consumption |
| Medical & Research | 1.0 × 10¹² W | 0.6 × 10¹² W | Hospitals, labs |
| Other Systems | 2.0 × 10¹² W | 1.0 × 10¹² W | Security, admin, services |
| **TOTAL PEAK** | **5.2 × 10¹³ W** | **3.1 × 10¹³ W** | |

### Demand Profile

```
Peak demand (daytime, full agricultural lighting): 5.2 × 10¹³ W
Average demand (24-hour rolling): 3.1 × 10¹³ W
Minimum demand (night cycle, reduced agriculture): 1.8 × 10¹³ W

Variation factor: 2.9× between peak and minimum
Design capacity: 6.0 × 10¹³ W (115% of peak for margin)
```

---

## Solar Power Generation System

### Primary Energy Source: External Solar Array

**Justification:** The station orbits in space with continuous access to solar flux (no atmospheric losses). With counter-rotating cylinders and proper orientation, at least one end is optimally positioned for solar collection at all times.

### Solar Array Configuration

```
Solar constant (1 AU): 1,361 W/m²
Accounting for array angle variation (±30° from ideal): 90% effective
Working solar flux: 1,225 W/m²

Array area required for 6.0 × 10¹³ W:
A = P / (flux × efficiency)
A = 6.0 × 10¹³ W / (1,225 W/m² × 0.28)
A = 1.76 × 10¹¹ m² = 176,000 km²

This is impractically large. Solution: External mirrors concentrate light.
```

### Concentrated Solar Design with External Mirrors

```
Mirror array concept:
- Parabolic mirrors outside hull redirect sunlight to embedded solar cells
- Mirror area: 500 km² (30% concentration ratio)
- Concentrated flux: ~400 W/cm² locally

Embedded solar cells:
- Multi-junction (GaAs/Ge) photovoltaics: 45% efficiency
- Area: 5,000 km² interior embedded arrays
- Total thermal collection area: 15,000 km²

Power output calculation:
  Solar flux (concentrated): 1,225 W/m² × 30 = 36,750 W/m²
  Cell efficiency: 45%
  Useful power: 36,750 × 0.45 = 16,500 W/m² per cell

  Total generation: 16,500 W/m² × 5,000 × 10⁶ m²
                  = 8.25 × 10¹³ W
```

### External Mirror System

```
Mirror specification:
- Material: Aluminum foil substrate (Al deposited on Mylar/Kapton)
- Reflectance: 88% (space-rated aluminum)
- Deployed area: 500 km² = 5 × 10¹¹ m²

Thermal load from mirror absorption:
- Absorbed power: 1,225 W/m² × (1 - 0.88) × 5 × 10¹¹ m²
                = 7.35 × 10¹² W
- Must be radiated to space via thermal radiators
- Radiator panels sized appropriately (Module 01 integration)

Control system:
- Suntracking: Maintains optimal angle (±5° tolerance)
- Dust/debris mitigation: Self-cleaning coatings
- Failure redundancy: Segmented mirror array (loss of one segment = 5% capacity reduction)
```

### Interior Solar Cell Array

```
Photovoltaic cells embedded in interior surface:
- Located on 3 land strips (120° apart)
- Spacing: 2m² for every 1 m² cell (heat dissipation clearance)

Power output by position:
- Optimal zone (sun-facing): 5,000 km² × 45% × 1,225 W/m²
                           = 2.76 × 10¹³ W

- Secondary zones (reflected light): 3,000 km² × 25% × 800 W/m²
                                  = 6.0 × 10¹² W

- Total solar generation: 3.36 × 10¹³ W continuous

Seasonal variation:
- Due to orbital mechanics and mirror orientation: ±8% seasonal
- Design accommodates this with thermal storage
```

---

## Nuclear Fission Backup System

### Justification & Constraints

```
Solar generation: 3.36 × 10¹³ W (adequate but variable)
Peak demand: 5.2 × 10¹³ W
Storage capacity (see below): 2.4 hours maximum

Nuclear backup solves:
1. Peak demand periods (agriculture peak hours)
2. Multi-day cloud-free failures (low probability in space)
3. Long-term thermal energy supply
```

### Reactor Specifications

**Reactor Type:** Small Modular Reactor (SMR) with fast-neutron thermal coupling

```
Number of reactors: 4 × 500 MW thermal
Total thermal capacity: 2,000 MWth = 2.0 × 10¹² Wth

Electrical efficiency: 35% (advanced turbine cycle)
Electrical output: 700 MWe = 7.0 × 10¹¹ We per reactor
Total nuclear electrical: 2.8 × 10¹² We

Purpose:
- Baseload thermal power (heating, industrial processes)
- Electrical backup during peak demand
- Emergency power during solar failures
- Process heat for manufacturing (Module 10)
```

### Reactor Design Parameters

```
Fuel type: HALEU (High-Assay Low-Enriched Uranium, <20% U-235)
Burn time: 7-year cycles
Reactor mass per unit: 300 tonnes
Shielding mass per unit: 500 tonnes per reactor
Total nuclear system mass: ~3,200 tonnes (4 reactors + shielding)

Safety systems:
- Passive cooling (convection-based, no active systems required)
- Containment: 2m steel + 3m radiation shielding
- Emergency shutdown: Multiple independent control rod systems
- Waste handling: Integrated fuel storage (see below)

Reactor siting:
- Located in non-rotating hub (minimal shielding needed at hub center)
- Shielded toward non-habitable zones
- Radiator panels attached to outboard shielding (reuses structure)

Maintenance:
- Refueling every 7 years
- On-site uranium enrichment capability (supports fuel cycles)
```

### Fuel Management

```
Initial fuel load (7-year burn): 40 tonnes HALEU
Fuel enrichment: 5-8% U-235

Annual fuel consumption: 5.7 tonnes
Replacement sourcing:
- Asteroid uranium mining + on-site enrichment facility
- Enrichment cost-energy: ~200 GWh per year
- Accounts for ~0.4% of solar generation
```

### Thermal Power Integration

```
Thermal output (from reactors): 2.0 × 10¹² Wth
Uses:
- Electrical generation (700 MWe via turbines)
- Industrial heating (metallurgy, refining): 5.0 × 10¹¹ Wth
- Residual waste heat → storage or radiated to space

Thermodynamic cycle:
- Reactor coolant: Molten salt (FLiBe or similar, 600°C nominal)
- Heat exchanger to water-steam cycle
- Turbine-generator efficiency: 35%
- Condenser coolant: Radiator fluid at 80°C
```

---

## Energy Storage Systems

### Electrochemical Storage (Batteries)

```
Technology: Lithium iron phosphate (LiFePO₄) + solid-state backups
Capacity target: 2.4 hours × 3.1 × 10¹³ W = 8.6 × 10¹³ Wh = 310 TWh

Storage specification:
- Distributed battery farms across habitat zones
- 20 battery complexes (5 tonnes per MWh capacity)
- Total battery mass: 1.55 × 10⁶ tonnes

Cost-benefit tradeoff:
- Extremely expensive (material cost, replacement cycles)
- ~1.5 × 10¹⁶ USD at current battery prices
- Lifetime: 20 years (requires replacement)

Realistic approach:
- Short-term storage only (15-30 min peaks)
- Actual capacity: 0.5 × 10¹³ Wh (6 GWh equivalent)
- Mass: 50,000 tonnes (distributed)
- Purpose: Load leveling, not full reserve backup
```

### Thermal Storage (Molten Salt Tanks)

```
Concept: Store excess solar thermal energy in molten salt medium
Medium: FLiBe (Lithium fluoride-beryllium fluoride, Tm = 460°C)
Tank locations: Distributed throughout non-rotating hub

Storage capacity:
- Tank volume: 500,000 m³ total
- Salt mass: ~800 kg/m³ = 4.0 × 10⁸ kg
- Heat capacity: 4.2 kJ/(kg⋅K)
- Temperature swing: 600°C → 400°C = 200°C ΔT

Energy stored: 4.0 × 10⁸ kg × 4.2 kJ/(kg⋅K) × 200 K
             = 3.36 × 10¹⁴ J = 9.3 × 10¹⁰ Wh = 93 GWh

Time coverage: 93 GWh / (3.1 × 10¹³ W × 24 h)
             = ~0.3 hours (18 minutes effective)

Better application: Thermal smoothing for industrial processes
- Provides steady heat even when solar varies
- Supports multi-step manufacturing sequences
```

### Mechanical Storage (Flywheel System)

```
Superconducting magnetic bearings allow frictionless rotation
Capacity: 1.0 × 10¹³ Wh total
Stored energy: E = ½Iω² = ½ × M × R² × ω²

Design:
- 10 flywheels, each 100,000 tonnes mass
- Radius: 50m
- Angular velocity: 3,000 RPM

Energy per flywheel: ½ × 10⁵ tonne × (50m)² × (314 rad/s)²
                   = 2.5 × 10¹² J = 0.7 GWh

Total storage: 7 GWh, sufficient for ~8 minutes of peak demand

Advantages in space:
- No atmospheric drag losses
- Magnetic bearings: 99.99% efficiency
- Long cycle life (>1M cycles)
```

### Hybrid Storage System

```
Primary (24/7 baseline): Solar thermal + nuclear (3.36 + 0.28 = 3.64 × 10¹³ W)
                        Covers 117% of average demand

Peak demand (agriculture peak): Nuclear 2.8 × 10¹² We supplemented by
                              Flywheel + battery for 10-15 minutes

Multi-day solar failure: Nuclear runs at 100% thermal capacity
                        Diverts thermal to additional turbine for extra
                        electrical capacity (temporary 20% boost possible)

Failure redundancy:
- Loss of one reactor: Covered by solar + other 3 reactors
- Loss of all solar: 4 reactors provide 54% of average demand (critical services only)
- Loss of all nuclear: Solar covers 108% of average (manageable with storage)
```

---

## Power Distribution Grid

### High-Voltage Transmission

```
Backbone architecture: Superconducting cables (zero resistance loss)
- Operating temperature: 20K (liquid hydrogen cooling)
- Voltage: ±500 kV DC (selected for minimum transmission loss)
- Ring configuration with 4 main distribution hubs (spaced 8 km apart)

Cable mass: 200,000 tonnes for 128 km main ring circuit
Hub diameter: 2 km copper/superconductor buses
Transformer capacity per hub: 2.0 × 10¹³ W

Advantages:
- Zero resistance loss (no I²R heating)
- Compact size (HTS cables 1/10th volume of copper)
- Reliability: Ring topology allows fault isolation
```

### Medium-Voltage Distribution (500 kV → 10 kV)

```
Distributed secondary substations: 100 locations throughout habitat
- Spaced ~3 km apart along cylinder length
- Each substation serves 10,000 km² area
- Power handling: 3.1 × 10¹¹ W per substation

Transformer cooling:
- Helium liquid cooling (no oil contamination risk)
- Passive cooling to radiator panels
- Efficiency: 99.5%
```

### Low-Voltage Distribution (10 kV → 120/240V residential)

```
Tertiary substations: 10,000 locations throughout habitat zones
- Neighborhood-scale distribution (density: 10 per 100 km²)
- Serving 50-100 residents each
- Underground distribution (integrated with water/communications)

Smart grid architecture:
- Real-time power metering (wireless mesh network)
- Automated load balancing across zones
- Demand response (HVAC + industrial loads shift to low-demand hours)
- Redundancy: Every circuit has two independent paths to source
```

### Failure Management

```
Single point of failure: Catastrophic hub damage
Mitigation: Modular reactor placement
- One reactor in hub center for thermal integration
- Three reactors distributed at 0°, 120°, 240° in habitat zones
- If main hub damaged, local reactors provide 21% of electrical power
  (sufficient for survival/emergency protocols)

Circuit breaker protection:
- Automated isolation of faulted sections
- Fault current limited to 10× nominal (prevents equipment damage)
- Manual bypass for emergency loads during repair
```

---

## Power System Mass Budget

| Component | Mass (tonnes) | Notes |
|---|---|---|
| Solar cell arrays (embedded) | 2.0 × 10⁵ | Multi-junction GaAs/Ge cells |
| Mirror arrays (external) | 2.0 × 10⁵ | Aluminum foil structure |
| Nuclear reactors (4 units) | 1.2 × 10³ | Core + pressure vessel only |
| Reactor shielding | 2.0 × 10³ | Lead + boron carbide composite |
| Thermal radiator panels | 5.0 × 10⁵ | Aluminum/copper structure |
| Battery storage system | 5.0 × 10⁴ | LiFePO₄ distributed packs |
| Flywheel system | 1.0 × 10⁶ | 10 × 100kt rotors + bearings |
| Superconducting cables | 2.0 × 10⁵ | Niobium-titanium alloy |
| Distribution transformers | 5.0 × 10⁴ | Copper + iron cores |
| Power conversion equipment | 1.0 × 10⁵ | Inverters, rectifiers, controllers |
| Fuel/coolant (initial) | 1.0 × 10⁴ | Molten salt thermal storage |
| **TOTAL POWER SYSTEM** | **~3.3 × 10⁶** | About 0.04% of station mass |

---

## Thermal Rejection System

### Radiator Design

```
Total heat rejection requirement:
- Solar to space: 200 TW from mirrors
- Nuclear waste heat: 1.3 × 10¹² W
- Industrial processes: 2 × 10¹² W
- HVAC/equipment losses: 5 × 10¹¹ W

Total radiative load: ~2.0 × 10¹² W

Radiator specification:
- Type: Deployable panel radiators (15m × 5m modules)
- Working fluid: Ammonia (high heat of vaporization)
- Hot side temperature: 350K (77°C)
- Cold side temperature: 250K (-23°C)
- Emissivity: 0.9 (anodized aluminum)

Stefan-Boltzmann law: Q = ε σ A T⁴
Required area: A = Q / (ε σ T⁴)
             = 2.0 × 10¹² / (0.9 × 5.67 × 10⁻⁸ × 325⁴)
             = 5.8 × 10⁷ m² = 58,000 km²

Deployment:
- Radiator panels cover external hull surface
- ~7% of exterior area (most used for mirrors/shielding)
- Integrated heat pipes route heat from internal sources
```

### Heat Pipe Network

```
Copper/ammonia heat pipes distributed across all zones
Primary runs: From main heat sources → radiator panels
Secondary runs: From distributed sources → local radiators

Capacity: 2.0 × 10¹² W distributed across 500 × 10³ pipes
Average load per pipe: 4 × 10⁹ W

Heat flux capability: 1,000 W per pipe
Operating against 2 m gravity: Overhead head ~5 m
Pressure drop: Manageable with ammonia properties
```

---

## Open Questions

- [ ] Feasibility of large-scale superconducting cable deployment in space
- [ ] Optimal solar array orientation for counter-rotating cylinder pair
- [ ] Mirror surface degradation from cosmic radiation and micrometeorites
- [ ] Nuclear fuel enrichment on-site: energy cost vs. external supply
- [ ] Thermal storage integration with industrial process heat recovery
- [ ] Emergency power procedures if all reactors fail simultaneously
- [ ] Long-term performance of LiFePO₄ batteries in space thermal environment
- [ ] Optimization between flywheel and battery storage trade-offs
- [ ] Grounding and lightning protection in enclosed space environment
- [ ] Power fluctuations from rotating electrical generation equipment

---

## Contributing

Focus areas:

1. **Solar array optimization**
   - Mirror segmentation and control system design
   - Multi-junction cell performance in concentrated flux
   - Dust mitigation strategies for space mirrors

2. **Nuclear systems engineering**
   - SMR thermal-to-electric conversion
   - Passive cooling system validation
   - On-site uranium enrichment facility design

3. **Energy storage engineering**
   - Superconducting cable system design
   - Flywheel bearing stress analysis
   - Thermal energy storage medium optimization

4. **Power distribution architecture**
   - Fault analysis and protection coordination
   - Load balancing algorithms for demand response
   - Real-time grid monitoring and control systems

5. **Thermal management**
   - Heat pipe network optimization
   - Radiator panel design for space deployment
   - Waste heat recovery for secondary applications
