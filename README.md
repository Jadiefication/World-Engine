<div align="center">

  <img alt="World Engine logo" src=".github/img.png" width="160" height="160" />
  <h1>World-Engine</h1>
  <p>A terminal-based world engine written in Rust for managing entities, players, food, and obstacles.</p>

  <p>
    <a href="https://rust-lang.org"><img alt="Rust" src="https://img.shields.io/badge/rust-2024-blue.svg?logo=rust"></a>
    <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
    <a href="https://github.com/crossterm-rs/crossterm"><img alt="Crossterm" src="https://img.shields.io/badge/dependency-crossterm-orange.svg"></a>
  </p>
</div>

World-Engine is a simulation environment designed to be a flexible foundation for terminal-based games and simulations. It provides:

- **Entity Management**: A flexible entity system for defining world objects.
- **Event Loop**: A robust event-driven architecture using `mpsc` channels.
- **Terminal Rendering**: Efficient terminal manipulation via `crossterm`.
- **World Simulation**: Position tracking and interaction logic between entities.

## Tech Stack

- **Language:** [Rust (Edition 2024)](https://rust-lang.org/)
- **Build System:** [Cargo](https://doc.rust-lang.org/cargo/)
- **Libraries:** [crossterm](https://crates.io/crates/crossterm), [rand](https://crates.io/crates/rand)
- **Target OS:** Any terminal that supports raw mode.

## Project Structure

- `src/main.rs`: The main entry point. Sets up the terminal and initializes the event loop.
- `src/world.rs`: Manages entities and their positions.
- `src/entity.rs`: Defines the `Entity` trait.
- `src/player.rs`: Logic for the player-controlled entity.
- `src/event/`:
  - `event.rs`: Defines the `Event` enum.
  - `listener.rs`: Implements the keyboard event listener thread.

## Get started

### Requirements

- Rust Edition 2024 or later.
- Cargo (Rust's package manager).

### Installation

Clone the repository and build it:

```bash
git clone <repository-url>
cd World-Engine
cargo build
```

### Hello, World-Engine

To run the engine and see it in action:

```bash
cargo run
```

## Commands & Scripts

The project uses standard Cargo commands:

- `cargo build`: Compiles the project.
- `cargo run`: Runs the application.
- `cargo check`: Quickly checks the code for compilation errors.
- `cargo doc --open`: Generates and opens the project documentation.
- `cargo test`: Runs tests (none currently defined).

## Configuration & Env Vars

- **TODO**: Currently, there are no environment variables used by the engine. Configuration options could be added here in the future.

## Principles

#### Event-Driven
The engine uses a dedicated listener thread to capture input events and push them into a channel, ensuring a responsive and non-blocking simulation loop.

#### Modular
Entities are defined via a trait system, allowing easy addition of new types like food, obstacles, or AI-controlled agents.

#### Terminal-First
Designed specifically for terminal environments, leveraging raw mode for fine-grained control over input and rendering.

## Documentation

Core entry points:

- `src/main.rs` — Main loop and initialization.
- `src/world.rs` — World management logic.
- `src/event/listener.rs` — Input handling logic.

## Testing

Automated tests are not yet implemented.
- Run tests: `cargo test`

## Contributing

We welcome contributions! Please feel free to open issues or submit pull requests.

## License

[MIT](LICENSE) — © 2026 Jadiefication
