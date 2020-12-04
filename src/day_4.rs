use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_HGT: Regex = Regex::new("^(\\d+)(cm|in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new("^(\\d{9})$").unwrap();
}


pub fn print() {
    println!("Day 4, Part 1: {}", day4_part1()); // 204
    println!("Day 4, Part 1: {}", day4_part2()); // 183
}

fn day4_part1() -> i32 {
    let file = File::open("resources/input_day_4").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut count = 0;
    let mut fields = HashMap::new();

    for line in lines {
        let string = line.unwrap();
        if (&string).is_empty() {
            if fields.contains_key("byr")
                && fields.contains_key("iyr")
                && fields.contains_key("eyr")
                && fields.contains_key("hgt")
                && fields.contains_key("hcl")
                && fields.contains_key("ecl")
                && fields.contains_key("pid") {
                count += 1;
            }
            fields = HashMap::new();
        } else {
            for kv_pair in string.split_ascii_whitespace() {
                kv_pair.split_once(':')
                    .and_then(|tuple| fields.insert(String::from(tuple.0), String::from(tuple.1)));
            };
        }
    }
    if !fields.is_empty() {
        if fields.contains_key("byr")
            && fields.contains_key("iyr")
            && fields.contains_key("eyr")
            && fields.contains_key("hgt")
            && fields.contains_key("hcl")
            && fields.contains_key("ecl")
            && fields.contains_key("pid") {
            count += 1;
        }
    }
    return count;
}


fn day4_part2() -> i32 {
    let file = File::open("resources/input_day_4").expect("no such file");
    let lines = BufReader::new(&file).lines();

    let mut count = 0;
    let mut fields = HashMap::new();

    for line in lines {
        let string = line.unwrap();
        if (&string).is_empty() {
            if fields.contains_key("byr")
                && fields.contains_key("iyr")
                && fields.contains_key("eyr")
                && fields.contains_key("hgt")
                && fields.contains_key("hcl")
                && fields.contains_key("ecl")
                && fields.contains_key("pid")
                && is_valid(&fields) {
                count += 1;
            }
            fields = HashMap::new();
        } else {
            for kv_pair in string.split_ascii_whitespace() {
                kv_pair.split_once(':')
                    .and_then(|tuple| fields.insert(String::from(tuple.0), String::from(tuple.1)));
            };
        }
    }
    if !fields.is_empty() {
        if fields.contains_key("byr")
            && fields.contains_key("iyr")
            && fields.contains_key("eyr")
            && fields.contains_key("hgt")
            && fields.contains_key("hcl")
            && fields.contains_key("ecl")
            && fields.contains_key("pid")
            && is_valid(&fields) {
            count += 1;
        }
    }
    return count;
}

fn is_valid(fields: &HashMap<String, String>) -> bool {
    let mut result = true;
    for (k, v) in fields.iter() {
        match k.as_str() {
            "byr" => result &= valid_byr(&v),
            "iyr" => result &= valid_iyr(&v),
            "eyr" => result &= valid_eyr(&v),
            "hgt" => result &= valid_hgt(&v),
            "hcl" => result &= valid_hcl(&v),
            "ecl" => result &= valid_ecl(&v),
            "pid" => result &= valid_pid(&v),
            _ => {}
        }
        if !result { return false; }
    }
    return result;
}

fn valid_byr(string: &String) -> bool {
    return match string.parse::<i32>() {
        Ok(value) => 1920 <= value && value <= 2002,
        Err(_) => false
    };
}

fn valid_iyr(string: &String) -> bool {
    return match string.parse::<i32>() {
        Ok(value) => 2010 <= value && value <= 2020,
        Err(_) => false
    };
}

fn valid_eyr(string: &String) -> bool {
    return match string.parse::<i32>() {
        Ok(value) => 2020 <= value && value <= 2030,
        Err(_) => false
    };
}

fn valid_hgt(string: &String) -> bool {
    if !RE_HGT.is_match(string) {
        return false;
    }

    let captures = RE_HGT.captures(&string).unwrap();
    let unit = captures.get(2).unwrap().as_str();
    let value = captures.get(1).unwrap().as_str();
    return if unit == "cm" {
        match value.parse::<i32>() {
            Ok(n) => 150 <= n && n <= 193,
            Err(_) => false
        }
    } else if unit == "in" {
        match value.parse::<i32>() {
            Ok(n) => 59 <= n && n <= 76,
            Err(_) => false
        }
    } else {
        false
    };
}

fn valid_hcl(string: &String) -> bool {
    return RE_HCL.is_match(&string);
}

fn valid_ecl(string: &String) -> bool {
    return RE_ECL.is_match(&string);
}

fn valid_pid(string: &String) -> bool {
    return RE_PID.is_match(&string);
}


#[test]
fn test_day4() {
    print();
}


