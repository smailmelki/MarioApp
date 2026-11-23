# MarioApp

A colorful 2D side-scrolling platform adventure built with **Rust** and **Macroquad**.

> This is an original platform-game prototype inspired by classic side-scrolling games. It uses original geometric artwork and does not include Nintendo assets.

## Features

- Side-scrolling level with a follow camera
- Player movement, gravity, jumping and platform collision
- Collectible coins and score tracking
- Moving enemies; jump on them to defeat them
- Three lives, damage cooldown, win and game-over states
- Keyboard controls and on-screen touch controls
- Procedurally drawn graphics: no external assets required

## Controls

| Action | Keyboard | Touch / Mouse |
|---|---|---|
| Move left | A / Left Arrow | Bottom-left button |
| Move right | D / Right Arrow | Bottom-center button |
| Jump | Space / W / Up Arrow | Bottom-right button |
| Restart after finishing | Enter / R | — |

## Run locally

Install Rust using [rustup](https://rustup.rs/), then run:

```bash
git clone https://github.com/smailmelki/MarioApp.git
cd MarioApp
cargo run
```

For an optimized build:

```bash
cargo build --release
```

## Project structure

```text
MarioApp/
├── Cargo.toml
├── README.md
├── .gitignore
└── src/
    └── main.rs
```

## Technology

- Rust 2021
- [Macroquad](https://macroquad.rs/) 0.4

## License

MIT
