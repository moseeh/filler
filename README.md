# Filler

An intelligent AI player for the Filler algorithmic game, developed as part of the Zone01 Kisumu curriculum. This project implements advanced strategies including heat mapping and territory control to compete against various bots, including the challenging **terminator** bot.

## Table of Contents

- [About the Game](#about-the-game)
- [Installation](#installation)
- [Usage](#usage)
- [Building and Running](#building-and-running)
- [Testing Against Bots](#testing-against-bots)
- [Visualization (Optional)](#visualization-optional)
- [Algorithm Strategy](#algorithm-strategy)
- [Project Structure](#project-structure)
- [Features](#features)
- [Game Engine Flags](#game-engine-flags)
- [Notes](#notes)
- [Contributing](#contributing)

## About the Game

Filler is a competitive algorithmic game where two players (bots) compete on a rectangular grid called the _Anfield_. Each player must place randomly generated pieces on the board with the constraint that exactly **one cell of the new piece must overlap with their existing territory**.

### Game Rules

- Two players alternate turns placing pieces.
- Each piece must overlap **exactly one cell** with the player's territory.
- Players cannot overlap opponent pieces.
- The game ends when a player cannot place a piece.
- The player with the largest controlled area wins.

### Player Symbols

- **Player 1**: `@` (territory), `a` (last piece)
- **Player 2**: `$` (territory), `s` (last piece)

## Installation

```bash
git clone https://github.com/moseeh/filler.git
cd filler
```

### Prerequisites

- **Rust** (latest stable, edition 2024)
- **Docker** (for running the game engine)
- **SDL2 (optional)** – only required for visualization

### SDL2 Setup (Optional)

If you want visualization support:

#### macOS

```bash
brew install sdl2
```

#### Ubuntu/Debian

```bash
sudo apt-get install libsdl2-dev
```

#### Arch Linux

```bash
sudo pacman -S sdl2
```

> ⚠️ SDL2 is optional. The AI works perfectly inside Docker without it.

## Usage

### Building the AI Player

**For Docker (no visualization):**

```bash
cargo build --release
```

**For local development with visualization:**

```bash
cargo build --release --features visualizer
```

### Setting up the Game Environment

1. Download and extract the [game engine package](https://assets.01-edu.org/filler/filler.zip)
2. Build the Docker image:

   ```bash
   cd docker_image
   docker build -t filler .
   ```

3. Run the container with your solution mounted:

   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```

### Running Games Inside Docker

```bash
# Against bender bot
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender

# Against terminator bot
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/terminator

# Using different maps
./game_engine -f maps/map02 -p1 solution/target/release/filler -p2 robots/terminator
```

## Building and Running

### Recommended Build (Docker-Compatible)

```bash
cargo build --release
```

This produces an optimized binary at `target/release/filler` that runs inside Docker.

### Development Build with Visualization

```bash
cargo build --release --features visualizer
```

### Running Tests

```bash
cargo test
```

### Logging

The AI generates logs automatically:

- `game_input.log` → raw game engine input
- `ai_decisions.log` → decision-making process

## Testing Against Bots

### Available Bots in Docker

- **bender** (moderate difficulty)
- **terminator** (hardest challenge)
- Other bots included in engine package

### Batch Testing

```bash
# Run 10 quiet games against terminator
for i in {1..10}; do
    ./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/terminator -q
done
```

## Visualization (Optional)

If SDL2 is installed, you can watch games visually:

```bash
cargo build --release --features visualizer
```

Visualization provides:

- Real-time board updates
- Color-coded territories
- Smooth rendering independent of AI logic

⚠️ Visualization is **not supported inside Docker** – only use it locally.

## Algorithm Strategy

This AI uses a **multi-strategy weighted approach**:

### Heat Map Algorithm

- Assigns values to cells based on distance to opponent.
- **High heat** = close to opponent (aggressive).
- **Low heat** = far from opponent.

### Weighted Scoring

1. **Heat Score (100x)** – prioritize aggressive moves.
2. **Blocking Score (20x)** – block opponent paths.
3. **Expansion Score (5x)** – keep territory growth potential.
4. **Piece Efficiency (10x)** – prefer larger impactful pieces.

### Manhattan Distance

Efficient `|x1 - x2| + |y1 - y2|` distance metric.

### Territory Control

- Pushes toward opponent.
- Cuts off expansion.
- Balances growth vs. aggression.

## Project Structure

```
filler/
├── src/
│   ├── main.rs         # Main game loop + input
│   ├── filler_ai.rs    # Core AI logic
│   ├── player.rs       # Player representation
│   ├── piece.rs        # Game piece structure
│   ├── utils.rs        # Utilities & parsers
│   ├── visualizer.rs   # SDL2 visualizer (optional)
│   └── lib.rs          # Library entry
├── Cargo.toml          # Rust config + features
├── README.md           # Documentation
└── solution/           # Mounted build outputs (for Docker)
```

## Features

### Core

- Heat map algorithm for strategy.
- Multi-strategy scoring.
- Logging and debugging tools.
- Docker-compatible builds.

### Visualization (Optional)

- Real-time graphics with SDL2.
- Thread-safe non-blocking renderer.

### Debugging

- Input/output logs.
- Decision scoring details.
- Handles edge cases gracefully.

## Game Engine Flags

- `-f, -file string` → map file
- `-p1, -player1 string` → player 1 AI
- `-p2, -player2 string` → player 2 AI
- `-q, -quiet` → quiet mode
- `-s, -seed int` → random seed
- `-t, -time int` → move timeout (default 10s)
- `-r, -refresh` → refresh rate control

## Notes

- AI must always respond within timeout.
- Invalid moves → immediate loss.
- If no moves exist, return `0 0`.
- Docker builds use **no default features**.
- SDL2 is completely optional.

## Contributing

Contributions focus on:

1. Algorithm efficiency.
2. Strategic depth.
3. Code quality and maintainability.
4. Performance optimization.
5. Docker-first compatibility.
