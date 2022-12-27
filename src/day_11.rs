use regex::Regex;

use crate::utils::freadlines;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}
impl Operator {
    fn new(s: String) -> Self {
        match s.as_str() {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!("Invalid operation"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Operand {
    Old(i32),
    Value(i32),
}
impl Operand {
    fn new(s: String, current: i32) -> Self {
        if s.to_lowercase() == "old" {
            return Self::Old(current);
        }
        Self::Value(i32::from_str(&s).unwrap())
    }
}
#[derive(Debug, Clone, Copy)]
struct Operation {
    op: Operator,
    lhs: Operand,
    rhs: Operand,
}
impl Operation {
    fn new(op: String, lhs: String, rhs: String, current: i32) -> Self {
        Self {
            op: Operator::new(op),
            lhs: Operand::new(lhs, current),
            rhs: Operand::new(rhs, current),
        }
    }
    fn update(&mut self, current: i32) {
        self.rhs = match self.rhs {
            Operand::Old(_) => Operand::Old(current),
            Operand::Value(x) => Operand::Value(x),
        };
        self.lhs = match self.lhs {
            Operand::Old(_) => Operand::Old(current),
            Operand::Value(x) => Operand::Value(x),
        };
    }
    fn exec(self) -> i32 {
        let lhs = match self.lhs {
            Operand::Old(x) => x,
            Operand::Value(x) => x,
        };
        let rhs = match self.rhs {
            Operand::Old(rhs) => rhs,
            Operand::Value(rhs) => rhs,
        };
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}
#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    op: Operation,
    div: i32,
    target: (i32, i32),
    worry: i32,
    inspected: i32,
}
impl Monkey {
    fn parse(lines: &mut VecDeque<String>) -> Self {
        // TODO: actually learn regex
        let regexes = vec![
            // Regex::new(r"Monkey (\d{1,3})"),
            Regex::new(r"(\d+)"),
            Regex::new(r"Operation: new = ([a-z]{3}|\d+) (\S) ([a-z]{3}|\d+)"),
            Regex::new(r"Test: divisible by (\d+)"),
            Regex::new(r"If true: throw to monkey (\d+)"),
            Regex::new(r"If false: throw to monkey (\d+)"),
        ]
        .into_iter()
        .map(|r| r.unwrap())
        .collect::<Vec<Regex>>();
        let line = lines.pop_front().unwrap();
        // let n = regexes[0]
        // .captures(&line)
        // .unwrap()
        // .get(1)
        // .map_or("", |s| s.as_str());
        let line = lines.pop_front().unwrap();
        let items = regexes[0]
            .captures_iter(&line)
            .map(|s| s.get(1).unwrap().as_str().to_string())
            .map(|s| i32::from_str(&s).unwrap())
            .collect::<VecDeque<i32>>();

        let line = lines.pop_front().unwrap();
        let result = regexes[1].captures(&line).unwrap();
        let (lhs, op, rhs) = (
            result.get(1).unwrap().as_str(),
            result.get(2).unwrap().as_str(),
            result.get(3).unwrap().as_str(),
        );
        let operation = Operation::new(op.to_string(), lhs.to_string(), rhs.to_string(), 0);

        let line = lines.pop_front().unwrap();
        let test = regexes[2]
            .captures(&line)
            .unwrap()
            .get(1)
            .map(|s| s.as_str())
            .map(|s| i32::from_str(s).unwrap())
            .unwrap();
        let line = lines.pop_front().unwrap();
        let if_true = regexes[3]
            .captures(&line)
            .unwrap()
            .get(1)
            .map(|s| s.as_str())
            .map(|s| i32::from_str(s).unwrap())
            .unwrap();
        let line = lines.pop_front().unwrap();
        let if_false = regexes[4]
            .captures(&line)
            .unwrap()
            .get(1)
            .map(|s| s.as_str())
            .map(|s| i32::from_str(s).unwrap())
            .unwrap();
        Self {
            items: items,
            op: operation,
            div: test,
            target: (if_true, if_false),
            worry: 0,
            inspected: 0,
        }
    }

    fn inspect(&mut self) -> i32 {
        let item = self.items.pop_front().unwrap();
        self.op.update(item);
        let worry = self.op.exec() / 3;
        self.inspected += 1;
        worry
    }
    fn throw(&mut self, item: i32) -> (i32, i32) {
        let result = item % self.div == 0;
        if result {
            return (self.target.0, item);
        }
        (self.target.1, item)
    }
}

struct State {
    monkeys: VecDeque<Monkey>,
}
impl State {
    fn next(&mut self) {
        for i in 0..self.monkeys.len() {
            loop {
                if self.monkeys[i].items.len() <= 0 {
                    break;
                }
                let item = self.monkeys[i].inspect();
                let (target, item) = self.monkeys[i].throw(item);
                dbg!(target, item);
                self.monkeys[target as usize].items.push_back(item);
            }
        }
    }
    fn monkey_business(self) -> i32 {
        let mut inspections = vec![];
        self.monkeys
            .iter()
            .for_each(|m| inspections.push(m.inspected));
        inspections.sort();
        inspections[inspections.len() - 2..inspections.len()]
            .into_iter()
            .product()
    }
}
pub fn test11a() {
    let path = PathBuf::from_str("./src/data/11.txt").unwrap();
    let mut lines: VecDeque<String> = freadlines(path).into_iter().collect();
    let mut monkeys = VecDeque::new();
    while (lines.len() > 0) {
        monkeys.push_back(Monkey::parse(&mut lines));
        lines.pop_front();
        // break;
    }
    let mut state = State { monkeys };
    for _ in 0..20 {
        state.next();
    }
    for monkey in &state.monkeys {
        dbg!(monkey.inspected);
    }
    println!("11a: {}", state.monkey_business());
}
pub fn test11b() {}
