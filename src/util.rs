use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> HashSet<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|s| s.parse().unwrap())
        .collect()
}