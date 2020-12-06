use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use bit_vec::BitVec;

pub fn print() {
    println!("Day 6, Part 1: {}", day6_part1()); // 6259
    println!("Day 6, Part 2: {}", day6_part2()); // 3178
}

fn day6_part1() -> i32 {
    let file = File::open("resources/input_day_6").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut sum = 0;
    let mut group_answers = HashSet::new();
    for line in lines {
        let string = line.unwrap();

        if (&string).is_empty() {
            sum += group_answers.len();
            group_answers.clear();
        } else {
            for char in string.chars() {
                group_answers.insert(char);
            }
        }
    }
    if !group_answers.is_empty() {
        sum += group_answers.len();
    }
    return sum as i32;
}


fn day6_part2() -> i32 {
    let file = File::open("resources/input_day_6").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut sum = 0;
    let mut group_answers: Vec<BitVec> = Vec::new();
    for line in lines {
        let string = line.unwrap();
        let mut bits = BitVec::from_elem(26, false);


        if (&string).is_empty() {
            sum += count_answers(&mut group_answers);

            group_answers.clear();
        } else {
            for char in string.chars() {
                bits.set(*&char.to_digit(36).map(|d| d - 10).unwrap() as usize, true);
            }
            group_answers.push(bits);
        }
    }
    if !group_answers.is_empty() {
        sum += count_answers(&mut group_answers);
    }
    return sum as i32;
}

fn count_answers(group_answers: &mut Vec<BitVec>) -> usize {
    group_answers.iter()
        .fold(BitVec::from_elem(26, true), |mut a: BitVec, b: &BitVec| {
            a.and(&b);
            a
        })
        .iter()
        .filter(|b| *b)
        .count()
}

#[test]
fn test_day6() {
    print();
}
