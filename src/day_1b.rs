use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn solution1b(path: PathBuf) -> i32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut count = 0;
    let mut counts = vec![];

    for line in lines {
        match i32::from_str_radix(&line.unwrap(), 10) {
            Ok(i) => {
                count += i;
            },
            Err(_) => {
                counts.push(count);
                count = 0;
            }
        }
    }
    counts.sort();
    let mut result = 0;
    let mut iter = counts.iter().rev();
    for _ in 0..3 {
        let i = iter.next().unwrap();
        result += i;
    }
    return result;
}
pub fn test1b() {
    let path = match PathBuf::from_str("./src/data/1.txt") {
        Ok(p) => p,
        Err(_) => panic!("Invalid path"),
    };
    println!("{}", solution1b(path));
}
