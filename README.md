# Tic-Tac-Toe AI

A Tic-Tac-Toe game in Rust with an optimal AI opponent, built using Macroquad for graphics.

## Features
- Menu to select game mode: Human vs Human or Human vs AI
- For AI mode, choose to play as X or O
- AI uses minimax algorithm for perfect play
- Win/draw detection and game state management
- Restart game or return to menu after completion
- Simple, responsive graphical interface

## Requirements
- Rust (latest stable version)
- Macroquad library (installed via Cargo)

## Setup

```bash
git clone https://github.com/trfhgx/tic-tac-toe-ai
cd tic-tac-toe-ai
cargo run
```

## Project Structure
- `src/main.rs`: Main game loop, GUI, and state management
- `src/board.rs`: Board data structure and utilities
- `src/game.rs`: Game logic and rules
- `src/ai.rs`: Minimax AI implementation

## License
MIT License
