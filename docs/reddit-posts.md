# Cooper Station -- Social Media Post Drafts

Links for reference:
- GitHub: https://github.com/zheimr/cooper-station
- Live Dashboard: https://zheimr.github.io/cooper-station/dashboard.html
- Landing Page: https://zheimr.github.io/cooper-station/

---

## 1. r/space (~25M members)

**Title:** We're designing an open-source O'Neill Cylinder space habitat -- 32km rotating station with real physics engine

**Body:**

We have been working on an open-source design for an O'Neill Cylinder space habitat called Cooper Station. The idea is straightforward: take the Island III concept from Gerard O'Neill's 1976 book and NASA's SP-413 study, and try to build a rigorous, reproducible engineering specification for it -- with actual physics, not hand-waving.

The station is 32 km long and 8 km in diameter, rotating at 0.473 RPM to produce 1g at the rim. It is designed around 12 independent engineering modules covering everything from structural analysis and rotational dynamics to atmosphere/ECLSS, agriculture, power systems, radiation shielding, habitat planning, transportation, communications, industrial manufacturing, docking logistics, and command & control. The interior has about 402 km^2 of habitable area across three of six longitudinal strips, supporting a target population of 1 million people.

The core physics engine is written in Rust and compiles to WebAssembly, so you can actually interact with the calculations in your browser. It covers gravity gradients (g = omega^2 * r across all radii), Coriolis deflection and drop trajectories, atmospheric pressure modeling, and structural hoop stress analysis. There are 29 unit tests and 22 WASM-exported functions. You can play with the live interactive dashboard here: https://zheimr.github.io/cooper-station/dashboard.html

Right now, 10 of the 12 modules are marked "Open" and need contributors. We are looking for aerospace engineers, structural engineers, life scientists (ECLSS and agriculture are wide open), urban planners, and Rust/WASM developers. Every calculation must show its work, cite sources, and document feasibility gaps honestly -- no sci-fi hand-waving allowed.

If you are interested, the full project is on GitHub under MIT license: https://github.com/zheimr/cooper-station. The landing page with a 3D visualization is at https://zheimr.github.io/cooper-station/. Pick a module, read its SPEC.md, do the math, and submit a PR. We would love to have more people working on this.

---

## 2. r/rust (~300K members)

**Title:** Rust physics engine compiled to WASM for an interactive space habitat simulator

**Body:**

I built a Rust physics engine for an open-source O'Neill Cylinder space habitat project and wanted to share the technical details. The engine simulates rotational gravity, Coriolis effects, atmospheric pressure gradients, and structural analysis for a 32 km rotating space station. It compiles to WebAssembly and powers an interactive browser dashboard.

The architecture is five core modules: `station.rs` (core configuration and derived parameters), `gravity.rs` (g = omega^2 * r gradient calculations and zone mapping), `coriolis.rs` (deflection simulation and drop trajectory modeling), `atmosphere.rs` (barometric pressure gradient with rotation, ECLSS budgets), and `structure.rs` (hoop stress calculations, a material library with real alloy data, and mass estimates). The WASM layer in `wasm.rs` exposes 22 functions via `wasm_bindgen` -- things like `gravity_at(radius)`, `coriolis_acceleration(velocity)`, `pressure_at(radius)`, `structural_analysis_json(material, safety_factor)`, `drop_trajectory_json(height, radius, steps)`, and `custom_station_derived_json(length, radius, target_g)` for user-defined station parameters.

The build pipeline uses `wasm-pack build --target web --features wasm`. The `wasm` feature gates the `wasm.rs` module so the native build stays clean. Serialization is handled through `serde` and `serde_json` -- all complex return types are JSON-serialized strings. There are 29 unit tests covering the physics calculations, and `cargo clippy` runs clean with zero warnings. CI runs tests, clippy, and the WASM build on every push via GitHub Actions.

