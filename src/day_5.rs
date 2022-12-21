use crate::utils::freadlines;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::{Chars, FromStr};

type Stack = Vec<char>;
#[derive(Debug, Clone)]
struct StackState {
    data: Vec<Stack>,
    n: usize,
}
impl StackState {
    fn exec(&mut self, cmd: String, seq: bool) {
        let split = cmd.split(" ").collect::<Vec<&str>>();
        let n = usize::from_str(split[1]).unwrap();
        let src = usize::from_str(split[3]).unwrap() - 1;
        let dst = usize::from_str(split[5]).unwrap() - 1;
        if seq {
            for _ in 0..n {
                let moved = self.data[src].pop().unwrap();
                self.data[dst].push(moved);
            }
        } else {
            let upper = self.data[src].len();
            let lower = upper - n;
            let split = self.data[src].split_at(lower);
            let remaining = split.0.to_owned();
            let moved = split.1.to_owned();
            self.data[src] = remaining.to_vec();
            self.data[dst].extend(moved);
        }
    }
    fn output(&mut self) -> String {
        let mut s = String::new();
        for i in 0..self.n {
            let c = match self.data[i].pop() {
                Some(x) => x,
                None => ' ',
            };
            s.push(c);
        }
        return s;
    }
}
impl StackState {
    fn new(mut v: VecDeque<Chars>, n: usize) -> Self {
        let mut data: Vec<Stack> = Vec::new();
        for _ in 0..n {
            data.push(Vec::with_capacity(20));
        }
        let mut line = v.pop_back();
        let field_offset = 4;
        loop {
            if !line.is_some() {
                break;
            }
            let l = line.unwrap();
            for (i, c) in l.enumerate() {
                if i > 0 {
                    if (i - 1) % field_offset == 0 {
                        if c.is_alphabetic() {
                            data[(i - 1) / 4].push(c);
                        }
                    }
                }
            }
            line = v.pop_back();
        }

        Self { data, n }
    }
}
fn parse_state(path: PathBuf, seq: bool) -> StackState {
    let lines = freadlines(path);
    let mut initial_state = VecDeque::new();

    for line in &lines {
        if line.starts_with("move") {
            break;
        } else {
            initial_state.push_back(line.chars())
        }
    }
    initial_state.pop_back();
    let field_offset = 4;
    let n = initial_state.pop_back().unwrap().count() / field_offset + 1;
    let mut state = StackState::new(initial_state, n);
    for i in n + 1..lines.len() {
        state.exec(lines[i].clone(), seq);
    }
    return state;
}
pub fn test5a() {
    let path = PathBuf::from_str("./src/data/5.txt").unwrap();
    let mut state = parse_state(path, true);
    println!("{}", state.output());
}

pub fn test5b() {
    let path = PathBuf::from_str("./src/data/5.txt").unwrap();
    let mut state = parse_state(path, false);
    println!("{}", state.output());
}