#[test]
fn test_byr() {
    assert_eq!(valid_byr(&String::from("1919")), false);
    assert_eq!(valid_byr(&String::from("1920")), true);
    assert_eq!(valid_byr(&String::from("2002")), true);
    assert_eq!(valid_byr(&String::from("2003")), false);
    assert_eq!(valid_byr(&String::from("")), false);
    assert_eq!(valid_byr(&String::from("x")), false);
}

#[test]
fn test_iyr() {
    assert_eq!(valid_iyr(&String::from("2009")), false);
    assert_eq!(valid_iyr(&String::from("2010")), true);
    assert_eq!(valid_iyr(&String::from("2020")), true);
    assert_eq!(valid_iyr(&String::from("2021")), false);
    assert_eq!(valid_iyr(&String::from("")), false);
    assert_eq!(valid_iyr(&String::from("x")), false);
}

#[test]
fn test_eyr() {
    assert_eq!(valid_eyr(&String::from("2019")), false);
    assert_eq!(valid_eyr(&String::from("2020")), true);
    assert_eq!(valid_eyr(&String::from("2030")), true);
    assert_eq!(valid_eyr(&String::from("2031")), false);
    assert_eq!(valid_eyr(&String::from("")), false);
    assert_eq!(valid_eyr(&String::from("x")), false);
}

#[test]
fn test_hgt() {
    assert_eq!(valid_hgt(&String::from("149cm")), false);
    assert_eq!(valid_hgt(&String::from("150cm")), true);
    assert_eq!(valid_hgt(&String::from("193cm")), true);
    assert_eq!(valid_hgt(&String::from("194cm")), false);
    assert_eq!(valid_hgt(&String::from("58in")), false);
    assert_eq!(valid_hgt(&String::from("59in")), true);
    assert_eq!(valid_hgt(&String::from("76in")), true);
    assert_eq!(valid_hgt(&String::from("77in")), false);
    assert_eq!(valid_hgt(&String::from("")), false);
    assert_eq!(valid_hgt(&String::from("x")), false);
    assert_eq!(valid_hgt(&String::from("xcm")), false);
    assert_eq!(valid_hgt(&String::from("xin")), false);
    assert_eq!(valid_hgt(&String::from("150m")), false);
    assert_eq!(valid_hgt(&String::from("60i")), false);
}

#[test]
fn test_hcl() {
    assert_eq!(valid_hcl(&String::from("#123abc")), true);
    assert_eq!(valid_hcl(&String::from("#123456")), true);
    assert_eq!(valid_hcl(&String::from("#056789")), true);
    assert_eq!(valid_hcl(&String::from("#abcfef")), true);
    assert_eq!(valid_hcl(&String::from("#12345")), false);
    assert_eq!(valid_hcl(&String::from("#1234567")), false);
    assert_eq!(valid_hcl(&String::from("#123abz")), false);
    assert_eq!(valid_hcl(&String::from("#gggggg")), false);
    assert_eq!(valid_hcl(&String::from("#abcde")), false);
    assert_eq!(valid_hcl(&String::from("#abcdeff")), false);
    assert_eq!(valid_hcl(&String::from("123abc")), false);
}

#[test]
fn test_ecl() {
    // amb blu brn gry grn hzl oth
    assert_eq!(valid_ecl(&String::from("amb")), true);
    assert_eq!(valid_ecl(&String::from("blu")), true);
    assert_eq!(valid_ecl(&String::from("brn")), true);
    assert_eq!(valid_ecl(&String::from("gry")), true);
    assert_eq!(valid_ecl(&String::from("grn")), true);
    assert_eq!(valid_ecl(&String::from("hzl")), true);
    assert_eq!(valid_ecl(&String::from("oth")), true);
    assert_eq!(valid_ecl(&String::from("amb blu")), false);
    assert_eq!(valid_ecl(&String::from("ambbrn")), false);
    assert_eq!(valid_ecl(&String::from("#am")), false);
    assert_eq!(valid_ecl(&String::from("")), false);
    assert_eq!(valid_ecl(&String::from("xyz")), false);
}

#[test]
fn test_pid() {
    // amb blu brn gry grn hzl oth
    assert_eq!(valid_pid(&String::from("000000001")), true);
    assert_eq!(valid_pid(&String::from("000000000")), false);
    assert_eq!(valid_pid(&String::from("123456789")), true);
    assert_eq!(valid_pid(&String::from("1234567890")), false);
    assert_eq!(valid_pid(&String::from("0123456789")), false);
    assert_eq!(valid_pid(&String::from("00000001")), false);
    assert_eq!(valid_pid(&String::from("0000000001")), false);
    assert_eq!(valid_pid(&String::from("a00000001")), false);
    assert_eq!(valid_pid(&String::from("00000a001")), false);
}