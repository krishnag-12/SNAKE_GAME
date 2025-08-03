# 🐍 Snake Game in Rust

A classic Snake game built in Rust using the [`piston_window`](https://crates.io/crates/piston_window) graphics library. The game showcases basic game loop structure, keyboard input handling, and 2D rendering in Rust.

## 🎮 Gameplay

- Use **Arrow keys** to control the snake's direction.
- Eat food to grow your snake.
- Avoid running into yourself or the screen edges.
- The game ends when the snake collides with itself or the wall.

## 📷 Screenshot
 
![Snake Game Screenshot](https://github.com/krishnag-12/SNAKE_GAME/blob/82815103f1ca6f6e4ecf0893289d52e8898e4357/Screenshot%202025-08-03%20173444.png)

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/)
- [piston_window](https://crates.io/crates/piston_window) – For graphics and event handling
- [rand](https://crates.io/crates/rand) – For random food placement

## 📦 Installation

1. **Install Rust** (if not already):  
   Visit [https://rustup.rs](https://rustup.rs) and follow the instructions to install Rust.

2. **Clone the repository and navigate into it:**
   ```bash
   git clone https://github.com/your-username/snake-game.git
   cd snake-game
3. **Run the game:
   ```bash
   cargo run

##📁 Project Structure
.
├── src
│   ├── main.rs       # Game setup and main loop
│   ├── draw.rs       # Drawing helper functions
│   ├── game.rs       # Game logic (collision, food, game over)
│   └── snake.rs      # Snake movement and behavior
├── Cargo.toml        # Project metadata and dependencies
├── Cargo.lock        # Automatically generated dependency versions lock file
└── README.md         # You're here!

##✅ Features
- Modular codebase (separate files for logic and rendering)
- Responsive snake movement
- Self-collision and wall detection
- Growing snake after eating food

##🚧 Future Improvements
- Add score tracking and display
- Implement difficulty levels
- Add sound effects or animations
