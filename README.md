# Filler

An intelligent AI player for the Filler algorithmic game, developed as part of the Zone01 Kisumu curriculum. This project implements advanced strategies including heat mapping and territory control to compete against various bots, including the challenging terminator bot.

## Table of Contents

- [About the Game](#about-the-game)
- [Installation](#installation)
- [Usage](#usage)
- [Algorithm Strategy](#algorithm-strategy)
- [Project Structure](#project-structure)
- [Features](#features)
- [Building and Running](#building-and-running)
- [Testing Against Bots](#testing-against-bots)
- [Visualization](#visualization)
- [Contributing](#contributing)

## About the Game

Filler is a competitive algorithmic game where two players (bots) compete on a rectangular grid called the "Anfield". Each player must place randomly generated pieces on the board, with the constraint that exactly one cell of the new piece must overlap with their existing territory.

### Game Rules

- Two players alternate turns placing pieces on the board
- Each piece must overlap exactly one cell with the player's existing territory
- Players cannot overlap opponent's pieces
- The game ends when a player cannot place any more pieces
- The player occupying the largest area wins

### Player Symbols

- **Player 1**: `@` (territory), `a` (last placed piece)
- **Player 2**: `$` (territory), `s` (last placed piece)

## Installation

```bash
git clone https://github.com/moseeh/filler.git
cd filler
```

### Prerequisites

- Rust (latest stable version)
- Docker (for running the game engine)
- SDL2 (for visualization feature)

### SDL2 Setup

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

## Usage

### Building the AI Player

```bash
cargo build --release
```

### Setting up the Game Environment

1. Download and extract the provided docker_image folder [here](https://assets.01-edu.org/filler/filler.zip)
2. Build the Docker image:
```bash
cd docker_image
docker build -t filler .
```

3. Run the container with your solution mounted:
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Running Games

Inside the Docker container, you can run games against different bots:

```bash
# Against bender bot
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender

# Against terminator bot (the challenge)
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/terminator

# With different maps
./game_engine -f maps/map02 -p1 solution/target/release/filler -p2 robots/terminator
```

## Algorithm Strategy

This AI implementation uses a sophisticated multi-strategy approach:

### Heat Map Algorithm

The core strategy revolves around a heat map that assigns values to each board cell based on its distance to the opponent's territory:

- **High Heat**: Cells close to the opponent (aggressive positioning)
- **Low Heat**: Cells far from the opponent
- **Strategy**: Always try to place pieces on the highest heat cells

### Scoring System

The AI uses a weighted scoring system combining multiple strategies:

1. **Heat Score (100x weight)**: Primary aggressive strategy toward opponent
2. **Blocking Score (50x weight)**: Cut off opponent expansion paths
3. **Expansion Score (10x weight)**: Maintain territory growth potential

### Manhattan Distance

Uses Manhattan distance formula `|x1-x2| + |y1-y2|` for efficient distance calculations between positions.

### Territory Control

- **Enclosure Strategy**: Actively surrounds opponent territory
- **Path Blocking**: Identifies and blocks opponent expansion routes
- **Expansion Optimization**: Considers future expansion potential

## Project Structure

```
filler/
├── src/
│   ├── main.rs              # Main game loop and input parsing
│   ├── filler_ai.rs         # Core AI logic and strategies
│   ├── player.rs            # Player representation
│   ├── piece.rs             # Game piece structure
│   ├── utils.rs             # Utility functions and parsers
│   └── visualizer.rs        # SDL2-based game visualization
├── Cargo.toml               # Rust project configuration
├── README.md                # Project documentation
└── solution/                # Compiled binaries (Docker mount point)
```

## Features

### Core Features

- **Advanced Heat Map Algorithm**: Strategic positioning based on opponent proximity
- **Multi-Strategy Scoring**: Combines aggressive, defensive, and expansion strategies
- **Real-time Logging**: Detailed game state and decision logging
- **Robust Input Parsing**: Handles various board sizes and piece shapes

### Visualization Features

- **Real-time Game Visualization**: Watch games unfold with SDL2-based graphics
- **Color-coded Players**: Different colors for each player's territory
- **Live Updates**: Board updates in real-time as pieces are placed

### Debugging Features

- **Comprehensive Logging**: Game input and AI decisions logged to files
- **Decision Tracking**: Detailed logs of scoring and move selection
- **Error Handling**: Graceful handling of invalid inputs and edge cases

## Building and Running

### Development Build

```bash
cargo build
```

### Optimized Release Build

```bash
cargo build --release
```

### Running with Logging

The AI automatically generates log files:
- `game_input.log`: All game engine input
- `ai_decisions.log`: AI reasoning and decision process

### Testing Locally

```bash
# Run unit tests
cargo test

# Run with specific features
cargo run --features visualization
```

## Testing Against Bots

### Available Bots (in Docker container)

- **bender**: Moderate difficulty bot
- **terminator**: Advanced bot (the main challenge)
- **Other bots**: Various difficulty levels available

### Performance Metrics

Track your AI's performance by running multiple games:

```bash
# Run 10 games against terminator
for i in {1..10}; do
    ./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/terminator -q
done
```

## Visualization

The project includes an optional SDL2-based visualizer that provides:

- Real-time game board rendering
- Color-coded player territories
- Smooth updates as pieces are placed
- Window-based display with event handling

To enable visualization, ensure SDL2 is installed and build with:

```bash
cargo build --release
```

## Algorithm Performance

This implementation focuses on:

- **Aggressive Positioning**: Consistently moves toward opponent territory
- **Strategic Blocking**: Prevents opponent expansion
- **Efficient Territory Control**: Maximizes controlled area
- **Adaptive Strategy**: Adjusts based on board state and opponent behavior

### Key Optimizations

- Manhattan distance for O(1) distance calculations
- Heat map caching for performance
- Multi-threaded visualization (non-blocking)
- Memory-efficient board representation

## Contributing

This project is part of the Zone01 Kisumu curriculum. The implementation focuses on:

1. **Algorithm Efficiency**: Fast decision making within time constraints
2. **Strategic Depth**: Multiple complementary strategies
3. **Code Quality**: Clean, maintainable Rust code
4. **Performance**: Optimized for competitive play

## Game Engine Flags

When running the game engine, you can use these flags:

- `-f, -file string`: Path to map file
- `-p1, -player1 string`: Path to player 1 AI
- `-p2, -player2 string`: Path to player 2 AI  
- `-q, -quiet`: Quiet mode (minimal output)
- `-r, -refresh`: Throttling mode
- `-s, -seed int`: Use specific random seed
- `-t, -time int`: Set timeout in seconds (default 10)

## Notes

- The AI must respond within the specified timeout (default 10 seconds)
- Invalid moves result in the player being eliminated
- The AI should always return a coordinate, even if no valid moves exist (return `0 0`)
- Log files are automatically generated for debugging and analysis
