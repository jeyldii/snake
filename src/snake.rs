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

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match *self {
            Direction::Up => {
                other == Direction::Down
            }
            Direction::Left => {
                other == Direction::Right
            }
            Direction::Right => {
                other == Direction::Left
            }
            Direction::Down => {
                other == Direction::Up
            }
        }
    }
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
        let prev_head = self.head;
        if self.body.len() != 0 {
            let start = self.body[0];
            self.body[0].x = prev_head.x;
            self.body[0].y = prev_head.y;


            let mut i = self.body.len() - 1;
            while i > 1 {
                let next = self.body[i - 1];
                self.body[i] = next;
                i -= 1;
            }

            if self.body.len() >= 2 {
                self.body[1] = start
            }
            // for i in 2..self.body.len() {
            //     let next = self.body[i - 1];
            //     println!("prev {:?}", self.body[i]);
            //     println!("next {:?}", next);
            //     self.body[i] = next;
            // }
        }
        // println!("body len: {}", self.body.len());
        self.head = pos
    }

    pub fn set_tail(&mut self, pos: Position) {
        self.tail = pos
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    pub fn get_body(&self) -> &[Position] {
        self.body.as_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn eat(&mut self, food: Position) {
        // match self.get_direction() {
        //     Direction::Up => {
        //         self.body.push(Position { x: food.x, y: food.y  })
        //     }
        //     Direction::Left => {
        //         self.body.push(Position { x: food.x + 1, y: food.y })
        //     }
        //     Direction::Right => {
        //         self.body.push(Position { x: food.x - 1, y: food.y })
        //     }
        //     Direction::Down => {
        //         self.body.push(Position { x: food.x, y: food.y - 1 })
        //     }
        // }
        self.body.push(food);
        self.len += 1;
    }
}