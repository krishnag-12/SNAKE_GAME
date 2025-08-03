use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

// Snake's color on the screen
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// Enum to define possible directions the snake can move
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    // Returns the opposite direction
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Struct representing each segment of the snake's body
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

// Main struct representing the snake
pub struct Snake {
    direction: Direction,         // Current direction of the snake's movement
    body: LinkedList<Block>,      // List of blocks making up the snake's body
    tail: Option<Block>,          // Last removed tail block (used when growing)
}

impl Snake {
    // Initializes a new snake with 3 blocks starting at (x, y)
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Draws the snake on the game window
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // Returns the current position of the snake's head
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // Moves the snake forward in the current or new direction
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Update direction if provided
        if let Some(d) = dir {
            self.direction = d;
        }

        // Compute the new head position based on direction
        let (last_x, last_y) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y },
        };

        // Add new head and remove tail
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // Returns the current direction of the snake's head
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Calculates the next head position without actually moving
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let moving_dir = dir.unwrap_or(self.direction);

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // Restores the tail block that was previously removed (used when eating food)
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // Checks if the given (x, y) position overlaps with the snake's body (excluding the head)
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            // Skip the head (last block is the head in the LinkedList)
            if ch == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}