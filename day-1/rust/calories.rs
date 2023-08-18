use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = File::open("inputs.txt").unwrap();
    let reader = BufReader::new(file);
    let mut highest_1 = 0;
    let mut highest_2 = 0;
    let mut highest_3 = 0;
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if sum > highest_1 {
                highest_3 = highest_2;
                highest_2 = highest_1;
                highest_1 = sum;
            } else if sum > highest_2 {
                highest_3 = highest_2;
                highest_2 = sum;
            } else if sum > highest_3 {
                highest_3 = sum;
            }
            sum = 0;
            continue;
        }
        let num = u32::from_str(&line).unwrap();
        sum += num;
    }
    if sum > highest_1 {
        highest_3 = highest_2;
        highest_2 = highest_1;
        highest_1 = sum;
    } else if sum > highest_2 {
        highest_3 = highest_2;
        highest_2 = sum;
    } else if sum > highest_3 {
        highest_3 = sum;
    }

    println!("Highest is: {highest_1}");
    println!("Total of highest 3: {}", highest_1 + highest_2 + highest_3);
}
