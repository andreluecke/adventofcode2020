use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

struct Password {
    min: i32,
    max: i32,
    pattern: String,
    password: String,
}


pub fn print() {
    println!("Not yet implemented");
    let re: Regex = Regex::new(r"^(\d+)-(\d+) ([A-Za-z]+): (\w+)$").unwrap();

    let file = File::open("resources/input_day_2").expect("no such file");
    let buf = BufReader::new(file);
    let count = buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|s| parse_password(&s, &re))
        .filter(|pw| valid(pw))
        .count();

    println!("Day 2: {}", count); // 462
}


fn parse_password(s: &String, re: &Regex) -> Password {
    let captures = re.captures(&s).unwrap();
    Password {
        min: captures.get(1).unwrap().as_str().parse().unwrap(),
        max: captures.get(2).unwrap().as_str().parse().unwrap(),
        pattern: captures.get(3).unwrap().as_str().parse().unwrap(),
        password: captures.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn valid(password: &Password) -> bool {
    let count = password.password.matches(&password.pattern).count() as i32;
    password.min <= count && count <= password.max
}

#[test]
fn test_day1_part1() {
    print();

    // assert_eq!(day1_part1(&numbers), 864864);
}

#[test]
fn test_valid_pw_1() {

    // 3-4 t: dttt
    let pw: Password = Password {
        min: 3,
        max: 4,
        pattern: String::from("t"),
        password: String::from("dttt"),
    };

    assert!(valid(&pw));
}


#[test]
fn test_valid_pw_2() {

    // 5-7 l: llmlqmblllh
    let pw: Password = Password {
        min: 5,
        max: 7,
        pattern: String::from("l"),
        password: String::from("llmlqmblllh"),
    };

    assert!(valid(&pw));
}


#[test]
fn test_invalid_pw_too_many() {

    // 3-10 g: gggxwxggggkgglklhhgg
    let pw: Password = Password {
        min: 3,
        max: 10,
        pattern: String::from("g"),
        password: String::from("gggxwxggggkgglklhhgg"),
    };

    assert!(!valid(&pw));
}

#[test]
fn test_invalid_pw_too_few() {

    // 8-10 g: gxwxgkgglklhhgg
    let pw: Password = Password {
        min: 8,
        max: 10,
        pattern: String::from("g"),
        password: String::from("gxwxgkgglklhhgg"),
    };

    assert!(!valid(&pw));
}