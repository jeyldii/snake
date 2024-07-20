pub struct Snake {
    head: Position,
    tail: Position,
    body: Vec<Position>,
    direction: Direction,
    len: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Snake {
    pub fn new(head: Position, tail: Position) -> Self {
        Self {
            head,
            tail,
            body: Vec::new(),
            direction: Direction::Right,
            len: 0,
        }
    }

    pub fn get_head(&self) -> Position {
        self.head
    }

    pub fn get_tail(&self) -> Position {
        self.tail
    }

    pub fn set_head(&mut self, pos: Position) {
        self.head = pos
    }

    pub fn set_tail(&mut self, pos: Position) {
        self.tail = pos
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn eat(&mut self) {
        todo!();
        let x = self.tail.x - 1;
        let y = self.tail.y;
        self.body.push(Position { x, y });
        self.len += 1;
    }
}