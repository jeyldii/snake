#![feature(slice_range)]

use std::cmp::{max, min};
use std::io::BufRead;
use std::slice;
use crate::Snake;
use crate::game::Core;
use crate::snake::Position;

const SIZE: usize = 40;


#[derive(Clone, Copy)]
pub struct Cell {
    data: char,
}

impl Cell {
    pub fn new(data: char) -> Self {
        Self {
            data
        }
    }

    pub fn set(&mut self, data: char) {
        self.data = data
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            data: '-',
        }
    }
}

pub struct Board {
    pub grid: [[u8; SIZE]; SIZE],
    rabbit: Position,
}

impl Board {
    pub fn new() -> Self {
        let grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
        Self {
            grid,
            rabbit: Position::default(),
        }
    }

    pub fn size() -> usize {
        SIZE
    }

    fn flush(&mut self) {
        for (i, y) in self.grid.iter_mut().enumerate() {
            for (j, x) in y.iter_mut().enumerate() {
                if *x == Core::Rabbit as u8 {
                    continue;
                }
                *x = 0;
            }
        }
    }

    pub fn get_rabbit_position(&self) -> Position {
        self.rabbit
    }

    pub fn set_rabbit(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Core::Rabbit as u8;
        self.rabbit = Position {
            x,
            y,
        }
    }

    pub fn set_snake(&mut self, snake: &Snake) {
        self.flush();
        let head = snake.get_head();
        let body = snake.get_body();
        self.grid[head.y][head.x] = Core::SnakeHead as u8;
        for i in body {
            self.grid[i.y][i.x] = Core::SnakeBody as u8;
        }
        // self.grid[tail.1][tail.0] = Core::SnakeTail as u8;
    }

    pub fn echo(&self) -> String {
        let mut s = String::new();
        s.push_str("#");
        for i in self.grid {
            s.push_str("#");
        }

        for i in self.grid {
            s.push_str("#");
            for j in i {
                if j == 0 {
                    s.push_str(" ");
                }
                if j == Core::Rabbit as u8 {
                    s.push_str("*")
                }
                if j == Core::SnakeHead as u8 {
                    s.push_str("@")
                }
                if j == Core::SnakeBody as u8 {
                    s.push_str("o")
                }
                // if j == Core::SnakeTail as u8 {
                //     s.push_str(" 0")
                // }
            }
            s.push_str("#");
            s.push_str("\r\n");
        }
        s.push_str("#");
        s.push_str("#");
        for i in self.grid {
            s.push_str("#")
        }
        s
    }
}