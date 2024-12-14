# Crust - Creatures 2 Fan Remake

A fan remake of **Creatures 2**, written in **Rust** using the **Bevy** game engine. This project is an homage to the beloved artificial life simulation game, aiming to bring its rich world and unique mechanics into the modern age with enhanced performance and cross-platform support, leveraging Rust and Bevy's ECS (Entity Component System) for smooth and reliable gameplay.

![Screenshot - Background tiles, rooms and room information](./screenshots/dec-2024/crust2.png)

> **Note**: This project is currently a work-in-progress and unfinished.

## Features (Completed)

| Feature | Completion | Notes |
| -- | -- | -- |
| **Background Tile Rendering** | 90% | Initial implementation works well but probably not the most efficient
| **Room Debug Rendering** | 70% | Not exactly the same as in the original game but fairly close. Currently always active.
| **World Wrapping** | 20% | Rooms and background tiles will correctly display seamlessly, some glitches when crossing the wrap threshold
| **Debug Navigation** | 100% | Move / Zoom using the keyboard (AWSD / XZ) - Probably nothing like the original game
| **Multiple Camera Rendering** | 100% | Support for rendering different views simultaneously. |
| Loading SFC Files | 20%  | Can load rooms
| `s16` Asset Loading | 100% | Fully implemented

## Features (Planned)

- **Artificial Life Simulation**: World simulation of bacteria, light / heat / chemicals / etc.
- **Cross-Platform Compatibility**: Target modern platforms, including Windows, macOS, and Linux.
- **Graphics and Sound**: Support the original graphics and sounds of the game, with optional modern enhancements.
- **Modding Support**: Allow players to create and share custom content by implementing the CAOS language within the project and creating various tooling to help create new content and edit existing content.

## Goals

The ultimate goal of this project is to recreate **Creatures 2** as faithfully as possible while making it accessible to new and returning fans. By building it in Rust with Bevy, I aim to provide a high-performance and robust foundation for the game, making it easier to expand and improve upon in the future.

I do not / have not had much time to dedicate to this project, and I'm working on it in my spare time. It is a labour of love. I have invested *far* less time into this than those involved with [openc2e](https://github.com/OpenC2E/OpenC2E) and this is not a competiting thing that anyone should pay attention to.

## Changelog

- 2024-12-14: Initial commit of the project. World rendering, initial debug rendering, camera support, and room loading.

## Getting Started

To run the current build of the project, you'll need:

1. **Rust Toolchain**: Install the latest stable version of Rust. [Get Rust here](https://www.rust-lang.org/tools/install)
2. **Clone the Repository**:
```bash
git clone https://github.com/robinduckett/crust.git
cd crust
```
3.	Build the Project:
```bash
cargo build
```
4.	Run the Project:

```bash
cargo run
```

## Screenshots

<div style="text-align: center;">
<i>Rooms</i>
<a href="./screenshots/dec-2024/crust2.png"><img src="./screenshots/dec-2024/crust2.png" alt="World Inspector"></a>
</div>

<div style="text-align: center;">
<i>World Inspector</i>
<a href="./screenshots/dec-2024/crust.png"><img src="./screenshots/dec-2024/crust.png" alt="World Inspector"></a>
</div>

Thank you for checking out this project! Let’s bring the world of Creatures 2 back to life! 🐾

Note: Since this is an unfinished project, expect incomplete functionality and bugs.

You must provide your own assets in order to run this project. I will streamline this process in the future.
