use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn solution1(path: PathBuf) -> i32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut max_cal = 0;
    let mut count = 0;
    for line in lines {
        match i32::from_str_radix(&line.unwrap(), 10) {
            Ok(i) => {
                count += i;
            },
            Err(_) => {
                if count > max_cal {
                    max_cal = count;
                }
                count = 0;
            }
        }
    }
    return max_cal;
}
pub fn test1() {
    let path = match PathBuf::from_str("./src/data/1.txt") {
        Ok(p) => p,
        Err(_) => panic!("Invalid path"),
    };
    println!("{}", solution1(path));
}
