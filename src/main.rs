use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let numbers = lines_from_file("resources/input_day_1");

    println!("Day 1, Part 1: {}", day1_part1(numbers));
}

fn day1_part1(numbers: Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        let n = numbers[i];
        for j in i..numbers.len() {
            let m = numbers[j];

            if n + m == 2020 {
                return n * m;
            }
        }
    }
    return -1;
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| s.parse().unwrap())
        .collect()
}

