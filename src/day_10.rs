use core::prelude;
use std::{path::PathBuf, str::FromStr};

use crate::utils::freadlines;


#[derive(Debug, Clone, Default)]
struct CPU {
    x: isize,
    cycle: isize,
    critical_cycles: Vec<isize>,
}
impl CPU {
    fn addx(&mut self, n: isize) -> Option<isize> {
        let mut ret = None;
        ret = self.noop().or(ret);
        ret = self.noop().or(ret);
        self.x += n;
        ret
    }
    fn noop(&mut self) -> Option<isize> {
        self.cycle += 1;
        self.strength()
    }
    fn strength(&self) -> Option<isize> {
        let strength = self.x * self.cycle;
        if self.critical_cycles.contains(&self.cycle) {
            dbg!(self, strength);
            return Some(strength);
        }
        None
    }
    fn execute(&mut self, cmd: String) -> Option<isize> {
        let split = cmd.split_at(4);
        match split {
            ("addx", n) => self.addx(isize::from_str(n.trim()).unwrap()),
            ("noop", _) => self.noop(),
            _ => panic!("Invalid instruction"),
        }
    }

}
pub fn test10a() {
    let path = PathBuf::from_str("./src/data/10.txt").unwrap();
    let lines = freadlines(path);
    let mut cpu = CPU::default();
    cpu.x = 1;
    cpu.critical_cycles = vec![20, 60, 100, 140, 180, 220];//.iter().map(|i| i-1).collect();
    let mut total = 0;
    for line in lines {
        match cpu.execute(line) {
            Some(n) => total += n,
            _ => {}
        };
    }
    println!("10a: {}", total);
    
}

pub fn test10b() {

}