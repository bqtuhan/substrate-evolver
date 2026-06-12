# 🧬 Substrate Evolver

*An emergent artificial-life simulation running in your browser, powered by Rust and WebAssembly.*

[![CI/CD](https://github.com/bqtuhan/substrate-evolver/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/bqtuhan/substrate-evolver/actions/workflows/ci-cd.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)

Watch digital lifeforms navigate a toroidal world, evolve complex neural-driven behaviour, and battle Ice Ages – all inside a modern web browser. The entire simulation runs inside a **Rust → WebAssembly** module, painted on an HTML5 Canvas with zero terminal dependencies.

---

## 🧠 Architectural Highlights

### 10-Input Neural Network Brain
Every agent inherits a tiny feedforward neural network whose weights are encoded in its genome. The network perceives:

| Input | Description |
|-------|-------------|
| 1-2   | Normalised relative position of nearest **food** (toroidal wrap) |
| 3-4   | Normalised relative position of nearest **mate** |
| 5     | Current energy percentage |
| 6     | Boolean **food visibility** flag (-1 / 1) |
| 7     | Boolean **mate visibility** flag |
| 8     | Global **day-phase** signal (sine wave) |
| 9     | Internal **clock oscillator** – allows rhythmic patterns |
| 10    | **Local cell density** – crowding sensing |

Outputs (2 neurons) control movement direction and speed.

### Zero-Allocation Toroidal Collision System
Agents move on a wrapped grid (left ↔ right, top ↔ bottom). When two agents share a cell, a borrow-checker-safe **`split_at_mut`** technique allows predators (speed-3 agents under 30% energy) to steal energy from a victim in constant time, with no extra heap allocations.

### Emergent Evolution
- **Two-point crossover** preserves gene blocks during reproduction.
- **Gaussian mutation** during Ice Ages injects variety when the population stagnates.
- **Shuffled processing order** ensures no agent has a systematic advantage.

### Dynamic Environment
The world is split into an **Oasis** (left half) and **Tundra** (right half) with different food rates and metabolic costs. Every 1200 ticks an **Ice Age** descends, slashing food and doubling age-related energy drain.

---

## 🚀 Quick Start (Web)

### Prerequisites
- [Rust toolchain](https://rustup.rs)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A modern web browser

### Build and Run Locally

```bash
# Clone the repository
git clone https://github.com/bqtuhan/substrate-evolver.git
cd substrate-evolver

# Build the WebAssembly module
wasm-pack build --target web

# Serve the project (choose one)
python3 -m http.server          # then open http://localhost:8000
# or
npx serve .                     # then open http://localhost:3000

```

Open index.html – you'll see an 80×24 toroidal grid populated with agents (coloured circles) and food (green blocks). Use the buttons to pause, speed up, spawn food, or trigger an Ice Age manually.
### 📱 Mobile & Touch Support
The canvas automatically resizes to fit your device. Touch controls work seamlessly on iOS and Android.


## 📦 Repository Structure
```
substrate-evolver/
├── Cargo.toml              # Rust project definition (cdylib)
├── src/
│   ├── lib.rs              # WASM boundary – flat memory buffers for JS
│   ├── config.rs           # Simulation constants
│   ├── i18n.rs             # Localisation (EN / TR)
│   ├── agent/
│   │   ├── brain.rs        # Feedforward neural network
│   │   ├── genetics.rs     # Genome, crossover, mutation
│   │   └── state.rs        # Agent struct (position, energy, age)
│   └── core/
│       ├── mod.rs          # Simulation tick loop
│       ├── environment.rs  # Day/night and Ice Age logic
│       └── substrate.rs    # Toroidal grid, food spawning
├── index.html              # Canvas frontend and JavaScript glue
├── .github/workflows/      # CI/CD pipelines
└── README.md

```
## 📄 License
Licensed under either of MIT License or Apache License 2.0 at your option.
## 🙌 Contributing
Bug reports and feature requests are welcome! Please open an issue using the provided templates, or reach out directly for security concerns (see SECURITY.md).