The dashboard (https://zheimr.github.io/cooper-station/dashboard.html) loads the WASM module and calls these exported functions to render 12 interactive panels -- gravity profiles, Coriolis visualizations, atmospheric conditions, structural analysis with different materials, mass breakdowns, and more. All computation happens client-side in the WASM runtime; no backend.

The full source is at https://github.com/zheimr/cooper-station (MIT license). If you are interested in Rust, WASM, physics simulation, or space engineering, contributions are welcome. The engine could use more simulation modules, better numerical methods, and additional WASM exports for the remaining engineering domains. The landing page is at https://zheimr.github.io/cooper-station/.

---

## 3. r/engineering (~500K members)

**Title:** Open-source engineering project: O'Neill Cylinder space habitat -- 10 modules need engineers

**Body:**

We are building an open-source engineering specification for an O'Neill Cylinder space habitat. The goal is to take the concept seriously: real materials, real calculations, documented assumptions, and honest feasibility assessments. The project currently has 12 engineering modules, 10 of which are open for contribution, and a Rust physics engine with 29 tests that compiles to WebAssembly for browser-based interaction.

The station is 32 km long, 8 km diameter, rotating at 0.473 RPM (0.0495 rad/s) to produce 9.81 m/s^2 at the 4,000 m rim. Some of the engineering challenges already under investigation: the structural shell experiences 78.4 MPa hoop stress from centrifugal loading at design conditions (manageable for steel, but the multi-layer composite design with 12 m of radiation shielding drives the total mass to roughly 8.75 billion tonnes). The atmosphere module models a 0.5 atm system (40% O2, 60% N2) with a pressure gradient from rotation. ECLSS needs to handle 1,000 tonnes/day of CO2 removal and 353,000 m^3/day of water recycling at 98%+ closure. The agriculture module has to produce 7.5 x 10^11 kcal/year across 400 km^2 of mixed hydroponic, aeroponic, and soil farming.

The modules that need the most help right now: Atmosphere & ECLSS (Module 03), Agriculture (04), Power Systems (05), Radiation Shielding (06), Habitat & Urban Planning (07), Transportation (08), Communications (09), Industrial Manufacturing (10), Docking Ports (11), and Command & Control (12). Each has a SPEC.md with the current state of calculations and a list of open questions. For example, the power systems module estimates 31 TW average demand with a peak of 52 TW -- mostly driven by agricultural lighting -- served by a combination of solar concentration and nuclear backup. The transportation module specs maglev trains at 100 m/s with 45-degree banking to compensate for Coriolis, plus 500 variable-gravity elevators.

You can explore the interactive physics dashboard at https://zheimr.github.io/cooper-station/dashboard.html -- it runs the actual Rust engine compiled to WASM in your browser, so you can see real-time gravity gradients, Coriolis deflection, atmospheric profiles, and structural analysis with different materials and safety factors.

We need structural engineers, mechanical engineers, chemical/environmental engineers (ECLSS is a massive challenge), electrical engineers (power distribution for a 31 TW system), aerospace engineers, and anyone with domain expertise in the open modules. The quality bar is: show your work, cite your sources, use SI units, and document what is not yet feasible instead of glossing over it. Everything is MIT licensed on GitHub: https://github.com/zheimr/cooper-station.

---

## 4. r/SpaceXLounge (~900K members)

**Title:** Open-source rotating space habitat simulator -- physics engine + interactive dashboard

**Body:**

While SpaceX is working on getting us to Mars, we have been working on the other half of the equation: where do people actually live long-term in space? Cooper Station is an open-source O'Neill Cylinder design project -- a 32 km rotating habitat that produces 1g at the rim through centrifugal force, designed for a population of 1 million. The idea is that habitats like this complement planetary colonization: they can be placed at L5, in Mars orbit, or anywhere else, and they solve the gravity and radiation problems that surface habitats on Mars still face (0.38g and limited magnetic field shielding).

The project has a Rust physics engine that compiles to WebAssembly, powering an interactive dashboard where you can explore the station's physics in your browser: gravity gradients from 1g at the rim to 0g at the central axis, Coriolis effects on moving objects, atmospheric pressure profiles in a rotating reference frame, and structural analysis under centrifugal loading. The engine has 29 tests, 22 WASM exports, and zero clippy warnings. Try it here: https://zheimr.github.io/cooper-station/dashboard.html

The engineering side is organized into 12 independent modules covering structure, rotation/gravity, atmosphere/ECLSS, agriculture, power systems, radiation shielding, habitat planning, transportation, communications, industrial manufacturing, docking, and command & control. The station specs include 402 km^2 of habitable area, 0.5 atm atmosphere, 98%+ water recycling, 400 km^2 of agriculture, and a 31 TW power system driven primarily by solar concentration. The total mass estimate is around 8.75 billion tonnes, most of which is radiation shielding material that could be sourced from asteroid mining -- which is where the Starship-class heavy lift comes in.

10 of the 12 modules are open for contributors. We need people who can do the math on everything from ECLSS closure rates to hoop stress in composite shell structures to Coriolis compensation in maglev transportation systems. Every calculation has to be reproducible, every material choice has to cite real data, and every feasibility gap has to be documented honestly.

If you are interested, the GitHub repo is at https://github.com/zheimr/cooper-station (MIT license) and the landing page with a 3D visualization is at https://zheimr.github.io/cooper-station/. Pick a module, read the SPEC.md, and submit a PR.

---

## 5. r/aerospace

**Title:** Designing a 32km O'Neill Cylinder: open-source physics engine & 12 engineering modules

**Body:**

We are working on an open-source engineering design for an O'Neill Cylinder (Island III) space habitat and could use input from aerospace engineers. The project includes a Rust physics engine with 29 unit tests, 22 WASM-exported functions, and an interactive browser dashboard, plus 12 engineering module specifications with detailed calculations.

The rotational dynamics: 4,000 m radius, angular velocity of 0.0495 rad/s (0.473 RPM), producing 9.81 m/s^2 centripetal acceleration at the rim. Period is 126.9 seconds, rim velocity 198 m/s. The gravity gradient follows g(r) = omega^2 * r, giving us usable zones from microgravity at the axis through 0.1g, 0.38g (Mars-equivalent), 0.5g, and up to 1.0g at the rim. Coriolis acceleration for a radially-moving object is a_c = 2 * omega * v, which at typical walking speeds (1.5 m/s radial) gives about 0.15 m/s^2 -- noticeable but manageable. The engine simulates full drop trajectories showing the curved path of a falling object in the rotating frame.

On the structural side: the cylindrical shell under centrifugal loading experiences hoop stress sigma = rho * omega^2 * R^2, which works out to 78.4 MPa for the baseline steel configuration at 4,000 m radius. The multi-layer wall includes an 8 m structural shell plus 12 m of radiation shielding (regolith, hydrogen-rich polymers, water ice, boron carbide, lead). Total station mass is estimated at 8.75 x 10^9 tonnes. The atmosphere model uses a modified barometric equation for a rotating reference frame, with a 0.5 atm (40% O2, 60% N2) system that shows measurable pressure increase from axis to rim.

The spin-up procedure is specced at 6 months of gradual acceleration, requiring approximately 8.6 x 10^16 J per cylinder. Counter-rotation uses a magnetic levitation bearing system. The station's 24 docking ports are on a 2,000 m diameter non-rotating hub, with transition sections for spin-up/spin-down during ingress/egress.

The interactive dashboard is at https://zheimr.github.io/cooper-station/dashboard.html -- it runs the Rust engine compiled to WASM client-side. You can query gravity at any radius, simulate Coriolis deflection, view atmospheric profiles, and run structural analysis with different materials and safety factors. The full project (MIT license) is at https://github.com/zheimr/cooper-station. 10 of 12 engineering modules are open for contribution -- we especially need help with ECLSS closure modeling, power system architecture (31 TW average demand), and transportation dynamics (Coriolis-compensated maglev at 100 m/s).

---

## 6. Hacker News (Show HN)

**Title:** Show HN: Open-source O'Neill Cylinder -- Rust/WASM physics engine + interactive dashboard

**Body:**

Cooper Station is an open-source O'Neill Cylinder space habitat design project. It has a Rust physics engine that compiles to WebAssembly, powering an interactive dashboard you can use in your browser right now.

The engine simulates a 32 km long, 8 km diameter rotating habitat designed for 1 million people at 1g. It models gravity gradients (g = omega^2 * r from rim to axis), Coriolis effects and drop trajectories in the rotating frame, atmospheric pressure profiles under centrifugal force, and structural hoop stress analysis with real material properties. There are 29 unit tests, 22 WASM-exported functions via wasm-bindgen, and zero clippy warnings. The dashboard calls these functions client-side to render 12 interactive panels.

The project also includes 12 engineering module specifications covering structure, rotation/gravity, atmosphere/ECLSS, agriculture, power systems, radiation shielding, habitat, transportation, communications, industrial manufacturing, docking, and command & control. Each module has a SPEC.md with calculations, open questions, and documented feasibility gaps. The quality standard is: show your work, cite sources, use SI units, and be honest about what is not yet feasible.

Tech stack: Rust engine with serde/serde_json for serialization, wasm-pack for the WASM build (feature-gated behind `--features wasm`), vanilla HTML/JS dashboard that loads the .wasm module, Three.js landing page, GitHub Actions CI (test + clippy + WASM build), GitHub Pages deployment. No framework, no backend.

10 of 12 modules are open for contributors. We need people who can do physics, engineering, or Rust/WASM development. MIT license.

- Dashboard: https://zheimr.github.io/cooper-station/dashboard.html
- Landing page: https://zheimr.github.io/cooper-station/
- GitHub: https://github.com/zheimr/cooper-station
