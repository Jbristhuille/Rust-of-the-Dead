# Rust of the Dead

A top-down zombie survival shooter built 100% from scratch in Rust.  
No crates. No shortcuts. Just brains.

## 🎯 Objective

Create a fully playable 2D zombie shooter game where the player moves with the keyboard, aims with the mouse, and survives waves of enemies.

## 🧱 Constraints

- No third-party Rust libraries.
- Only the Rust standard library (`std`) is allowed.
- External C/C++ libraries may be used via FFI only if strictly necessary (i.e., not reasonably doable in pure Rust).

## 🛠️ Approach

The game is broken down into technical milestones: window creation, input handling, software rendering, image loading, collision detection, audio playback, and game logic.

Each component is developed and validated independently, then integrated step by step into the final game.

## 🚧 Status

Work in progress.  
The first milestone (`engine_core`) focuses on creating a native Windows window with a stable 60 FPS loop.

## 🪪 License

This project is licensed under the [MIT License](LICENSE).
