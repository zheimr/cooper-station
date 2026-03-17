# Cooper Station Roadmap

## Current State

Cooper Station has a working Rust physics engine (29 tests, 22 WASM exports), 12 engineering module specifications, an interactive browser dashboard, and automated CI/CD. The project has no external contributors yet.

---

## Phase 1: Foundation (Current)

- [x] Core physics engine — gravity, Coriolis, atmosphere, structural analysis
- [x] WASM compilation and browser integration
- [x] 12 module specifications drafted with academic references
- [x] Interactive dashboard with 12 panels
- [x] CI/CD pipeline — test, clippy, WASM build, GitHub Pages deploy
- [x] GitHub Issues for all open modules with labels
- [ ] Complete Module 01 (Structure) and Module 02 (Rotation & Gravity) to full depth
- [ ] First external contributor

## Phase 2: Depth

- [ ] Expand engine: thermal dynamics, power flow simulation, orbital mechanics
- [ ] Advance at least 6 modules to "In Progress" status
- [ ] Add integration tests — cross-module consistency checks (power supply ≥ demand, ECLSS closure rates)
- [ ] 3D visualization of gravity zones in dashboard
- [ ] Property-based testing (`proptest`) for physics engine
- [ ] Performance benchmarks for WASM functions
- [ ] Higher-order numerical integration (RK4) for trajectory simulation

## Phase 3: Community

- [ ] 5+ active contributors
- [ ] Module ownership assignments (maintainers for key modules)
- [ ] Monthly progress updates
- [ ] Enable GitHub Discussions for technical Q&A
- [ ] Contributor recognition in README
- [ ] Conference presentation or paper submission

## Phase 4: Maturity

- [ ] All 12 modules at "In Progress" or "Complete" status
- [ ] Cross-module simulation — end-to-end station model with energy/mass/population balance
- [ ] Peer review of engineering specs by domain experts
- [ ] Published technical report
- [ ] Versioned releases with changelogs
- [ ] External validation against NASA SP-413 and peer literature

---

## How to Help

1. Browse [open issues](https://github.com/zheimr/cooper-station/issues) — look for `good first issue` and `help wanted` labels
2. Pick a module, read its `SPEC.md`, and check the **Open Questions** section
3. See [CONTRIBUTING.md](CONTRIBUTING.md) for quality standards and PR process
