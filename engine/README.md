# Cooper Engine

Rust-based physics simulation engine for Cooper Station.

## Modules

- **station** — Core station configuration and derived parameters
- **gravity** — Gravity gradient calculations and zone mapping
- **coriolis** — Coriolis effect simulation and trajectory modeling
- **atmosphere** — Atmospheric pressure, density, ECLSS budgets
- **structure** — Structural analysis, material library, mass estimates

## Build

```bash
# Native build
cargo build --release

# Run tests
cargo test

# WASM build (requires wasm-pack)
wasm-pack build --target web
```

## Usage

```rust
use cooper_engine::StationConfig;

let station = StationConfig::new(); // Default Island III config
let derived = station.derive();

println!("RPM: {:.3}", derived.rpm);
println!("Rim gravity: {:.2} m/s²", station.gravity_at_radius(4000.0));

let effect = station.coriolis_effect(1.5); // walking speed
println!("Coriolis: {:.3} m/s² ({:?})", effect.acceleration, effect.perception);
```

## WASM

The engine compiles to WebAssembly for browser-based simulations. Build with the `wasm` feature:

```bash
wasm-pack build --target web --features wasm
```

## License

MIT
