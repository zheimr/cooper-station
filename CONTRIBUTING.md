# Contributing to Cooper Station

Welcome to the Cooper Station project. We're building a space habitat -- together.

Whether you're an aerospace engineer, a physics student, a Rust developer, or just someone who thinks humanity should have a backup plan, there's meaningful work here for you.

---

## Quick Start

Contributing is a three-step process:

1. **Pick a module** -- browse the 12 modules in [`modules/`](modules/) or check [open issues labeled "good first issue"](https://github.com/cooperstation/cooper-station/labels/good%20first%20issue).
2. **Read its SPEC.md** -- understand the current state, open questions, and where help is needed.
3. **Submit a PR** -- fork the repo, make your changes, and open a pull request.

That's it. No committees, no approval gates. Find something that interests you and start building.

---

## Types of Contributions

### Engineering Specifications

Improve or expand a module's `SPEC.md` with detailed technical specifications, calculations, trade studies, or subsystem designs.
Every number must trace back to a formula or reference.
Put new specifications directly in the relevant module's `SPEC.md`.

### Physics Engine

The simulation engine lives in [`engine/`](engine/) and is written in Rust, compiled to WebAssembly.
Contributions include new physics modules, bug fixes, performance improvements, additional tests, and new WASM exports for the dashboard.

### Dashboard

The interactive web dashboard lives in [`web/`](web/).
You can add new visualization panels, improve existing ones, fix UX issues, or optimize rendering.
The dashboard uses vanilla JavaScript -- no frameworks.

### Simulations

Standalone simulation scripts live in [`simulations/`](simulations/) or in a module's `simulations/` subdirectory.
These model specific aspects of the station (thermal, structural, orbital) and should be reproducible with documented dependencies.

### Documentation

Fix typos, clarify explanations, improve diagrams, add cross-references between modules, or help make the project more approachable.
Small fixes are just as valuable as big features.

### Research

Literature reviews, paper summaries, feasibility analyses, and trade studies.
Put references in the relevant module's `references/` directory.
If you find a paper that changes our assumptions, open an issue.

### Code Review

Reviewing others' pull requests is a first-class contribution.
Check the math, verify units, test the code, and leave thoughtful feedback.

### Bug Reports

Found a physics error? A broken simulation? A dashboard glitch?
Open an issue with as much detail as possible -- and if you can, include the correct calculation.

---

## Quality Standards

### Calculations

- All calculations must show their derivation: formula, variable definitions, substitution, and result.
- Intermediate steps matter. Someone reading your work should be able to follow the reasoning without re-deriving it.
- If you're using an approximation, state it and justify why it's acceptable.

### Units

- Use SI units everywhere: meters, kilograms, seconds, pascals, watts, kelvins.
- Explicitly label units in every table, formula, and inline value.
- If a source uses imperial units, convert and show the conversion.

### Sources and References

- Cite your sources. Prefer peer-reviewed papers, NASA technical reports, and engineering handbooks.
- Include author, title, year, and a URL or DOI when available.
- Wikipedia is fine for background context but should not be the sole source for a design parameter.

### Honesty About Feasibility

- If something isn't feasible with current technology, say so clearly.
- Document gaps, unknowns, and assumptions. Don't gloss over them.
- An honest "we don't know how to solve this yet" is more valuable than a hand-wavy answer.

### Code Quality

- Rust code must pass `cargo test` and `cargo clippy -- -D warnings` with no errors.
- Format Rust code with `cargo fmt` before committing.
- JavaScript should work in modern browsers without a build step.

---

## Pull Request Process

1. **Fork the repository** and clone your fork locally.
2. **Create a feature branch** from `main`:
   ```
   git checkout -b module-03/eclss-improvements
   ```
3. **Make your changes.** Write clear code, show your math, cite your sources.
4. **Ensure CI passes.** Run `cargo test` and `cargo clippy -- -D warnings` for engine changes. Open `web/index.html` locally and verify dashboard changes look right.
5. **Write a clear PR description.** Explain what you changed, why you changed it, and flag any open questions. Use the [PR template](.github/PULL_REQUEST_TEMPLATE.md) -- it will auto-populate when you open a PR on GitHub.
6. **Submit your PR.** Reviewers will check for technical accuracy, code quality, and consistency with the rest of the project.
7. **Respond to feedback.** Most PRs need at least one round of revision. That's normal and expected.

Don't worry about making everything perfect on the first try. A good-faith contribution with clear reasoning will always get a respectful review.

---

## Commit Message Convention

Use this format for commit messages:

```
<type>(<scope>): <description>
```

### Types

| Type | When to use |
|---|---|
| `feat` | Adding new functionality, a new subsystem, or a new module section |
| `fix` | Correcting a calculation error, physics bug, or code defect |
| `docs` | Documentation-only changes (typos, clarifications, diagrams) |
| `refactor` | Restructuring code or specs without changing behavior |
| `test` | Adding or updating tests |
| `ci` | Changes to CI/CD configuration or build scripts |

### Scopes

- `engine` -- Rust physics engine changes
- `module-XX` -- Changes to a specific module (e.g., `module-03`, `module-07`)
- `dashboard` -- Web dashboard changes
- `docs` -- General documentation

### Examples

```
feat(engine): add thermal dynamics simulation module
fix(module-02): correct centripetal acceleration formula
docs(module-05): expand power distribution analysis with solar array sizing
refactor(dashboard): split monolithic render loop into per-panel updates
test(engine): add unit tests for atmospheric pressure calculations
ci: add wasm-pack build step to GitHub Actions
```

Keep the description concise (under 72 characters) and use the imperative mood ("add", not "added" or "adds").

---

## Code Style

### Rust (`engine/`)

- Follow standard Rust conventions.
- Run `cargo fmt` before every commit.
- Use `cargo clippy -- -D warnings` to catch common issues.
- Write doc comments (`///`) for public functions and modules.
- Prefer explicit types over inference for function signatures.

### Markdown (`modules/`, `docs/`)

- Use ATX-style headings (`#`, `##`, `###`).
- Write one sentence per line. This makes diffs cleaner and reviews easier.
- Use fenced code blocks with language hints (` ```rust `, ` ```python `).
- Keep lines under 120 characters where practical.

### HTML / JavaScript (`web/`)

- Use 2-space indentation.
- Vanilla JavaScript only -- no frameworks, no build tools, no npm.
- Use `const` and `let`, never `var`.
- Prefer descriptive variable names over comments explaining cryptic ones.

---

## Branch Naming

```
module-XX/feature-name     (module-specific work, e.g., module-03/water-recycling-system)
engine/feature-name        (physics engine work, e.g., engine/thermal-dynamics)
dashboard/feature-name     (web dashboard work, e.g., dashboard/power-panel)
simulation/sim-name        (cross-module simulations)
docs/topic                 (documentation improvements)
fix/description            (corrections to existing specs or code)
```

---

## Module Contribution Guide

### How to Pick a Module

1. **Browse the modules** in [`modules/`](modules/) and read the [Module Summary](MODULE_SUMMARY.md) for an overview of all 12 modules and their current status.
2. **Check the open issues** -- look for issues tagged with a specific module or "good first issue."
3. **Read the SPEC.md** for any module that interests you. Pay attention to the "Open Questions" section at the bottom -- those are concrete problems waiting for someone to tackle.
4. **Check if someone is already working on it.** Look at open PRs and the module's "Lead" field in its SPEC.md. Coordinate to avoid duplicate effort.

### Structure of a SPEC.md

Every module follows the same structure (defined in [`modules/MODULE_TEMPLATE.md`](modules/MODULE_TEMPLATE.md)):

| Section | What goes here |
|---|---|
| **Overview** | 2-3 paragraphs: what the module covers, why it matters, how it connects to other modules |
| **Specifications** | Key parameters table with values, units, and source/rationale for each |
| **Design** | Detailed engineering description, broken into subsystems |
| **Calculations** | All math, fully derived. Formula, variables, substitution, result |
| **Dependencies** | What this module needs from others, and what others need from it |
| **Mass Budget** | Component-level mass breakdown in kilograms |
| **Power Budget** | Component-level power breakdown in watts, with duty cycles |
| **Open Questions** | Unsolved problems, uncertainties, areas needing research |
| **References** | Numbered list of cited sources |

### Starting a New Module Section

1. Copy the relevant section from [`modules/MODULE_TEMPLATE.md`](modules/MODULE_TEMPLATE.md).
2. Fill in what you know. Leave explicit `[TODO]` markers for what you don't.
3. Add at least one calculation with full derivation.
4. List your sources in the References section.
5. Open a PR.

---

## Module Ownership

Anyone can claim ownership of a module by opening a PR that adds their name to the module's SPEC.md `Lead` field and demonstrates a meaningful initial contribution.

Module owners:

- Review PRs that touch their module
- Keep the module's SPEC.md up to date
- Coordinate with adjacent module owners on interfaces and shared assumptions
- Help newcomers find good first tasks within their module

Ownership is stewardship, not gatekeeping. If a module owner goes inactive, ownership can be transferred.

---

## Code of Conduct

Be excellent to each other.
We're building humanity's future home -- leave egos at the airlock.

Disagreements about engineering tradeoffs are welcome and expected.
Personal attacks are not.
Argue with math, not volume.

---

## License

All contributions are under the [MIT License](LICENSE).
