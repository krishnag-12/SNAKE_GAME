// Import required types and functions from the piston_window crate
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

// Define the size of each block in the game grid (in pixels)
const BLOCK_SIZE: f64 = 25.0;

/// Converts game grid coordinates (i32) to GUI coordinates (f64)
/// This is used for rendering objects correctly in the window
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Converts game grid coordinates to GUI coordinates (u32)
/// Useful when dimensions need to be passed in unsigned format
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// Draws a single block (typically used for snake segments or food)
/// 
/// # Arguments
/// * `color` - Color of the block
/// * `x`, `y` - Game coordinates of the block
/// * `con` - Rendering context
/// * `g` - Graphics buffer
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], // Position and size
        con.transform,
        g,
    );
}

/// Draws a rectangle made up of blocks (used for borders, UI elements, etc.)
/// 
/// # Arguments
/// * `color` - Color of the rectangle
/// * `x`, `y` - Starting game coordinates
/// * `width`, `height` - Number of blocks in width and height
/// * `con` - Rendering context
/// * `g` - Graphics buffer
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),  // Width in pixels
            BLOCK_SIZE * (height as f64), // Height in pixels
        ],
        con.transform,
        g,
    );
}