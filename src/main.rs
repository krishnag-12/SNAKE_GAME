// External crates used in the project
extern crate rand;
extern crate piston_window;

// Internal modules defined in the project
mod draw;
mod snake;
mod game;

// Import necessary types and functions from piston_window crate
use piston_window::*;
use piston_window::types::Color;

use draw::to_coord_u32;
use game::Game;

// Background color of the game window (gray)
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // Dimensions of the game board in grid blocks
    let (width, height) = (30, 30);

    // Create a new game window with specified dimensions and title
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)], // Convert grid dimensions to pixels
    )
    .exit_on_esc(true) // Allow exit on pressing Esc key
    .build()
    .unwrap(); // Panic if window creation fails

    // Initialize the game state
    let mut game = Game::new(width, height);

    // Main game loop: handles input, rendering, and updating
    while let Some(event) = window.next() {
        // Handle keyboard input
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw the current game frame
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g); // Clear window with background color
            game.draw(&c, g);     // Draw game objects (snake, food, etc.)
        });

        // Update game state based on elapsed time
        event.update(|arg| {
            game.update(arg.dt); // Pass delta time to update logic
        });
    }
}