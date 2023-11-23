use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "noop" {
            return Ok(Self::Noop);
        }
        let (ins, val) = s.split_once(' ').ok_or("unable to get value")?;
        if ins != "addx" {
            return Err("unknow instruction");
        }
        let val = val.parse::<i32>().map_err(|_| "not able to parse value")?;
        Ok(Self::AddX(val))
    }
}

#[derive(Debug)]
struct RegisterCycle {
    curr_val: i32,
    values: Vec<i32>,
}

impl RegisterCycle {
    fn new() -> Self {
        Self {
            curr_val: 1,
            values: vec![],
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Noop => {
                self.values.push(self.curr_val);
            }
            Instruction::AddX(v) => {
                self.values.push(self.curr_val);
                self.curr_val = self.curr_val + v;
                self.values.push(self.curr_val);
            }
        }
    }

    fn cycle_strength(&self, cycle: usize) -> i32 {
        assert!(cycle > 2);
        self.values[cycle - 2] * (cycle as i32)
    }
}

#[derive(Debug)]
struct Screen<'a> {
    x_reg_val: i32,
    curr_row: usize,
    curr_col: usize,
    rows: [[&'a str; 40]; 6],
}

impl<'a> Screen<'a> {
    fn new() -> Self {
        Self {
            x_reg_val: 1,
            curr_row: 0,
            curr_col: 0,
            rows: [["."; 40]; 6],
        }
    }

    fn draw(&mut self) {
        let col = self.curr_col as i32;
        if col == self.x_reg_val || col == self.x_reg_val - 1 || col == self.x_reg_val + 1 {
            self.rows[self.curr_row][self.curr_col] = "#";
        }
        self.cycle();
    }

    fn cycle(&mut self) {
        let col = self.curr_col + 1;
        if col == 40 {
            self.curr_row += 1;
            self.curr_col = 0;
        } else {
            self.curr_col = col;
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Noop => {
                self.draw();
            }
            Instruction::AddX(v) => {
                self.draw();
                self.draw();
                self.x_reg_val += v;
            }
        }
    }
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let instructions = reader
        .lines()
        .map(|line| Instruction::from_str(line.unwrap().as_str()).unwrap())
        .collect::<Vec<_>>();
    let mut register_cycle = RegisterCycle::new();
    instructions.iter().for_each(|i| register_cycle.execute(i));
    let s1 = register_cycle.cycle_strength(20);
    let s2 = register_cycle.cycle_strength(60);
    let s3 = register_cycle.cycle_strength(100);
    let s4 = register_cycle.cycle_strength(140);
    let s5 = register_cycle.cycle_strength(180);
    let s6 = register_cycle.cycle_strength(220);
    let sum = s1 + s2 + s3 + s4 + s5 + s6;
    println!("{sum}");
    let mut screen = Screen::new();
    instructions.iter().for_each(|i| screen.execute(i));
    for row in screen.rows {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
