use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err("Invalid string"),
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        }
    }
}

impl RPS {
    fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score(&self, other: &Self) -> u32 {
        let score = if self == other {
            3
        } else if self < other {
            0
        } else {
            6
        };
        score + self.shape_score()
    }

    fn choice(&self, strategy: &str) -> Self {
        match strategy {
            "X" => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            "Y" => self.clone(),
            "Z" => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_score_1 = 0;
    let mut total_score_2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let (opponent, strategy) = line.split_once(' ').unwrap();
        let opponent = opponent.parse::<RPS>().unwrap();
        let choice_1 = strategy.parse::<RPS>().unwrap();
        total_score_1 += choice_1.score(&opponent);
        let choice_2 = opponent.choice(strategy);
        total_score_2 += choice_2.score(&opponent);
    }
    println!("Total score is: {} {}", total_score_1, total_score_2);
}
