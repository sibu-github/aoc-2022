use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum HeadMove {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for HeadMove {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, c) = s.trim().split_once(' ').unwrap();
        let c = c.parse::<usize>().unwrap();
        match d {
            "U" => Ok(Self::Up(c)),
            "D" => Ok(Self::Down(c)),
            "L" => Ok(Self::Left(c)),
            "R" => Ok(Self::Right(c)),
            _ => Err("Invalid string"),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn move_up(&mut self) {
        self.y += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_up_right(&mut self) {
        self.x += 1;
        self.y += 1;
    }
    fn move_up_left(&mut self) {
        self.x -= 1;
        self.y += 1;
    }
    fn move_down_left(&mut self) {
        self.x -= 1;
        self.y -= 1;
    }
    fn move_down_right(&mut self) {
        self.x += 1;
        self.y -= 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KnotPosition {
    knot_no: usize,
    pos: Point,
}

impl KnotPosition {
    fn origin(knot_no: usize) -> Self {
        Self {
            knot_no,
            pos: Point::origin(),
        }
    }
}


struct MultiKnotRope {
    knots_count: usize,
    knot_curr_pos: Vec<KnotPosition>,
    knot_positions: Vec<KnotPosition>,
}

impl MultiKnotRope {
    fn new(knots_count: usize) -> Self {
        let knot_curr_pos = (0..knots_count)
            .map(KnotPosition::origin)
            .collect::<Vec<_>>();
        let knot_positions = knot_curr_pos.clone();
        Self {
            knots_count,
            knot_curr_pos,
            knot_positions,
        }
    }

    fn perform_move(&mut self, knot_no: usize, direction: Direction) {
        for knot in self.knot_curr_pos.iter_mut() {
            if knot.knot_no == knot_no {
                match direction {
                    Direction::Up => knot.pos.move_up(),
                    Direction::Down => knot.pos.move_down(),
                    Direction::Left => knot.pos.move_left(),
                    Direction::Right => knot.pos.move_right(),
                    Direction::UpRight => knot.pos.move_up_right(),
                    Direction::UpLeft => knot.pos.move_up_left(),
                    Direction::DownLeft => knot.pos.move_down_left(),
                    Direction::DownRight => knot.pos.move_down_right(),
                }
                if !self.knot_positions.iter().any(|k| k == knot) {
                    self.knot_positions.push(*knot);
                }
                break;
            }
        }
        let knot_no = knot_no + 1;
        if let Some(direction) = self.require_move(knot_no) {
            self.perform_move(knot_no, direction);
        }
    }

    fn require_move(&self, knot_no: usize) -> Option<Direction> {
        if knot_no == 0 || knot_no >= self.knots_count {
            return None;
        }
        let prev = self
            .knot_curr_pos
            .iter()
            .find(|knot| knot.knot_no == knot_no - 1)?;
        let this = self
            .knot_curr_pos
            .iter()
            .find(|knot| knot.knot_no == knot_no)?;
        if prev.pos.x > this.pos.x + 1 {
            if prev.pos.y > this.pos.y {
                return Some(Direction::UpRight);
            } else if prev.pos.y == this.pos.y {
                return Some(Direction::Right);
            } else {
                return Some(Direction::DownRight);
            }
        } else if prev.pos.x < this.pos.x - 1 {
            if prev.pos.y > this.pos.y {
                return Some(Direction::UpLeft);
            } else if prev.pos.y == this.pos.y {
                return Some(Direction::Left);
            } else {
                return Some(Direction::DownLeft);
            }
        } else if prev.pos.y > this.pos.y + 1 {
            if prev.pos.x > this.pos.x {
                return Some(Direction::UpRight);
            } else if prev.pos.x == this.pos.x {
                return Some(Direction::Up);
            } else {
                return Some(Direction::UpLeft);
            }
        } else if prev.pos.y < this.pos.y - 1 {
            if prev.pos.x > this.pos.x {
                return Some(Direction::DownRight);
            } else if prev.pos.x == this.pos.x {
                return Some(Direction::Down);
            } else {
                return Some(Direction::DownLeft);
            }
        }
        None
    }

    fn get_knot_positions(&self, knot_no: usize) -> Vec<&KnotPosition> {
        self.knot_positions
            .iter()
            .filter(|k| k.knot_no == knot_no)
            .collect()
    }
}


fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let moves = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| HeadMove::from_str(&line).unwrap())
        .collect::<Vec<_>>();
    let mut multi_knot_rope = MultiKnotRope::new(2);
    moves
        .iter()
        .for_each(|m| {
            match m {
                HeadMove::Up(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Up)),
                HeadMove::Down(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Down)),
                HeadMove::Left(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Left)),
                HeadMove::Right(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Right)),
            }
        });
    println!("{}", multi_knot_rope.get_knot_positions(1).len());

    let mut multi_knot_rope = MultiKnotRope::new(10);
    moves
        .iter()
        .for_each(|m| {
            match m {
                HeadMove::Up(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Up)),
                HeadMove::Down(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Down)),
                HeadMove::Left(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Left)),
                HeadMove::Right(c) => (0..*c).for_each(|_| multi_knot_rope.perform_move(0, Direction::Right)),
            }
        });
    println!("{}", multi_knot_rope.get_knot_positions(9).len());
}
