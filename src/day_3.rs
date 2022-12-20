use std::hash::Hash;
use std::path::PathBuf;
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;

fn parse_file(path: PathBuf) -> Vec<String> {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    lines.map(|l| l.unwrap()).collect()
}
fn find_dup(s1: String, s2: String) -> char {
    let set1 = HashSet::<_>::from_iter(s1.chars());    
    let set2 = HashSet::<_>::from_iter(s2.chars());
    let dups = set1.intersection(&set2);
    *dups.to_owned().next().unwrap() 
}
fn get_prio(c: char) -> i32 {
    let mut prio = 0;
    if c.is_ascii_lowercase() {
        prio = (c as u8) - ('a' as u8) + 1;
    } else {
        prio = (c as u8) - ('A' as u8) + 27;
    }
    return prio as i32;
}
fn find_dups(v: &[String]) -> HashSet<char> {
    let maps = v.into_iter()
        .map(|s| 
            HashSet::<char>::from_iter(s
                .chars()));
    maps.reduce(|acc, s| {
        acc.intersection(&s).map(|c| *c).collect()
    }).unwrap()
}
fn solution3a() -> i32 {
    let lines = parse_file(PathBuf::from_str("./src/data/3.txt").unwrap());
    let dups = lines.iter().map(|s| {
        let (first, second) = s.split_at(s.len() / 2);
        find_dup(first.to_string(), second.to_string())
    }).collect::<Vec<char>>();
    dups.iter().fold(0, |acc, c| acc + get_prio(*c))
}
fn solution3b() -> i32 {
    let lines = parse_file(PathBuf::from_str("./src/data/3.txt").unwrap());
    let mut total = 0;
    for group in lines.chunks(3) {
        let dups = find_dups(group).into_iter().next().unwrap();
        total += get_prio(dups);
    }
    return total;
}
pub fn test3a() {
    println!("{}", solution3a());
}

pub fn test3b() {
    println!("{}", solution3b());
}