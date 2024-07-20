mod board;
mod snake;
mod game;

use std::io::{BufRead, Read, Write};
use std::sync::Arc;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use snake::{Snake, Position};
use crate::board::Board;
use crate::game::Core;

fn main() {
    let mut board = Box::new(Board::new());

    let mut snake = Snake::new(Position { x: 4, y: 12 }, Position { x: 4 - 1, y: 12 });
    board.set_rabbit(10, 5);

    let head = snake.get_head();
    let tail = snake.get_tail();
    board.set_snake((head.x, head.y), (tail.x, tail.y));


    let mut game = game::Game::new(&mut board, &mut snake);

    game.start_game();

    // let stdin = std::io::stdin();
    // let mut s = String::new();
    // for i in 0..10 {
    //     stdin.read_line(&mut s);
    //     board.echo();
    //     print!("\x1B[2J");
    // }
    // return;
}
