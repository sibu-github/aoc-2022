use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
enum ParseError {
    InvalidString,
    InvalidNumber,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber => write!(f, "Invalid Number"),
            Self::InvalidString => write!(f, "Invalid String"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').ok_or(ParseError::InvalidString)?;
        let x = x
            .trim()
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let y = y
            .trim()
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidNumber)?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct RockPath {
    src: Point,
    dst: Point,
}

impl RockPath {
    fn new(src: Point, dst: Point) -> Self {
        Self { src, dst }
    }
}

fn get_rockbed_size(paths: &Vec<RockPath>) -> (usize, usize) {
    let all_points = paths
        .iter()
        .flat_map(|v| vec![v.src, v.dst])
        .collect::<Vec<_>>();
    let max_x = all_points.iter().map(|p| p.x).max().unwrap();
    let max_y = all_points.iter().map(|p| p.y).max().unwrap();
    (max_x, max_y)
}

fn populate_rock_path(paths: &Vec<RockPath>, rockbed: &mut Vec<Vec<char>>) {
    for p in paths {
        if p.src.x == p.dst.x {
            let x = p.src.x;
            let (y1, y2) = if p.src.y < p.dst.y {
                (p.src.y, p.dst.y)
            } else {
                (p.dst.y, p.src.y)
            };
            let y2 = y2 + 1;
            for y in y1..y2 {
                rockbed[y][x] = '#';
            }
        } else if p.src.y == p.dst.y {
            let y = p.src.y;
            let (x1, x2) = if p.src.x < p.dst.x {
                (p.src.x, p.dst.x)
            } else {
                (p.dst.x, p.src.x)
            };
            let x2 = x2 + 1;
            for x in x1..x2 {
                rockbed[y][x] = '#';
            }
        }
    }
}

fn execute_sandfall(sandfall_point: Point, rockbed: &mut Vec<Vec<char>>) -> Option<()> {
    if rockbed[sandfall_point.y][sandfall_point.x] == 'O' {
        return None;
    }
    let mut curr_point = sandfall_point;
    loop {
        if curr_point.y + 1 == rockbed.len() {
            return None;
        }
        if rockbed[curr_point.y + 1][curr_point.x] == '.' {
            curr_point.y += 1;
            continue;
        }
        if rockbed[curr_point.y + 1][curr_point.x - 1] == '.' {
            curr_point.y += 1;
            curr_point.x -= 1;
            continue;
        }
        if rockbed[curr_point.y + 1][curr_point.x + 1] == '.' {
            curr_point.y += 1;
            curr_point.x += 1;
            continue;
        }
        rockbed[curr_point.y][curr_point.x] = 'O';
        return Some(());
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let all_rock_paths = content
        .lines()
        .flat_map(|line| {
            let splitted = line.split("->").collect::<Vec<_>>();
            splitted
                .windows(2)
                .map(|v| {
                    let src = Point::from_str(&v[0]).unwrap();
                    let dst = Point::from_str(&v[1]).unwrap();
                    RockPath::new(src, dst)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sandfall_point = Point { x: 500, y: 0 };
    let (max_x, max_y) = get_rockbed_size(&all_rock_paths);
    let max_x = max_x + (max_y * 2);
    let rows = ".".repeat(max_x).chars().collect::<Vec<_>>();
    let rockbed = (0..max_y + 1).map(|_| rows.clone()).collect::<Vec<_>>();
    {
        let mut rockbed = rockbed.clone();
        populate_rock_path(&all_rock_paths, &mut rockbed);
        let mut count = 0;
        while let Some(_) = execute_sandfall(sandfall_point, &mut rockbed) {
            count += 1;
        }
        println!("{}", count);
    }
    {
        let mut rockbed = rockbed.clone();
        populate_rock_path(&all_rock_paths, &mut rockbed);
        rockbed.push(rows.clone());
        let rows = "#".repeat(max_x).chars().collect::<Vec<_>>();
        rockbed.push(rows);
        let mut count = 0;
        while let Some(_) = execute_sandfall(sandfall_point, &mut rockbed) {
            count += 1;
        }
        println!("{}", count);
    }
}
