#!/bin/bash
# Create GitHub Issues for Cooper Station open modules
# Run after: gh auth login
# Usage: bash scripts/create-issues.sh

set -e

REPO="zheimr/cooper-station"

echo "Creating labels..."
gh label create "module-03" --color "0e8a16" --description "Atmosphere & ECLSS module" --repo "$REPO" 2>/dev/null || true
gh label create "module-04" --color "1d76db" --description "Agriculture & Food Production module" --repo "$REPO" 2>/dev/null || true
gh label create "module-05" --color "d93f0b" --description "Power Systems module" --repo "$REPO" 2>/dev/null || true
gh label create "module-06" --color "5319e7" --description "Radiation Shielding module" --repo "$REPO" 2>/dev/null || true
gh label create "module-07" --color "fbca04" --description "Habitat & Urban Planning module" --repo "$REPO" 2>/dev/null || true
gh label create "module-08" --color "006b75" --description "Transportation Systems module" --repo "$REPO" 2>/dev/null || true
gh label create "module-09" --color "b60205" --description "Communications & Navigation module" --repo "$REPO" 2>/dev/null || true
gh label create "module-10" --color "c2e0c6" --description "Industrial Manufacturing module" --repo "$REPO" 2>/dev/null || true
gh label create "module-11" --color "e99695" --description "Docking Ports & Logistics module" --repo "$REPO" 2>/dev/null || true
gh label create "module-12" --color "0075ca" --description "Command & Control module" --repo "$REPO" 2>/dev/null || true
gh label create "good first issue" --color "7057ff" --description "Good for newcomers" --repo "$REPO" 2>/dev/null || true
gh label create "help wanted" --color "008672" --description "Extra attention is needed" --repo "$REPO" 2>/dev/null || true

echo ""
echo "Creating issues..."

# Module 03: Atmosphere & ECLSS
gh issue create --repo "$REPO" \
  --title "Module 03: Atmosphere & ECLSS — Environmental Control and Life Support" \
  --label "good first issue,help wanted,module-03" \
  --body "$(cat <<'ISSUEEOF'
## Module 03: Atmosphere & ECLSS

