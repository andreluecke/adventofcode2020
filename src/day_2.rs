use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

use regex::Regex;
use std::error::Error;

struct Password {
    min: i32,
    max: i32,
    pattern: String,
    password: String,
}


pub fn print() {
    println!("Day 2, Part 1: {}", day2_part1()); // 586
    println!("Day 2, Part 2: {}", day2_part2()); // 352
}

fn day2_part1() -> usize {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) ([A-Za-z]+): (\w+)$").unwrap();

    let file = File::open("resources/input_day_2").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|s| parse_password(&s, &re))
        .map(|pw_result| pw_result.unwrap())
        .filter(|pw| valid_by_rule_part_1(pw))
        .count()
}


fn day2_part2() -> usize {
    let re: Regex = Regex::new(r"^(\d+)-(\d+) ([A-Za-z]+): (\w+)$").unwrap();

    let file = File::open("resources/input_day_2").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|s| parse_password(&s, &re))
        .map(|pw_result| pw_result.unwrap())
        .filter(|pw| valid_by_rule_part_2(pw))
        .count()
}


fn parse_password(s: &String, re: &Regex) -> Result<Password, Box<dyn Error>> {
    let captures = re.captures(&s).ok_or_else(|| std::io::Error::new(ErrorKind::InvalidInput, "unable to parse password"))?;
    Ok(Password {
        min: captures.get(1).unwrap().as_str().parse()?,
        max: captures.get(2).unwrap().as_str().parse()?,
        pattern: captures.get(3).unwrap().as_str().to_owned(),
        password: captures.get(4).unwrap().as_str().to_owned(),
    })
}

fn valid_by_rule_part_1(password: &Password) -> bool {
    let count = password.password.matches(&password.pattern).count() as i32;
    password.min <= count && count <= password.max
}


fn valid_by_rule_part_2(pw: &Password) -> bool {
    let min_range = ((pw.min - 1) as usize)..((pw.min) as usize);
    let max_range = ((pw.max - 1) as usize)..((pw.max) as usize);

    pw.password.get(min_range).contains(&pw.pattern)
        ^ pw.password.get(max_range).contains(&pw.pattern)
}


#[test]
fn test_day2() {
    print();
}


#[test]
fn test_valid_by_rule_part_2_1() {

    // 3-4 t: dttt
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dttt"),
    };

    assert!(!valid_by_rule_part_2(&pw));
}


#[test]
fn test_valid_by_rule_part_2_2() {

    // 3-4 t: dttx
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dttx"),
    };

    assert!(valid_by_rule_part_2(&pw));
}


#[test]
fn test_valid_by_rule_part_2_3() {

    // 3-4 t: dtxt
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dtxt"),
    };

    assert!(valid_by_rule_part_2(&pw));
}


#[test]
fn test_valid_by_rule_part_2_4() {

    // 3-4 t: dtxx
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dtxx"),
    };

    assert!(!valid_by_rule_part_2(&pw));
}


#[test]
fn test_valid_by_rule_part_1_1() {

    // 3-4 t: dttt
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dttt"),
    };

    assert!(valid_by_rule_part_1(&pw));
}


#[test]
fn test_valid_by_rule_part_1_2() {

    // 5-7 l: llmlqmblllh
    let pw: Password = Password {
        min: 5,
        max: 7,
        pattern: String::from("l"),
        password: String::from("llmlqmblllh"),
    };

    assert!(valid_by_rule_part_1(&pw));
}


#[test]
fn test_valid_by_rule_part_1_too_many() {

    // 3-10 g: gggxwxggggkgglklhhgg
    let pw: Password = Password {
        min: 3,
        max: 10,
        pattern: String::from("g"),
        password: String::from("gggxwxggggkgglklhhgg"),
    };

    assert!(!valid_by_rule_part_1(&pw));
}

#[test]
fn test_valid_by_rule_part_1_too_few() {

    // 8-10 g: gxwxgkgglklhhgg
    let pw: Password = Password {
        min: 8,
        max: 10,
        pattern: String::from("g"),
        password: String::from("gxwxgkgglklhhgg"),
    };

    assert!(!valid_by_rule_part_1(&pw));
}