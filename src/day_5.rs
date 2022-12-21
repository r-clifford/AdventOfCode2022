use std::path::PathBuf;
use std::collections::VecDeque;
use std::str::{FromStr, Chars};
use crate::utils::freadlines;

type Stack = Vec<char>;
#[derive(Debug, Clone )]
struct StackState {
    data: Vec<Stack>,
    n: usize
}
impl StackState {
    fn exec(&mut self, cmd: String) {
        let split = cmd.split(" ").collect::<Vec<&str>>();
        let n = usize::from_str( split[1]).unwrap();
        let src = usize::from_str(split[3]).unwrap() - 1;
        let dst = usize::from_str(split[5]).unwrap() - 1;
        
        for _ in 0..n {
            let moved = self.data[src].pop().unwrap();
            self.data[dst].push(moved);
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
            data.push(vec![]);
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
                    if (i-1) % field_offset == 0 {
                        if c.is_alphabetic() {
                            data[(i-1) / 4].push(c);
                        }
                    }
                }
            }
            line = v.pop_back();
        }

        Self {
            data,
            n,
        }
    }
}
fn parse_state(path: PathBuf) -> StackState {
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
    dbg!(&state);
    for i in n+1..lines.len() {
        dbg!(i, &lines[i]);
        state.exec(lines[i].clone());
    }
    return state;
}
pub fn test5a() {
    let path = PathBuf::from_str("./src/data/5.txt").unwrap();
    let mut state = parse_state(
        path
    );
    println!("{}", state.output());
}