# Rusty Ballistics

A small ballistic simulation project built in Rust, with the goal of creating a lightweight physics-driven sandbox for projectile motion and target interaction using Bevy.

## Overview

This project is intended to evolve into a simple but fun ballistic simulator where bullets are fired, affected by gravity and drag, and can interact with a world that contains obstacles, targets, or other objects.

The current implementation is a starting point. The main focus is to keep the code simple, understandable, and extensible while adding gameplay and physics features iteratively.

## Features

Current features in the project:

- Rust project setup
- Basic ballistic simulation structure
- Bullet and world models
- Physics foundation for movement and force calculations
- Bevy integration for rendering and game loop management

## Project structure

```text
rusty-ballistic/
|-- Cargo.toml
|-- src/
|   |-- main.rs
|   |-- models/
|   |   |-- bullet.rs
|   |   |-- mod.rs
|   |   |-- physics.rs
|   |   |-- world.rs
|-- target/
|-- output.txt
|-- readme.md
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

- [x] Rust project created
- [x] Basic project structure defined
- [x] Bullet model created
- [x] World model created
- [x] Physics foundation added
- [x] Bevy dependency configured
- [x] Basic simulation loop setup
- [x] Bullet trail rendering
- [x] Pause, reset flow
- [x] UI for stats / debugging
- [x] Generate a new random bullet when pressing a key
- [x] Collision detection between bullets and objects
- [x] Collision response and impact effects
- [x] Wind
- [ ] Better projectile physics tuning
- [x] Launch project with mouse dragging
- [ ] Multiple "weapon" types or "firing modes"
- [ ] Better camera and scene management
- [ ] Audio feedback

## License

This project is currently a personal learning and development project. 

## Summary

Rusty Ballistics is a small Rust-based simulation project focused on projectile motion and basic game mechanics. The immediate priorities are player-triggered firing, collision logic, and a stronger physics model. The project has a good foundation and plenty of room for expansion into a more complete ballistics sandbox.

