use std::{rc::Rc, cell::RefCell};
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::freadlines;

#[derive(Debug, Clone, Copy, Default)]
struct Visibility {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
    score: usize,
}
impl Visibility {
    fn new() -> Self {
        Default::default()
    }
}
type TreeNode = Rc<RefCell<Option<Tree>>>;
fn rcrf_none() -> TreeNode {
    Rc::new(
        RefCell::new(
            None
        )
    )
}
fn create_unlinked_node(height: usize) -> TreeNode {
    let tree = Tree {
        height,
        up: rcrf_none(),
        down: rcrf_none(),
        left: rcrf_none(),
        right: rcrf_none(),
        visibility: Visibility::new(),
    };
    Rc::new(
        RefCell::new(
            Some(
                tree
            )
        )
    )
}
#[derive(Debug, Clone)]
struct Tree {
    height: usize,
    up: TreeNode,
    down: TreeNode,
    left: TreeNode,
    right: TreeNode,
    visibility: Visibility,
}
#[derive(Debug, Clone)]
struct Forest {
    data: Vec<Vec<TreeNode>>
}
impl Forest {
    fn create(forest: Vec<Vec<usize>>) -> Forest {
        let xsize = forest[0].len();
        let ysize = forest.len();
        let mut data = Vec::with_capacity(ysize);
        for i in 0..ysize {
            data.push(vec![])
        }
        for (y, row) in forest.iter().enumerate() {
            for x in row {
                data[y].push(
                    create_unlinked_node(*x)
                )
            }
        }
        let mut ret = Forest {
            data
        };
        ret.link();
        let mut node = &ret.data[0][0];
        let mut tmp = node;
        while node.borrow_mut().is_some() {
            dbg!(&node.borrow_mut().as_ref().unwrap().height);
        }
        ret
    }

    fn link(&mut self) {
        for row in self.data.iter_mut() {
            for window in row.windows(3) {
                let first = &window[0];
                let middle = &window[1];
                let last = &window[2];

                let mut outer = first.borrow_mut();
                let mut inner = outer.take().unwrap();
                inner.left = middle.clone();
                outer.insert(inner);
                
                let mut outer = middle.borrow_mut();
                let mut inner = outer.take().unwrap();
                inner.right = first.clone();
                inner.left = last.clone();
                outer.insert(inner);

                let mut outer = last.borrow_mut();
                let mut inner = outer.take().unwrap();
                inner.left = middle.clone();
                outer.insert(inner);

            }
        }
    }
}
pub fn test8b() {
    let path = PathBuf::from_str("./src/data/8.txt").unwrap();
    let lines = freadlines(path);
    let data = lines.iter()
        .map(|s| {
            s
            .chars()
            .map(|c| 
                usize::from_str(&c.to_string())
                .unwrap()
            ).collect()
        }).collect();
    let forest = Forest::create(data);
}