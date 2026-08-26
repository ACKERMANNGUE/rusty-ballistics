# Rusty Ballistics

A small ballistic simulation project built in Rust, with the goal of creating a lightweight physics-driven sandbox for projectile motion and target interaction using Bevy.

## Overview

This project is intended to evolve into a simple but fun ballistic simulator where bullets are fired, affected by gravity and drag, and can interact with a world that contains obstacles, targets, or other objects.

The current implementation is a starting point. The main focus is to keep the code simple, understandable, and extensible while adding gameplay and physics features iteratively.

## Project structure

```text
rusty-ballistics/
|-- Cargo.toml
|-- readme.md
|-- formulas.md
|-- src/
|   |-- main.rs
|   |-- config.rs
|   |-- assets/
|   |   |-- shapes/
|   |   |   |-- bullets.json
|   |-- collision/
|   |-- components/
|   |-- factories/
|   |-- geometry/
|   |-- loaders/
|   |-- models/
|   |-- rendering/
|   |-- resources/
|   |-- systems/
|-- target/
```

## Tech stack

- Rust
- Cargo
- Bevy
- glam

## Prerequisites

### Windows

1. Install Rust using rustup:

   - Download and run: https://rustup.rs/
   - Restart your terminal after installation

2. Make sure the toolchain is available:

```bash
rustc --version
cargo --version
```

3. Install Visual Studio Build Tools if needed for native dependencies:

   - Install Visual Studio 2022
   - Select "Desktop development with C++"

4. Then run the project from the repository root:

```bash
cargo run
```

### Linux

For Ubuntu or Debian-based systems:

```bash
sudo apt update
sudo apt install build-essential pkg-config libasound2-dev libudev-dev
```

Then verify Rust is installed:

```bash
rustc --version
cargo --version
```

If Rust is not installed yet:

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

Then run:

```bash
cargo run
```

## Getting started

From the project root:

```bash
cargo build
cargo run
```

If you want to run in release mode:

```bash
cargo run --release
```

## Current status checklist

- [x] Rust project scaffolded with Cargo and Bevy
- [x] Basic simulation world and bullet models in place
- [x] Physics foundation with gravity, drag, and fixed-step updates
- [x] Bevy integration for rendering, input, and game loop
- [x] Bullet trail rendering for projectile visualization
- [x] Pause / resume flow with time control
- [x] Stats and debugging UI with egui
- [x] Random bullet generation and bulk spawning controls
- [x] Collision detection between bullets using SAT-based polygon checks
- [x] Collision response and restitution / friction handling
- [x] Wind system with turbulence
- [x] Drag-to-launch bullet mechanic
- [x] Camera movement and zoom controls
- [ ] Angular damping based on shape
- [ ] Static obstacles
- [ ] Target entities
- [ ] Iron Dome or defensive turret mechanic
- [ ] Better projectile physics tuning
- [ ] Multiple firing modes / weapon types
- [ ] Audio feedback

## License

This project is currently a personal learning and development project. 

## Summary

Rusty Ballistics is a small Rust-based simulation project focused on projectile motion and basic game mechanics. The immediate priorities are player-triggered firing, collision logic, and a stronger physics model. The project has a good foundation and plenty of room for expansion into a more complete ballistics sandbox.

