use rand::Rng;
use std::collections::LinkedList;

const INITIAL_SNAKE_TAIL_LENGTH: u32 = 2;

pub struct SnakeGame {
    size: (u32, u32),
    snake: Snake,
    food: Position,
    pub score: u32,
    over: bool,
}

impl SnakeGame {
    pub fn new(width: u32, height: u32) -> Self {
        // size: (width, height);

        //     let mut rng = rand::thread_rng();
        //     let x = rng.gen_range(0, width);
        //     let y = rng.gen_range(0, height);
        //     let head = Position { x, y };
        //     let snake = Snake::new(head);
        //     let food = Self::generate_food(&snake, width, height);
        Self {
            size: (width, height),
            snake: Snake::new(get_random_position(width, height)),
            food: get_random_position(width, height),
            score: 0,
            over: false,
        }
    }
}

#[derive(Clone)]
struct Position {
    pub x: u32,
    pub y: u32,
}

// impl Position {
//     pub fn move_to_dir(&mut self, dir: Direction) {
//         match dir {
//             Direction::Up => self.y -= 1,
//             Direction::Down => self.y += 1,
//             Direction::Left => self.x -= 1,
//             Direction::Right => self.x += 1,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)] //PartialEq,
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
    updated_tail_pos: bool,
}

impl Snake {
    fn new(head: Position) -> Self {
        let (x, y) = (head.x, head.y);
        let mut tail = LinkedList::new();

        for i in 1..INITIAL_SNAKE_TAIL_LENGTH + 1 {
            tail.push_back(Position { x, y: y - i });
        }

        Self { direction: Direction::Down, head: head.clone(), tail, updated_tail_pos: false }
    }
    fn move_to_dir(&mut self, dir: Direction) {
        if dir.opposite() == self.direction {
            return;
        }
        self.direction = dir;
    }
}

fn get_random_position(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();
    Position { x: rng.gen_range(0..width), y: rng.gen_range(0..height) }
}