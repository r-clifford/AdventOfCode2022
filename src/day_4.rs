use std::path::PathBuf;
use std::str::FromStr;
use crate::utils::freadlines;

struct Range {
    lower: u32,
    upper: u32,
}
impl Range {
    fn contains(&self, other: &Self) -> bool {
        (self.lower <= other.lower) && (self.upper >= other.upper)
    }
    fn overlaps(&self, other: &Self) -> bool {
        if self.lower <= other.lower {
            return self.upper >= other.lower;
        } else {
            return self.lower <= other.upper;
        }
    }
}

struct Pair(Range, Range);
fn parse_pair(s: String) -> Pair {
    let mut split = s.split(",");
    let first = split.next().unwrap().to_string();
    let second = split.next().unwrap().to_string();

    let first = parse_range(first);
    let second = parse_range(second);
    Pair(first, second)
}
fn parse_range(s: String) -> Range {
    let mut split = s.split("-");

    let lower = split.next().unwrap().parse::<u32>().unwrap();
    let upper = split.next().unwrap().parse::<u32>().unwrap();

    Range {
        lower,
        upper,
    }
}
fn test_pair(acc: i32, s: String) -> i32 {
    let pair = parse_pair(s);
    if pair.0.contains(&pair.1) || (pair.1.contains(&pair.0)) {
        return acc + 1;
    }
    acc
}
fn solution4a() {
    let lines = freadlines(
        PathBuf::from_str("./src/data/4.txt").unwrap()
    );
    let result = lines.into_iter().fold(0, test_pair );
    println!("{}", result);
    
}
fn solution4b() {
    let lines = freadlines(
        PathBuf::from_str("./src/data/4.txt").unwrap()
    );
    let result = lines.into_iter().fold(0, |acc, s| {
        let pair = parse_pair(s);
        if pair.0.overlaps(&pair.1) {
            return acc + 1;
        }
        acc
    } );
    println!("{}", result);
 
}
pub fn test4a() {
    solution4a();
}
pub fn test4b() {
    solution4b();
}