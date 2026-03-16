# Contributing to Cooper Station

Welcome to the Cooper Station Project. We're building a space habitat — together.

---

## How to Contribute

### 1. Choose Your Module

Browse the 12 core modules in the `modules/` directory. Each module has a `SPEC.md` with current specifications and open questions. Pick one that matches your expertise.

### 2. Types of Contributions

**Engineering Specs** — Detailed technical specifications with calculations. Every number must trace back to a formula or reference.

**Simulations** — Runnable code (Python, JavaScript, Rust, etc.) that models some aspect of the station. Put these in `modules/XX/simulations/`.

**Visualizations** — 3D models, diagrams, schematics, interactive demos. Put these in `modules/XX/visuals/` or `simulations/`.

**Research** — Literature reviews, paper summaries, trade studies. Put references in `modules/XX/references/`.

**Bug Reports** — Found a physics error? Open an issue with the correct calculation.

### 3. Quality Standards

- All physics must show their work (formulas, variables, units)
- Material choices must cite real research
- Simulations must be reproducible with documented dependencies
- No science fiction handwaving — if something isn't feasible, document the gap honestly
- Use SI units everywhere (metric system)

### 4. Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b module-03/water-recycling-system`
3. Make your changes with clear commit messages
4. Ensure all calculations are verified
5. Submit a PR with a description of what you've added and why

### 5. Branch Naming

```
module-XX/feature-name     (module-specific work)
simulation/sim-name        (cross-module simulations)
docs/topic                 (documentation improvements)
fix/description            (corrections to existing specs)
```

---

## Module Ownership

Anyone can claim ownership of a module by opening a PR that adds their name to the module's SPEC.md and demonstrates meaningful initial contribution. Module owners:

- Review PRs for their module
- Maintain the module's SPEC.md
- Coordinate with adjacent module owners on interfaces

---

## Code of Conduct

Be excellent to each other. We're building humanity's future home — leave egos at the airlock.

---

## License

All contributions are under the MIT License.
