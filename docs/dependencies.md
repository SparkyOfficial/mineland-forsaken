# Project Dependencies

## Core Dependencies

### Bevy (`bevy = { version = "0.14", features = ["dynamic_linking"] }`)
The main game engine that provides:
- ECS (Entity Component System) architecture
- Rendering system (2D and 3D)
- Input handling
- Asset management
- UI system
- State management

The `dynamic_linking` feature speeds up compile times during development.

### Bevy Rapier 3D (`bevy_rapier3d = { version = "0.27" }`)
Physics engine integration for:
- Collision detection
- Rigid body dynamics
- Physics simulation
- Spatial queries

### Tokio (`tokio = { version = "1.0", features = ["full"] }`)
Async runtime for:
- Asynchronous operations
- Networking (when implemented)
- Concurrent tasks

### Bevy Asset Loader (`bevy_asset_loader = "0.20"`)
Asset management utilities for:
- Loading assets efficiently
- Asset state management
- Resource preloading

## Development Profile Optimizations

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

These settings optimize dependencies while keeping the main code fast to compile, improving development iteration speed.

## Why These Dependencies?

1. **Bevy** was chosen as the main engine because it's modern, data-oriented, and written in Rust, providing excellent performance and safety.

2. **Bevy Rapier** is the recommended physics engine for Bevy, providing robust 3D physics simulation.

3. **Tokio** is included for future networking implementation, as multiplayer games typically require async operations.

4. **Bevy Asset Loader** helps manage game assets efficiently, which will be important as the game grows in complexity.

This combination provides a solid foundation for a 3D game while keeping the project manageable and performant.