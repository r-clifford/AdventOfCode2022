use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Sub;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
enum PlayResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
}
#[derive(Debug, Clone, Copy)]
enum Plays {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Plays {
    fn from_char(c: &char) -> Self {
        match c {
            'A' | 'X' => Plays::Rock,
            'B' | 'Y' => Plays::Paper,
            'C' | 'Z' => Plays::Scissors,
            _ => panic!("Invalid play"),
        }
    }
    fn test(&self, opponent: &Self) -> PlayResult {
        match *self - *opponent {
            0 => PlayResult::Draw,
            -1 | 2 => PlayResult::Loss,
            -2 | 1 => PlayResult::Win,
            _ => panic!("Invalid play"),
        }
    }
}
impl Sub for Plays {
    type Output = i32;
    fn sub(self, rhs: Self) -> Self::Output {
        (self as i32) - (rhs as i32)
    }
}

fn calculate_score(path: PathBuf) -> i32 {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let split = line.split(" ").collect::<Vec<&str>>();
        let player = Plays::from_char(&split[1].chars().next().unwrap());
        let opponent = Plays::from_char(&split[0].chars().next().unwrap());
        let result = player.test(&opponent);
        let score = (result as i32) + (player as i32);
        total += score;
    }
    return total;
}

fn solution2() {
    let path = PathBuf::from_str("./src/data/2.txt").unwrap();
    print!("{}", calculate_score(path));
}
pub fn test2a() {
    solution2();
}
