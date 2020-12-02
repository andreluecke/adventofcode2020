use std::collections::HashSet;

pub fn print() {
    let numbers = crate::util::parse_numbers_from_file("resources/input_day_1");

    println!("Day 1, Part 1: {}", day1_part1(&numbers)); // 864864
    println!("Day 1, Part 2: {}", day1_part2(&numbers)); // 281473080
}


pub fn day1_part1(numbers: &HashSet<i32>) -> i32 {
    for n in numbers.iter() {
        match find_other_summand(2020, *n, numbers) {
            None => {}
            Some(m) => { return n * m; }
        }
    }
    return -1;
}


pub fn day1_part2(numbers: &HashSet<i32>) -> i32 {
    let sum = 2020;
    for x in numbers.iter() {
        let remainder = sum - x;
        for y in numbers.iter() {
            match find_other_summand(remainder, *y, numbers) {
                None => {}
                Some(z) => {
                    return x * y * z;
                }
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

#[test]
fn test_day1_part1() {
    let numbers = crate::util::parse_numbers_from_file("resources/input_day_1");

    assert_eq!(day1_part1(&numbers), 864864);
}


#[test]
fn test_day1_part2() {
    let numbers = crate::util::parse_numbers_from_file("resources/input_day_1");

    assert_eq!(day1_part2(&numbers), 281473080);
}