**Spec:** [`modules/03-atmosphere-eclss/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/03-atmosphere-eclss/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Environmental Control and Life Support System (ECLSS) maintaining a half-pressure (50.65 kPa) breathable atmosphere with 40% O₂ and 60% N₂ for 1 million inhabitants. This includes biological air revitalization, water recycling at 98%+ closure, temperature/humidity zone control, and waste processing targeting 99.5% material recovery.

The Rust physics engine already implements the pressure gradient model: `P(r) = P₀·exp(ω²r²M/2RT)` and basic ECLSS budgets. This module needs to go deeper.

### Open questions to answer

- **Atmospheric stability with weather systems at this scale** — can you have clouds, rain, wind patterns inside a 4km-radius cylinder?
- **Microbiome management** — preventing pathogenic evolution in a closed ecosystem
- **Fire suppression in half-atmosphere** — different flame behavior at 40% O₂ / 0.5 atm
- **Odor management across climate zones** — airflow patterns in a rotating cylinder
- **Atmospheric leakage rate and makeup requirements** — hull permeability over decades
- **Integration of natural water cycle with engineered systems** — rain harvesting vs. mechanical recycling

### What's already built

- Pressure gradient: exponential model accounting for centripetal "gravity"
- ECLSS budget: O₂ consumption (0.84 kg/person/day), water (25 kg/person/day), CO₂ removal
- Interactive dashboard visualization at [dashboard.html](https://zheimr.github.io/cooper-station/dashboard.html)

### Required expertise

Environmental Engineering, Chemical Engineering, Biology, Atmospheric Science

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/03-atmosphere-eclss/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations

All calculations must be reproducible. Material choices must reference real data. Gaps must be documented as gaps, not glossed over.
ISSUEEOF
)"
echo "  ✓ Module 03 created"

# Module 04: Agriculture & Food Production
gh issue create --repo "$REPO" \
  --title "Module 04: Agriculture & Food Production — Feeding 1 Million People" \
  --label "good first issue,help wanted,module-04" \
  --body "$(cat <<'ISSUEEOF'
## Module 04: Agriculture & Food Production

**Spec:** [`modules/04-agriculture/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/04-agriculture/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Sustaining 1 million inhabitants through integrated food production across multiple gravity zones: hydroponics (150 km²), aeroponics (100 km²), and soil farming (150 km²). Includes livestock integration (aquaculture, insect farming, algae production), specialized crop rotation, and a massive lighting power budget of 3.2×10¹³ W for grow lights.

### Open questions to answer

- **Optimal light spectrum for space-farming crop yields** — LED vs. focused solar
- **Pollination efficiency in 0.9g** — bumblebee behavior, wind pollination alternatives
- **Soil microbial community evolution** in an enclosed system over decades
- **Long-term crop genetic drift** in an isolated population
- **Water quality management** in closed-loop aquaculture at scale
- **Pathogenic disease management** across 400+ km² of crops
- **Labor efficiency** — automation levels needed for 1M-person food system
- **Nutritional completeness** of controlled agriculture diet
- **Emergency food stockpiling** strategy and shelf-life management

### What's already built

- Area allocations per farming method
- Power budget estimates for grow lights
- Basic caloric requirements per capita
- [Live dashboard](https://zheimr.github.io/cooper-station/dashboard.html) with ECLSS resource budgets

### Required expertise

Agricultural Engineering, Botany, Nutrition Science, Controlled Environment Agriculture

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/04-agriculture/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 04 created"

# Module 05: Power Systems
gh issue create --repo "$REPO" \
  --title "Module 05: Power Systems — 60 TW Generation & Distribution" \
  --label "good first issue,help wanted,module-05" \
  --body "$(cat <<'ISSUEEOF'
## Module 05: Power Systems

**Spec:** [`modules/05-power-systems/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/05-power-systems/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

6.0×10¹³ W generation capacity through concentrated solar (500 km² external mirrors + 5,000 km² embedded solar cells) and four 500 MW thermal nuclear reactors. Includes energy storage (batteries, thermal molten salt, flywheels), superconducting distribution grid, and thermal rejection via 58,000 km² radiator panels.

### Open questions to answer

- **Feasibility of large-scale superconducting cable deployment in space**
- **Optimal solar array orientation** for counter-rotating cylinder pair
- **Mirror surface degradation** from cosmic radiation and micrometeorites
- **Nuclear fuel enrichment on-site** — energy cost vs. external supply
- **Thermal storage integration** with industrial process heat recovery
- **Emergency power procedures** if all reactors fail simultaneously
- **Flywheel vs. battery storage trade-offs** at this scale
- **Power fluctuations** from rotating electrical generation equipment

### What's already built

- Total generation/consumption budgets
- Solar + nuclear split calculations
- Radiator panel sizing
- [Live dashboard](https://zheimr.github.io/cooper-station/dashboard.html) with rotational energy calculations

### Required expertise

Electrical Engineering, Nuclear Physics, Solar Energy, Power Systems Engineering

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/05-power-systems/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 05 created"

# Module 06: Radiation Shielding
gh issue create --repo "$REPO" \
  --title "Module 06: Radiation Shielding — Multi-Layer Protection Design" \
  --label "good first issue,help wanted,module-06" \
  --body "$(cat <<'ISSUEEOF'
## Module 06: Radiation Shielding

**Spec:** [`modules/06-radiation-shielding/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/06-radiation-shielding/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Multi-layer passive protection targeting <20 mSv/year average dose through 12m equivalent shielding: 4m lunar regolith, 1m polyethylene, 1m water/ice, 0.5m boron carbide, 0.5m lead, 2m aluminum. Includes radiation shelter in central hub (10,000 capacity) and real-time dosimetry monitoring with 10,000 distributed sensors.

### Open questions to answer

- **Long-term genetic effects** of chronic 3-5 mSv/year exposure over decades
- **Optimal regolith composition** for radiation protection (depth-density trade-off)
- **Secondary neutron production rates** in actual lunar regolith simulants
- **Shielding degradation** from cosmic ray nuclear spallation over 50-year lifecycle
- **Optimization of shelter location** vs. travel time trade-off for population distribution
- **Active shielding cost-benefit** — plasma/magnetic fields vs. passive mass shielding
- **Reproductive health considerations** for multi-generational habitation

### What's already built

- Layer-by-layer shielding composition
- Target dose rate calculations
- Shelter capacity and location
- Mass contribution to overall station mass budget

### Required expertise

Health Physics, Materials Science, Nuclear Engineering, Radiation Biology

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/06-radiation-shielding/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 06 created"

# Module 07: Habitat & Urban Planning
gh issue create --repo "$REPO" \
  --title "Module 07: Habitat & Urban Planning — City Design for 1M Residents" \
  --label "good first issue,help wanted,module-07" \
  --body "$(cat <<'ISSUEEOF'
## Module 07: Habitat & Urban Planning

**Spec:** [`modules/07-habitat/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/07-habitat/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Urban planning for 1 million inhabitants in 350 km² net residential/commercial area at ~2,857 people/km² density. Radial zoning by gravity (rim core urban to axis industrial), 50 km² distributed parks, four geographic districts with specialized functions, and psychological mitigation strategies for enclosed environment adaptation.

### Open questions to answer

- **Optimal day/night cycle length** — 24 hours vs. station rotation period (127s)?
- **Long-term psychological effects** of always-visible curved "horizon"
- **Optimal density for mental health** — is 2,857 people/km² justified?
- **Governance models** for a million-person micro-society
- **Social stratification risk** — gravity zones becoming class divisions
- **Population growth limits** and demographic planning
- **Crime prevention and justice system** for enclosed environment
- **Religious/spiritual practices** adapted for enclosed habitat

### What's already built

- Density and area calculations
- Zone allocation by gravity level
- District layout concept
- [Live dashboard](https://zheimr.github.io/cooper-station/dashboard.html) with human experience tables

### Required expertise

Urban Planning, Architecture, Psychology, Sociology, Governance Design

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/07-habitat/SPEC.md)
2. Pick one or more open questions
3. Research real-world analogues (submarine crews, Antarctic stations, ISS)
4. Submit a PR updating the SPEC with your analysis
ISSUEEOF
)"
echo "  ✓ Module 07 created"

# Module 08: Transportation Systems
gh issue create --repo "$REPO" \
  --title "Module 08: Transportation — Maglev, Elevators & Coriolis Compensation" \
  --label "good first issue,help wanted,module-08" \
  --body "$(cat <<'ISSUEEOF'
## Module 08: Transportation Systems

**Spec:** [`modules/08-transportation/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/08-transportation/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Multi-modal system: maglev trains along 32km axis (50-100 m/s with Coriolis compensation), radial elevators spanning variable gravity zones, spiral ramps, rotating transitions to zero-g hub, emergency evacuation routes, and local pedestrian/bicycle/automated ground transit. Total transportation peak power: 9.0×10¹⁰ W.

### Open questions to answer

- **Optimal maglev speed** — comfort vs. transit time trade-off
- **Coriolis compensation algorithm** validation at high speeds
- **Elevator cable dynamics** under variable gravity loading
- **Emergency evacuation** of 1M people — what's realistic?
- **Gravity transition methods** — rotating floors, gradual tunnels?
- **Traffic congestion modeling** during peak demand
- **Integration with emergency medical response**

### What's already built

- Coriolis acceleration model: `a = 2ωv` (implemented in Rust engine + dashboard)
- Drop trajectory simulation showing deflection in rotating frame
- [Live dashboard](https://zheimr.github.io/cooper-station/dashboard.html) — try the Coriolis simulator

### Required expertise

Mechanical Engineering, Transportation Engineering, Maglev Technology, Coriolis Physics

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/08-transportation/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 08 created"

# Module 09: Communications & Navigation
gh issue create --repo "$REPO" \
  --title "Module 09: Communications & Navigation — Network for 1M People" \
  --label "good first issue,help wanted,module-09" \
  --body "$(cat <<'ISSUEEOF'
## Module 09: Communications & Navigation

**Spec:** [`modules/09-communications/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/09-communications/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Four fiber optic rings (128 km backbone, 4 Pbps capacity) with mesh topology redundancy, distributed WiFi, 1 Mbps deep-space link to Earth (4-minute round-trip), optical positioning system with 40 laser beacons (10-30 cm accuracy), and cybersecurity with air-gapped critical systems. Supports 26.5 Mbps peak per-capita bandwidth.

### Open questions to answer

- **Deep-space communications** — optimal radio frequency vs. interference management
- **Quantum-secured protocols** for sensitive data
- **4-minute round-trip delay** — psychological effects on Earth communication
- **Privacy vs. security** in ubiquitous positioning system
- **Long-term data storage** reliability over 50+ year mission
- **Network congestion during emergencies** — preventing system overload
- **Language diversity** — translating 20+ languages in real-time

### What's already built

- Bandwidth allocation per capita
- Backbone topology design
- Positioning system concept

### Required expertise

Telecommunications Engineering, Network Science, Astrodynamics, Cybersecurity

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/09-communications/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 09 created"

# Module 10: Industrial Manufacturing
gh issue create --repo "$REPO" \
  --title "Module 10: Industrial Manufacturing — Zero-G Production & Recycling" \
  --label "good first issue,help wanted,module-10" \
  --body "$(cat <<'ISSUEEOF'
## Module 10: Industrial Manufacturing & Processing

**Spec:** [`modules/10-industrial/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/10-industrial/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Gravity-dependent manufacturing in low-g zones producing \$500B annual value: pharmaceuticals, specialty alloys, and optics. Zero-g protein crystal growth, foam materials, precision assembly. Asteroid ore processing (100k tonnes/year), 95% recycling closure (metals, organics, plastics), and additive manufacturing. Peak industrial power: 6.0×10¹⁰ W.

### Open questions to answer

- **Cost-benefit of space manufacturing** vs. Earth production
- **Worker safety** in high-temperature, toxic manufacturing environments
- **Facility automation** to reduce labor (currently assumes 50,000 workers)
- **Environmental contamination** from industrial processes — closed-loop assumption valid?
- **Optimal product mix** for economic sustainability
- **Cross-contamination prevention** in multi-product recycling systems
- **Supply chain management** for raw materials (mining frequency/reliability)

### What's already built

- Production value estimates
- Gravity-zone allocation for manufacturing types
- Recycling closure targets
- [Live dashboard](https://zheimr.github.io/cooper-station/dashboard.html) with mass breakdown

### Required expertise

Manufacturing Engineering, Materials Science, Industrial Economics, Chemical Processing

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/10-industrial/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 10 created"

# Module 11: Docking Ports & Logistics
gh issue create --repo "$REPO" \
  --title "Module 11: Docking Ports & Logistics — Space Traffic Management" \
  --label "good first issue,help wanted,module-11" \
  --body "$(cat <<'ISSUEEOF'
## Module 11: Docking Ports & Logistics

**Spec:** [`modules/11-docking-ports/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/11-docking-ports/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Non-rotating hub with 24 simultaneous docking ports (10 personnel, 6 cargo, 4 specialized, 4 emergency). Handles 180,000 tonnes/year imports and 1,000-5,000 tonnes exports. Spin-matched rotating section docking, cargo handling with robotic arms/conveyors, emergency lifeboats (10 × 100-person), rescue tugs, and fleet management for 18-20 operational spacecraft.

### Open questions to answer

- **Optimal docking port spacing** for maximum throughput
- **Spin-matching procedure safety** for uncrewed spacecraft
- **Emergency evacuation** of 1M population — feasibility and logistics
- **Supply chain sustainability** — how dependent on Earth resupply?
- **Damage control procedures** for hull breach at docking port
- **Quarantine protocols** for potential alien contamination
- **Population growth/immigration decisions** — ethical frameworks

### What's already built

- Port configuration and capacity
- Import/export volume estimates
- Emergency lifeboat sizing
- Fleet management concept

### Required expertise

Aerospace Engineering, Logistics, Space Operations, Orbital Mechanics

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/11-docking-ports/SPEC.md)
2. Pick one or more open questions
3. Do the math — cite sources, show work
4. Submit a PR updating the SPEC with your calculations
ISSUEEOF
)"
echo "  ✓ Module 11 created"

# Module 12: Command, Control & Operations
gh issue create --repo "$REPO" \
  --title "Module 12: Command & Control — Operations Center for 1M-Person Habitat" \
  --label "good first issue,help wanted,module-12" \
  --body "$(cat <<'ISSUEEOF'
## Module 12: Command, Control & Operations

**Spec:** [`modules/12-command-control/SPEC.md`](https://github.com/zheimr/cooper-station/blob/main/modules/12-command-control/SPEC.md)
**Status:** Open — needs contributor to develop calculations and design

### What this module covers

Central operations center in non-rotating hub managing all critical systems with 500 full-time staff across 7 divisions (Structure, ECLSS, Power, Transportation, Communications, Manufacturing, Habitat). 100,000 sensors provide real-time telemetry (1 PB/year) feeding automated control systems with tiered automation (human authority for Levels 1-3, emergency autonomy at Level 4).

### Open questions to answer

- **Optimal staffing levels** — are current assumptions reasonable?
- **AI/ML integration** for predictive failure detection — reliability?
- **Command succession procedures** if commander incapacitated
- **Competing system demands** — agriculture vs. power-intensive manufacturing
- **Governance structure** — direct democracy vs. representative for 1M people
- **Cybersecurity for control systems** — vulnerability to external attack
- **Automated decision authority limits** — where does AI governance stop?
- **Recovery procedures** after cascading multiple-system failure
- **Long-term data archiving** for 50-100 year operational history

### What's already built

- Division structure and staffing
- Sensor network architecture
- Automation level definitions
- Emergency tier classification

### Required expertise

Systems Engineering, Operations Management, AI/Automation, Governance, Cybersecurity

### How to contribute

1. Read the [SPEC.md](https://github.com/zheimr/cooper-station/blob/main/modules/12-command-control/SPEC.md)
2. Pick one or more open questions
3. Research real-world analogues (ISS operations, nuclear plant control rooms, air traffic control)
4. Submit a PR updating the SPEC with your analysis
ISSUEEOF
)"
echo "  ✓ Module 12 created"

echo ""
echo "All 10 issues created successfully!"
echo "View them at: https://github.com/zheimr/cooper-station/issues"
