use crate::utils::freadlines;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::VecDeque, path::PathBuf, str::FromStr};

// TODO: possibly revisit, many problems with implementation

#[derive(Debug, Clone)]
struct Row(VecDeque<Rc<RefCell<Tree>>>);
#[derive(Debug, Clone, Copy)]
struct Tree {
    height: isize,
    detected: bool,
}
impl Tree {
    fn get_visibility(&mut self) {

    }
}
impl Row {
    fn n_visible(&self) -> isize {
        // try l->r
        let mut maximum = -1;
        let mut n = 0;
        for i in self.0.iter() {
            if i.borrow().height > maximum {
                if !i.borrow().detected {
                    n += 1;
                    (*i).borrow_mut().detected = true;
                }
                maximum = i.borrow().height;
            }
        }
        maximum = 0;
        for i in self.0.iter().rev() {
            if i.borrow().height > maximum {
                if !i.borrow().detected {
                    n += 1;
                    i.borrow_mut().detected = true;
                }
                maximum = i.borrow().height;
            }
        }
        n
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl FromIterator<Tree> for Row {
    fn from_iter<T: IntoIterator<Item = Tree>>(iter: T) -> Self {
        Row(VecDeque::from_iter(
            iter.into_iter().map(|a| Rc::new(RefCell::new(a))),
        ))
    }
}
struct Forest {
    xsize: isize,
    ysize: isize,
    data: VecDeque<Row>,
}
impl Forest {
    fn row_from_string(s: String) -> Row {
        s.chars()
            .into_iter()
            .map(|c| Tree {
                height: isize::from_str(&c.to_string()).unwrap(),
                detected: false,
            })
            .collect()
    }
    fn new(forest: Vec<String>) -> Self {
        let data: VecDeque<Row> = forest
            .into_iter()
            .map(|s| Forest::row_from_string(s))
            .collect();

        let xsize: isize = data[0].len().try_into().unwrap();
        let ysize: isize = data.len().try_into().unwrap();

        Self { xsize, ysize, data }
    }
    fn cols(&self) -> VecDeque<Row> {
        let mut ret = VecDeque::with_capacity(self.xsize as usize);
        for i in 0..(self.xsize as usize) {
            let mut col = Row(VecDeque::with_capacity(self.ysize as usize));
            for j in 0..(self.ysize as usize) {
                col.0.push_back(self.data[j].0[i].clone());
            }
            ret.push_back(col);
        }
        ret
    }
    fn set_edges(&mut self) -> isize {
        // prob refactor
        let mut total = 0;
        for i in self.data[0].0.iter_mut() {
            if !i.borrow().detected {
                i.borrow_mut().detected = true;
                total += 1;
            }
        }
        for i in self.data[(self.ysize - 1) as usize].0.iter_mut() {
            if !i.borrow().detected {
                i.borrow_mut().detected = true;
                total += 1;
            }
        }
        for i in self.data.iter_mut() {
            let front = &i.0[0];
            if !front.borrow().detected {
                front.borrow_mut().detected = true;
                total += 1;
            }

            let back = &i.0[(self.xsize - 1) as usize];
            if !back.borrow().detected {
                back.borrow_mut().detected = true;
                total += 1;
            }
        }

        total
    }
    fn solve(&mut self) -> isize {
        let mut total = self.set_edges();
        // TODO: useless check on boundarys
        for row in self.data.iter_mut() {
            total += row.n_visible();
        }
        for col in self.cols().iter_mut() {
            total += col.n_visible();
        }
        total
    }
    fn find_score(data: &VecDeque<Row>, x: usize, y: usize) -> usize {
        let row = &data[y];
        let mut right = 0;
        let mut current = 0;
        let height = row.0[x].borrow().height;
        // test right
        for i in (x+1)..(row.len())  {
            current = row.0[i].borrow().height;
            right += 1;
            if current >= height {
                break;
            }
        }
        // test left
        let mut left = 0;
        for i in (0..x).rev() {
            current = row.0[i].borrow().height;
            left += 1;
            if current >= height {
                break;
            }
        }
        // test up
        let mut up = 0;
        for i in (0..y).rev() {
            current = data[i].0[x].borrow().height;
            up += 1;
            if current >= height {
                break;
            }
        }
        // test down
        let mut down = 0;
        for i in y+1..(data.len()) {
            current = data[i].0[x].borrow().height;
            down += 1;
            if current >= height {
                break;
            }
        }

        return left * right * up * down;
        
    }
    fn max_score(&self) -> usize {
        let mut max: usize = 0;
        for (y, row) in self.data.iter().enumerate() {
            for (x, tree) in row.0.iter().enumerate() {
                let score = Forest::find_score(&self.data, x, y);
                if score > max {
                    max = score
                }
            }
        }
        max
    }
}

fn solution8a() -> isize {
    let path = PathBuf::from_str("./src/data/8.txt").unwrap();
    let lines = freadlines(path);
    let mut forest = Forest::new(lines);
    let result = forest.solve();
    dbg!(&result);
    println!("Solution b: {}", forest.max_score());
    result
}

pub fn test8a() {
    println!("Solution a: {}", solution8a());
}

