use std::fs::File;
use std::io::{BufRead, BufReader};

trait Overlap {
    fn is_fully_contain(&self) -> bool;
    fn is_overlap(&self) -> bool;
}

impl Overlap for &str {
    fn is_fully_contain(&self) -> bool {
        let (left, right) = self.split_once(',').unwrap();
        let (left_lb, left_rb) = left.split_once('-').unwrap();
        let (right_lb, right_rb) = right.split_once('-').unwrap();
        let left_lb = left_lb.parse::<u32>().unwrap();
        let left_rb = left_rb.parse::<u32>().unwrap();
        let right_lb = right_lb.parse::<u32>().unwrap();
        let right_rb = right_rb.parse::<u32>().unwrap();
        (left_lb >= right_lb && left_rb <= right_rb) || (left_lb <= right_lb && left_rb >= right_rb)
    }

    fn is_overlap(&self) -> bool {
        let (left, right) = self.split_once(',').unwrap();
        let (left_lb, left_rb) = left.split_once('-').unwrap();
        let (right_lb, right_rb) = right.split_once('-').unwrap();
        let left_lb = left_lb.parse::<u32>().unwrap();
        let left_rb = left_rb.parse::<u32>().unwrap();
        let right_lb = right_lb.parse::<u32>().unwrap();
        let right_rb = right_rb.parse::<u32>().unwrap();
        (left_lb >= right_lb && left_lb <= right_rb)
            || (left_rb >= right_lb && left_rb <= right_rb)
            || (right_lb >= left_lb && right_lb <= left_rb)
            || (right_rb >= left_lb && right_rb <= left_rb)
    }
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut fully_contain_count = 0;
    let mut overlap_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.as_str().is_fully_contain() {
            fully_contain_count += 1;
        }
        if line.as_str().is_overlap() {
            overlap_count += 1;
        }
    }
    println!("Fully cotain count {fully_contain_count}");
    println!("Overlap count {overlap_count}");
}
