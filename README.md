![CI](https://github.com/zheimr/cooper-station/actions/workflows/ci.yml/badge.svg)
![Pages](https://github.com/zheimr/cooper-station/actions/workflows/pages.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![WASM](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white)

# Cooper Station

O'Neill Cylinder (Island III) space habitat — Rust physics engine, 12 engineering modules, WASM-compiled interactive dashboard.

32 km long, 8 km diameter, 0.473 RPM, 1g at rim. All parameters derived from `a = ω²r`.

**[Live Dashboard](https://zheimr.github.io/cooper-station/dashboard.html)** · **[Landing Page](https://zheimr.github.io/cooper-station/)**

---

## Station Parameters

| Parameter | Value | Derivation |
|---|---|---|
| Radius | 4,000 m | Design choice |
| Length | 32,000 m | Design choice |
| Target gravity | 9.81 m/s² | Earth-equivalent |
| ω | 0.0495 rad/s | √(g/r) |
| RPM | 0.473 | ω × 60/(2π) |
| Period | 126.9 s | 2π/ω |
| Rim velocity | 198 m/s | ωr |
| Habitable area | ~402 km² | πrL (3 of 6 strips) |
| Atmosphere | 40% O₂ / 60% N₂ | 0.5 atm at rim |
| Population | 1,000,000 | Design target |

---

## Engine

Rust physics engine in `engine/`. 29 unit tests, compiles to WASM.

```bash
cd engine
cargo test          # 29 tests
cargo clippy        # zero warnings
wasm-pack build --target web --features wasm
```

Modules: `station.rs` (core params), `gravity.rs` (g = ω²r, zones), `coriolis.rs` (deflection, drop trajectories), `atmosphere.rs` (pressure gradient, ECLSS), `structure.rs` (hoop stress, materials).

WASM API exports 20+ functions: `gravity_at(r)`, `coriolis_acceleration(v)`, `pressure_at(r)`, `structural_analysis_json(material, sf)`, etc.

---

## Modules

12 independent engineering domains. Each has a `SPEC.md` with calculations and open questions.

| # | Module | Status |
|---|---|---|
| 01 | [Structure](modules/01-structure/SPEC.md) | In Progress |
| 02 | [Rotation & Gravity](modules/02-rotation-gravity/SPEC.md) | In Progress |
| 03 | [Atmosphere & ECLSS](modules/03-atmosphere-eclss/SPEC.md) | Open |
| 04 | [Agriculture](modules/04-agriculture/SPEC.md) | Open |
| 05 | [Power Systems](modules/05-power-systems/SPEC.md) | Open |
| 06 | [Radiation Shielding](modules/06-radiation-shielding/SPEC.md) | Open |
| 07 | [Habitat](modules/07-habitat/SPEC.md) | Open |
| 08 | [Transportation](modules/08-transportation/SPEC.md) | Open |
| 09 | [Communications](modules/09-communications/SPEC.md) | Open |
| 10 | [Industrial](modules/10-industrial/SPEC.md) | Open |
| 11 | [Docking Ports](modules/11-docking-ports/SPEC.md) | Open |
| 12 | [Command & Control](modules/12-command-control/SPEC.md) | Open |

---

## Project Structure

```
cooper-station/
├── engine/                  # Rust physics engine
│   ├── src/
│   │   ├── station.rs       # Core config, derived params
│   │   ├── gravity.rs       # Gravity zones, g(r) = ω²r
│   │   ├── coriolis.rs      # Coriolis effects, drop sim
│   │   ├── atmosphere.rs    # Pressure gradient, ECLSS
│   │   ├── structure.rs     # Hoop stress, materials
│   │   └── wasm.rs          # WASM API (20+ exports)
│   └── Cargo.toml
├── modules/                 # 12 engineering specs
├── web/
│   ├── index.html           # Landing page (Three.js)
│   └── dashboard.html       # Interactive physics dashboard
└── .github/workflows/
    ├── ci.yml               # Rust test + clippy + WASM build
    └── pages.yml            # GitHub Pages deploy
```

---

## Contributing

**[Live Site](https://zheimr.github.io/cooper-station/)** — explore the station and pick a module.

1. Pick a module from the table above (10 of 12 are open)
2. Read its `SPEC.md` for current state and open questions
3. Do the math — cite sources, show work
4. Submit a PR

**Who we need:** aerospace engineers, structural engineers, life scientists (ECLSS/agriculture), urban planners, Rust/WASM developers, anyone who can do the math.

Requirements: calculations must be reproducible, material choices must reference real data, gaps in feasibility must be documented as gaps (not glossed over).

---

## References

- O'Neill, G.K. (1976). *The High Frontier: Human Colonies in Space*
- Johnson, R.D. & Holbrow, C. (1977). *Space Settlements: A Design Study* (NASA SP-413)
- NASA ECLSS Technical Brief

---

MIT License
