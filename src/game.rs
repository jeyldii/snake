use std::cmp::PartialEq;
use std::io::{Read, Write};
use std::{os, time};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, MutexGuard};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use rand::{Rng, thread_rng};
use termion::async_stdin;
use crate::board::Board;
use crate::snake::{Direction, Snake};

pub enum Core {
    SnakeHead = 1,
    SnakeBody = 4,
    SnakeTail = 3,
    Rabbit = 2,
}

pub struct Game<'a> {
    snake: &'a mut Snake,
    board: &'a mut Board,
}

impl<'a> Game<'a> {
    pub fn new(board: &'a mut Board, snake: &'a mut Snake) -> Self {
        Self {
            snake,
            board,
        }
    }

    fn gen_rabbit(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..Board::size());
        let y = rng.gen_range(0..Board::size());
        self.board.set_rabbit(x, y);
    }

    fn is_eligible_to_eat(&self) -> bool {
        let rabbit = self.board.get_rabbit_position();
        let head = self.snake.get_head();
        rabbit == head
    }

    fn move_snake(&mut self, direction: Direction) {
        let mut head = self.snake.get_head();
        match direction {
            Direction::Up => {
                let y = head.y as i64;
                if y - 1 < 0 {
                    head.y = Board::size() - 1;
                } else {
                    head.y = head.y - 1;
                }
            }
            Direction::Right => {
                let x = head.x as i64;
                if x + 1 > Board::size() as i64 - 1 {
                    head.x = 0
                } else {
                    head.x = head.x + 1;
                }
            }
            Direction::Left => {
                let x = head.x as i64;
                if x - 1 < 0 {
                    head.x = Board::size() - 1;
                } else {
                    head.x = head.x - 1;
                }
            }
            Direction::Down => {
                let y = head.y as i64;
                if y + 1 > Board::size() as i64 - 1 {
                    head.y = 0;
                } else {
                    head.y = head.y + 1;
                }
            }
        }
        self.snake.set_head(head);
        self.board.set_snake(self.snake);
        if self.is_eligible_to_eat() {
            self.snake.eat(self.board.get_rabbit_position());
            self.gen_rabbit()
        }
    }

    pub fn start_game(&mut self) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Hide);
        println!("{}", self.board.echo());
        stdout.flush().unwrap();

        let mut keys = async_stdin().keys();
        loop {
            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
            let event = keys.next();
            let mut direction = match event {
                None => self.snake.get_direction(),
                Some(event) => match event.unwrap() {
                    Key::Char('q') => panic!("\r\nQuit the game"),
                    Key::Up => Direction::Up,
                    Key::Down => Direction::Down,
                    Key::Right => Direction::Right,
                    Key::Left => Direction::Left,
                    _ => { self.snake.get_direction() }
                }
            };

            if direction.is_opposite(self.snake.get_direction()) {
                direction = self.snake.get_direction();
            }

            self.move_snake(direction);
            self.snake.set_direction(direction);
            write!(stdout, "{}", self.board.echo());
            stdout.flush().unwrap();
            std::thread::sleep(time::Duration::from_millis(150));
        }
    }
}