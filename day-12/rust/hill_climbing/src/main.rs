use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(usize, usize);

#[derive(Debug, Clone)]
struct Node {
    height: u8,
    distance: u32,
}

impl Node {
    fn new(height: u8) -> Self {
        Self {
            height,
            distance: u32::MAX,
        }
    }
}

#[derive(Debug, Clone)]
struct Hill {
    height_map: Vec<Vec<Node>>,
    start: Point,
    end: Point,
    all_paths: Vec<u32>,
}

impl FromStr for Hill {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut height_map = vec![];
        for (row, line) in s.lines().enumerate() {
            let mut map = vec![];
            for (col, ch) in line.char_indices() {
                if ch == 'S' {
                    start = Some(Point(row, col));
                    let height = ('a' as u8) - 96;
                    let node = Node::new(height);
                    map.push(node);
                } else if ch == 'E' {
                    end = Some(Point(row, col));
                    let height = ('z' as u8) - 96;
                    let node = Node::new(height);
                    map.push(node);
                } else {
                    let height = (ch as u8) - 96;
                    let node = Node::new(height);
                    map.push(node);
                }
            }
            height_map.push(map);
        }
        let hill = Self {
            height_map,
            start: start.unwrap(),
            end: end.unwrap(),
            all_paths: vec![],
        };
        Ok(hill)
    }
}

impl Hill {
    fn reset(&mut self) {
        self.all_paths.clear();
        self.height_map.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|node| {
                node.distance = u32::MAX;
            })
        });
    }

    fn find_path(&mut self) {
        let mut unvisited = vec![(self.start, 0)];
        while let Some((point, distance)) = unvisited.pop() {
            let node = &mut self.height_map[point.0][point.1];
            if point == self.end {
                self.all_paths.push(distance);
                continue;
            }
            if node.distance <= distance {
                continue;
            }
            node.distance = distance;
            if let Some(next) = self.next_up(&point) {
                if self.can_go_next(&point, &next) {
                    unvisited.push((next, distance + 1));
                }
            }
            if let Some(next) = self.next_left(&point) {
                if self.can_go_next(&point, &next) {
                    unvisited.push((next, distance + 1));
                }
            }
            if let Some(next) = self.next_down(&point) {
                if self.can_go_next(&point, &next) {
                    unvisited.push((next, distance + 1));
                }
            }
            if let Some(next) = self.next_right(&point) {
                if self.can_go_next(&point, &next) {
                    unvisited.push((next, distance + 1));
                }
            }
        }
    }

    fn next_up(&self, curr_point: &Point) -> Option<Point> {
        if curr_point.0 == 0 {
            None
        } else {
            Some(Point(curr_point.0 - 1, curr_point.1))
        }
    }

    fn next_down(&self, curr_point: &Point) -> Option<Point> {
        if curr_point.0 + 1 == self.height_map.len() {
            None
        } else {
            Some(Point(curr_point.0 + 1, curr_point.1))
        }
    }

    fn next_left(&self, curr_point: &Point) -> Option<Point> {
        if curr_point.1 == 0 {
            None
        } else {
            Some(Point(curr_point.0, curr_point.1 - 1))
        }
    }

    fn next_right(&self, curr_point: &Point) -> Option<Point> {
        if curr_point.1 + 1 == self.height_map[0].len() {
            None
        } else {
            Some(Point(curr_point.0, curr_point.1 + 1))
        }
    }

    fn can_go_next(&self, curr_point: &Point, next_point: &Point) -> bool {
        let l1 = self.get_height(curr_point);
        let l2 = self.get_height(next_point);
        l1 + 1 >= l2
    }

    fn get_height(&self, point: &Point) -> u8 {
        self.height_map[point.0][point.1].height
    }

    fn find_starting_points(&self) -> Vec<Point> {
        let start = self.start;
        let mut starting_points = vec![start];
        let mut unvisited = vec![start];
        while let Some(point) = unvisited.pop() {
            if let Some(next) = self.next_up(&point) {
                if !starting_points.contains(&next) {
                    if self.get_height(&start) == self.get_height(&next) {
                        starting_points.push(next);
                        unvisited.push(next);
                    }
                }
            }
            if let Some(next) = self.next_down(&point) {
                if !starting_points.contains(&next) {
                    if self.get_height(&start) == self.get_height(&next) {
                        starting_points.push(next);
                        unvisited.push(next);
                    }
                }
            }
            if let Some(next) = self.next_left(&point) {
                if !starting_points.contains(&next) {
                    if self.get_height(&start) == self.get_height(&next) {
                        starting_points.push(next);
                        unvisited.push(next);
                    }
                }
            }
            if let Some(next) = self.next_right(&point) {
                if !starting_points.contains(&next) {
                    if self.get_height(&start) == self.get_height(&next) {
                        starting_points.push(next);
                        unvisited.push(next);
                    }
                }
            }
        }
        starting_points
    }
    fn shortest_path(&mut self) -> u32 {
        self.all_paths.sort_unstable();
        self.all_paths.get(0).cloned().unwrap_or(u32::MAX)
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let mut hill = Hill::from_str(&content).unwrap();
    let starting_points = hill.find_starting_points();
    hill.find_path();
    println!("{}", hill.shortest_path());
    let mut all_paths = starting_points.iter().map(|&start| {
        hill.reset();
        hill.start = start;
        hill.find_path();
        hill.shortest_path()
    }).collect::<Vec<_>>();
    all_paths.sort_unstable();
    println!("{}", all_paths[0]);
}
