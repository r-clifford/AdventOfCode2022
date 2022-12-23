use std::{path::PathBuf, str::FromStr};

use crate::utils::freadlines;


#[derive(Debug, Clone, Default)]
struct CPU {
    x: isize,
    cycle: isize,
    critical_cycles: Vec<isize>,
    display: Display,
    use_display: bool,
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
        self.next();
        self.strength()
    }
    fn strength(&self) -> Option<isize> {
        let strength = self.x * self.cycle;
        if self.critical_cycles.contains(&self.cycle) {
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
    fn next(&mut self) {
        self.cycle += 1;
        if self.use_display {
            self.display.draw(self.x, self.cycle);
        }
    }

}


#[derive(Debug, Clone, Default)]
struct Line(Vec<char>);
impl Line {
    fn new(n: usize) -> Self {
        Self((0..n).map(|_| '.').collect())
    }
}
#[derive(Debug, Clone, Default)]
struct Display {
    x: usize,
    y: usize,
    data: Line,
}
impl Display {
    fn init(x: usize, y: usize) -> Self {
        let mut ret = Self::default();
        ret.x = x;
        ret.y = y;
        ret.data = Line::new(x);
        ret
    }
    fn draw(&mut self, x: isize, cycle: isize) {
        let sprite = vec![x-1, x, x+1];
        let ptr = (cycle-1) % 40;
        for i in sprite {
            if i == ptr {
                self.data.0[i as usize] = '#';
            }
        }
        if cycle % 40 == 0 {
            println!("{} {}", self.data.0.iter().collect::<String>(), cycle);
            *self = Display::init(self.x, self.y);

        }
    }
}
pub fn test10a() {
    let path = PathBuf::from_str("./src/data/test.txt").unwrap();
    let lines = freadlines(path);
    let mut cpu = CPU::default();
    cpu.use_display = false;
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
    let path = PathBuf::from_str("./src/data/10.txt").unwrap();
    let lines = freadlines(path);
    let mut cpu = CPU::default();
    cpu.use_display = true;
    let (x, y) = (40, 6);
    cpu.x = 1;
    cpu.critical_cycles = vec![20, 60, 100, 140, 180, 220];//.iter().map(|i| i-1).collect();
    cpu.display = Display::init(x, y);
    for line in lines {
        cpu.execute(line);
    }
    dbg!(cpu.cycle);
}