use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Tree {
    height: u32,
    position: (usize, usize),
}

impl Tree {
    fn new(height: u32, position: (usize, usize)) -> Self {
        Self { height, position }
    }
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .enumerate()
        .map(|(row, line)| (row, line.unwrap()))
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let height = c.to_digit(10).unwrap();
                    Tree::new(height, (row, col))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let row = grid.len() - 1;
    let col = grid[0].len() - 1;
    let mut invisible_trees = vec![];
    for i in 1..=row {
        for j in 1..=col {
            let tree = &grid[i][j];
            // from left
            let slice = &grid[i][0..j];
            if slice.iter().all(|t| t.height < tree.height) {
                continue;
            }
            // from right
            let slice = &grid[i][j + 1..];
            if slice.iter().all(|t| t.height < tree.height) {
                continue;
            }
            // from top
            let mut is_invisible_top = false;
            for r in 0..i {
                if grid[r][j].height >= tree.height {
                    is_invisible_top = true;
                    break;
                }
            }
            if !is_invisible_top {
                continue;
            }
            // from bottom
            let mut is_invisible_bottom = false;
            for r in i + 1..=row {
                if grid[r][j].height >= tree.height {
                    is_invisible_bottom = true;
                    break;
                }
            }
            if is_invisible_bottom {
                invisible_trees.push(tree);
            }
        }
    }

    println!(
        "visible count: {}",
        grid.len() * grid[0].len() - invisible_trees.len()
    );
    let mut tree_score = vec![];
    for i in 1..row {
        for j in 1..col {
            let mut left_count = 0;
            let mut right_count = 0;
            let mut top_count = 0;
            let mut bottom_count = 0;
            for k in (0..j).rev() {
                left_count += 1;
                if grid[i][k].height >= grid[i][j].height {
                    break;
                }
            }
            for k in j + 1..=col {
                right_count += 1;
                if grid[i][k].height >= grid[i][j].height {
                    break;
                }
            }
            for r in (0..i).rev() {
                top_count += 1;
                if grid[r][j].height >= grid[i][j].height {
                    break;
                }
            }
            for r in i + 1..=row {
                bottom_count += 1;
                if grid[r][j].height >= grid[i][j].height {
                    break;
                }
            }
            let score = left_count * right_count * top_count * bottom_count;
            assert!(score != 0);
            tree_score.push(score);
        }
    }
    tree_score.sort_unstable();
    println!("{}", tree_score.pop().unwrap());
}
