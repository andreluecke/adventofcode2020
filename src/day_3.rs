use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print() {
    println!("Day 3, Part 1: {}", day3_part1()); // 289
    println!("Day 3, Part 2: {}", day3_part2()); // 12885603696
}

fn day3_part1() -> i32 {
    let file = File::open("resources/input_day_3").expect("no such file");
    let buf = BufReader::new(file);

    let lines = buf.lines();

    let mut i = 0;
    let mut count = 0;
    for line in lines {
        let string = line.unwrap();
        let current_char = string.chars().nth(i).unwrap();
        if current_char == '#' { count += 1 };

        i = (i + 3) % string.len();
    }
    return count;
}

fn day3_part2() -> i64 {
    let file = File::open("resources/input_day_3").expect("no such file");
    let lines = BufReader::new(&file).lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut product: i64 = 1;
    for slope in slopes.iter() {
        let mut count = 0;
        let mut j = 0;

        for i in 0..lines.len() {
            if i % slope.1 == 0 {
                let line = lines.get(i).unwrap();
                let c = line.get(j).unwrap();
                if *c == '#' {
                    count += 1;
                }
                j = (j + slope.0) % line.len();
            }
        }
        println!("{}", &count);
        product *= count;
    }

    return product;
}

#[test]
fn test_day3() {
    print();
}

