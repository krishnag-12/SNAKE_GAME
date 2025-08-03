// Importing required modules from piston_window
use piston_window::*;
use piston_window::types::Color;

// For generating random numbers (used for placing food)
use rand::{thread_rng, Rng};

// Bringing in custom modules from the crate
use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

// Define constant colors for various game elements
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 1.0];

// Game speed and restart delay constants (in seconds)
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

// Main Game structure to hold all game-related data
pub struct Game {
    snake: Snake,       // The snake object

    food_exists: bool,  // Flag to check if food is currently present
    food_x: i32,        // X-coordinate of food
    food_y: i32,        // Y-coordinate of food
    width: i32,         // Width of the game area
    height: i32,        // Height of the game area

    game_over: bool,    // Flag for game over state
    waiting_time: f64,  // Time accumulator for snake movement and restart delay
}

impl Game {
    /// Initializes a new Game instance
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    /// Handles key press events for snake movement
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // Prevent snake from reversing direction
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    /// Draws all elements: snake, food, border, game over screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);                     // Top
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);      // Bottom
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);                   // Left
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);      // Right

        // Show red screen if game is over
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// Updates game logic: movement, food, and restart handling
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            // Restart game after delay
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // Generate new food if none exists
        if !self.food_exists {
            self.add_food();
        }

        // Move snake if movement period has passed
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// Checks if the snake has eaten the food
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail(); // Grow the snake
        }
    }

    /// Checks whether the snake will be alive after the next move
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false; // Collision with its own body
        }

        // Ensure the next position is within the game boundaries
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// Randomly places food on the board, ensuring it doesn't overlap the snake
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    /// Updates the snake's position, checks for collisions and food
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0; // Reset movement timer
    }

    /// Resets the game to its initial state
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}