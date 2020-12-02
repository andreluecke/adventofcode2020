use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let numbers = lines_from_file("resources/input_day_1");

    println!("Day 1, Part 1: {}", day1_part1(&numbers)); // 864864
    println!("Day 1, Part 2: {}", day1_part2(&numbers)); // 281473080
}

fn day1_part1(numbers: &HashSet<i32>) -> i32 {
    for n in numbers.iter() {
        match find_other_summand(2020, *n, numbers) {
            None => {}
            Some(m) => { return n * m; }
        }
    }
    return -1;
}


fn day1_part2(numbers: &HashSet<i32>) -> i32 {
    let sum = 2020;
    for x in numbers.iter() {
        let remainder = sum - x;
        for y in numbers.iter() {
            match find_other_summand(remainder, *y, numbers) {
                None => {}
                Some(z) => {
                    return x * y * z; }
            }
        }
    }
    return -1;
}


fn find_other_summand(sum: i32, first_summand: i32, numbers: &HashSet<i32>) -> Option<i32> {
    let remainder = sum - &first_summand;
    if numbers.contains(&remainder) {
        return Some(remainder);
    }
    return None;
}


fn lines_from_file(filename: impl AsRef<Path>) -> HashSet<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| s.parse().unwrap())
        .collect()
}

