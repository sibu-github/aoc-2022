use std::fs::File;
use std::io::{BufRead, BufReader};

trait ItemPriority {
    fn item_priority(&self) -> u32;
}

impl ItemPriority for char {
    fn item_priority(&self) -> u32 {
        if self.is_uppercase() {
            ((*self as u32) - 65) + 27
        } else {
            ((*self as u32) - 97) + 1
        }
    }
}

trait FindItem {
    fn find_item(&self) -> char;
}

impl FindItem for &str {
    fn find_item(&self) -> char {
        let mid = self.len() / 2;
        let (first, last) = self.split_at(mid);
        for c in first.chars() {
            if last.chars().any(|ch| ch == c) {
                return c;
            }
        }
        panic!("no common character found");
    }
}

trait FindBadge {
    fn find_badge(&self) -> char;
}

impl FindBadge for &[String] {
    fn find_badge(&self) -> char {
        if self.len() != 3 {
            panic!("group length must be 3");
        }
        for c in self[0].chars() {
            if self[1].chars().any(|ch| ch == c) && self[2].chars().any(|ch| ch == c) {
                return c;
            }
        }
        panic!("no common badge found");
    }
}

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let s = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|s| s.as_str().find_item().item_priority())
            .sum::<u32>();
        println!("Total: {s}");
    }
    {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let v = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
        let s = v
            .chunks(3)
            .map(|v| v.find_badge().item_priority())
            .sum::<u32>();
        println!("Total badge: {}", s);
    }
}
