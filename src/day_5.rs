use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print() {
    println!("Day 5, Part 1: {}", day5_part1()); // 963
    println!("Day 5, Part 2: {}", day5_part2()); // 592
}

fn day5_part1() -> i32 {
    let file = File::open("resources/input_day_5").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut max = -1;
    for line in lines {
        let string = line.unwrap();
        let tuple = string.split_at(7);
        let row_binary = tuple.0.replace("F", "0").replace("B", "1");
        let row_number = isize::from_str_radix(&*row_binary, 2).unwrap();

        let col_binary = tuple.1.replace("L", "0").replace("R", "1");
        let col_number = isize::from_str_radix(&*col_binary, 2).unwrap();

        let seat_id = row_number * 8 + col_number;
        println!("{}, {}, {}", row_number, col_number, seat_id);

        if seat_id > max { max = seat_id }
    }
    return max as i32;
}


fn day5_part2() -> i32 {
    let file = File::open("resources/input_day_5").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut seat_ids = lines
        .map(|line| line.unwrap().clone())
        .map(|string| split_string(&string))
        .map(|tuple| {
            let row_binary = tuple.0.replace("F", "0").replace("B", "1");
            let row_number = isize::from_str_radix(&*row_binary, 2).unwrap();

            let col_binary = tuple.1.replace("L", "0").replace("R", "1");
            let col_number = isize::from_str_radix(&*col_binary, 2).unwrap();

            (row_number, col_number)
        })
        .map(|tuple| (tuple.0 * 8 + tuple.1) as usize)
        .collect::<Vec<_>>();

    seat_ids.sort_unstable();

    for i in *seat_ids.first().unwrap()..*seat_ids.last().unwrap() {
        if seat_ids[i + 1] - seat_ids[i] == 2 {
            return (seat_ids[i] + 1) as i32;
        }
    }

    return -1;
}

fn split_string(s: &String) -> (String, String) {
    let tuple = s.split_at(7);
    (String::from(tuple.0), String::from(tuple.1))
}

#[test]
fn test_day4() {
    print();
}
