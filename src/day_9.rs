use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Add, Neg, Sub},
    path::PathBuf,
    str::FromStr,
};

use crate::utils::freadlines;

#[derive(Debug, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Hash)]
struct Movement {
    direction: Direction,
    distance: usize,
}
impl Movement {
    fn from_str<'a>(s: &'a str) -> Self {
        let chars = s.split_at(2);
        let dir = chars.0.chars().next().unwrap();
        let mag = usize::from_str(chars.1).unwrap();
        let direction = match dir {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        Self {
            direction,
            distance: mag,
        }
    }
}
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Position(isize, isize);
impl Position {
    fn x(&self) -> isize {
        self.0
    }
    fn y(&self) -> isize {
        self.1
    }
    fn up(&mut self) {
        self.1 += 1;
    }
    fn down(&mut self) {
        self.1 -= 1;
    }
    fn left(&mut self) {
        self.0 -= 1;
    }
    fn right(&mut self) {
        self.0 += 1;
    }
    fn exec(&mut self, movement: &mut Movement) {
        let n = &mut movement.distance;
        assert!(*n > 0);
        match movement.direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
        *n -= 1;
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Neg for Position {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
// impl PartialEq for Position {
//     fn eq(&self, other: &Self) -> bool {
//         (self.0 == other.0) && (self.1 == other.1)
//     }
// }
// impl Eq for Position {}
#[derive(Debug, Clone, Default)]
struct Visited {
    positions: HashSet<Position>,
}

#[derive(Debug, Clone, Default)]
struct State {
    head: Position,
    tail: Position,
    visited: Visited,
}
impl State {
    fn new() -> Self {
        Default::default()
    }
    fn next(&mut self, movement: &mut Movement) {
        // dbg!(*movement);
        while movement.distance > 0 {
            let old = self.head;
            self.head.exec(movement);
            let diff = self.head - self.tail;
            if (isize::abs(diff.0) > 1) || (isize::abs(diff.1) > 1) {
                self.tail = old;
                // match isize::signum(diff.0) {
                //     1 => self.tail.right(),
                //     -1 => self.tail.left(),
                //     _ => {},
                // }
                // match isize::signum(diff.1) {
                //    1 => self.tail.up(),
                //    -1 => self.tail.down(),
                //    _ => {},
                // }
                let inserted = self.visited.positions.insert(self.tail);
            }

        // dbg!(self.head.0, self.head.1, self.tail.0, self.tail.1);
        }
    }
}
pub fn test9a() {
    let path = PathBuf::from_str("./src/data/9.txt").unwrap();
    let lines = freadlines(path);
    let mut state = State::new();
    state.visited.positions.insert(state.tail);
    dbg!(&state);
    let mut i = 0;
    for line in lines {
        let mut movement = Movement::from_str(&line);
        i += movement.distance;
        state.next(&mut movement);
        // dbg!(state.head, state.tail);
    }
    dbg!(i);
    println!("9b: {}", state.visited.positions.len());
}
// pub fn test9a() {
//     let path = PathBuf::from_str("./src/data/9.txt").unwrap();
//     let lines = freadlines(path);
//     let mut head = (0,0);
//     let mut tail = (0,0);
//     let mut visited = HashSet::<(i32, i32)>::new();
//     visited.insert(tail);
//     let mut i = 0;
//     for line in lines {
//         let chars = line.split_at(2);
//         let dir = chars.0.chars().next().unwrap();
//         let mag = i32::from_str(chars.1).unwrap();

//         for _ in 0..mag {
//             i+=1;
//             let old = head;
//             head = match dir {
//                 'L' => (head.0 -1, head.1),
//                 'R' => (head.0 + 1, head.1),
//                 'U' => (head.0, head.1 + 1),
//                 'D' => (head.0, head.1 - 1),
//                 _ => panic!("Invalid move"),
//             };
//             let (x,y) = (head.0 - tail.0,head.1 - tail.1);
//             if (i32::abs(x)> 1) || (i32::abs(y) > 1) {
//                 // tail = old;
//                 tail = (tail.0 + i32::signum(x), tail.1 + i32::signum(y));
//                 visited.insert(tail);
//             }
//         }

//     }
//     dbg!(i);
//     println!("9a: {}", visited.len());
// }
