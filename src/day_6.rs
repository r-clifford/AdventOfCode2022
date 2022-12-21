use std::{collections::HashSet, path::PathBuf, str::FromStr};

use crate::utils::freadlines;

fn find_marker(chars: Vec<char>, size: usize) -> usize {
    let mut i = size;
    for window in chars.windows(size) {
        if is_marker(window) {
            return i;
        }
        i += 1;
    }
    return 0;
}
fn is_marker(w: &[char]) -> bool {
    let len = w.len();
    let set = HashSet::<_>::from_iter(w);
    set.len() == len
}
fn solution6a() {
    let path = PathBuf::from_str("./src/data/6.txt").unwrap();
    for line in freadlines(path) {
        let chars: Vec<char> = line.chars().collect();
        dbg!(&chars.len());
        let marker = find_marker(chars, 4);
        println!("{}", marker);
    }
}

fn solution6b() {
    let path = PathBuf::from_str("./src/data/6.txt").unwrap();
    for line in freadlines(path) {
        let chars: Vec<char> = line.chars().collect();
        dbg!(&chars.len());
        let marker = find_marker(chars, 14);
        println!("{}", marker);
    }
}
pub fn test6a() {
    solution6a();
}
pub fn test6b() {
    solution6b();
}
