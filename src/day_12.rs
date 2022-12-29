use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::freadlines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeType {
    Start,
    End,
    Elevation(u8),
}
#[derive(Debug, Clone, Copy)]
struct Node {
    elevation: NodeType,
    visited: bool,
    location: (usize, usize),
    prev: Option<(usize, usize)>,
    dist: usize,
    // prio: usize,
}
impl Node {
    fn new(c: char, x: usize, y: usize) -> Self {
        let nt = match c {
            'S' => NodeType::Start,
            'E' => NodeType::End,
            c => NodeType::Elevation((c as u8) - ('a' as u8)),
        };
        Self {
            elevation: nt,
            visited: false,
            location: (x, y),
            prev: None,
            dist: usize::MAX,
            // prio: usize::MAX,
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
impl Eq for Node {}
#[derive(Debug, Clone)]
struct Graph {
    nodes: VecDeque<VecDeque<Node>>,
    start: (usize, usize),
    end: (usize, usize),
    queue: BinaryHeap<Reverse<Node>>,
}
impl Graph {
    fn new(nodes: VecDeque<String>) -> Self {
        let mut start = None;
        let mut end = None;
        let nodes = nodes
            .into_iter()
            .enumerate()
            .map(|line| {
                line.1
                    .char_indices()
                    .map(|c| {
                        let pos = (c.0, line.0);
                        let n = Node::new(c.1, pos.0, pos.1);
                        match n.elevation {
                            NodeType::End => end = Some(pos),
                            NodeType::Start => start = Some(pos),
                            _ => {}
                        };
                        n
                    })
                    .collect::<VecDeque<Node>>()
            })
            .collect::<VecDeque<VecDeque<Node>>>();
        let start = start.unwrap();
        let end = end.unwrap();
        // let queue = nodes
        //     .clone()
        //     .into_iter()
        //     .flatten()
        //     .map(|n| Reverse(n))
        //     .collect();
        let queue = BinaryHeap::new();
        Self {
            nodes,
            start,
            end,
            queue,
        }
    }
    fn pathfind(&mut self, startx: usize, starty: usize, end_el: Option<usize>) -> Vec<Node> {
        let mut result = vec![];
        let mut first_node = &mut self.nodes[starty][startx];
        first_node.dist = 0;
        first_node.visited = true;
        self.queue.push(Reverse(*first_node));
        let mut current_node;
        while self.queue.len() > 0 {
            current_node = self.queue.pop().unwrap();
            let loc = current_node.0.location;
            // dbg!(loc);
            let current_el = match current_node.0.elevation {
                NodeType::Elevation(n) => n,
                NodeType::Start => 0,
                NodeType::End => 25, // TODO: check range
            };
            if end_el.is_none() {
                match current_node.0.elevation {
                    NodeType::End => return vec![current_node.0],
                    _ => {}
                }
            } else if current_el == end_el.unwrap() as u8 {
                result.push(current_node.0);
            }
            let mut neighbors = vec![
                (loc.0.checked_sub(1), Some(loc.1)),
                (Some(loc.0 + 1), Some(loc.1)),
                (Some(loc.0), loc.1.checked_sub(1)),
                (Some(loc.0), Some(loc.1 + 1)),
            ];
            neighbors = neighbors
                .into_iter()
                .filter(|n| bounds_check(*n, self.nodes[0].len(), self.nodes.len()).is_some())
                .collect();
            for neighbor in neighbors.iter() {
                let a = current_node.0.dist + 1;
                let neighbor = (neighbor.0.unwrap(), neighbor.1.unwrap());
                let node = &mut self.nodes[neighbor.1][neighbor.0];
                let neighbor_el = match node.elevation {
                    NodeType::Elevation(n) => n,
                    NodeType::Start => 0,
                    NodeType::End => 25,
                };
                if (neighbor_el as i32 - current_el as i32 <= 1) && end_el.is_none()
                    || (current_el as i32 - neighbor_el as i32 <= 1) && end_el.is_some()
                {
                    if !node.visited {
                        node.dist = a;
                        node.prev = Some(current_node.0.location);
                        node.visited = true;
                        self.queue.push(Reverse(*node));
                    }
                }
            }
        }
        result
    }
}
fn bounds_check(loc: (Option<usize>, Option<usize>), x: usize, y: usize) -> Option<(usize, usize)> {
    let mut px;
    let mut py;
    if loc.0.is_some() {
        px = loc.0.unwrap();
    } else {
        return None;
    }
    if loc.1.is_some() {
        py = loc.1.unwrap();
    } else {
        return None;
    }
    if px >= x {
        return None;
    } else if py >= y {
        return None;
    }
    Some((px, py))
}
pub fn test12a() {
    let path = PathBuf::from_str("./src/data/12.txt").unwrap();
    let lines = freadlines(path).into_iter().collect();
    let mut graph = Graph::new(lines);
    let node = graph.pathfind(graph.start.0, graph.start.1, None);
    println!("12a: {}", node[0].dist);
}
pub fn test12b() {
    let path = PathBuf::from_str("./src/data/12.txt").unwrap();
    let lines = freadlines(path).into_iter().collect();
    let mut graph = Graph::new(lines);

    let nodes = graph.pathfind(graph.end.0, graph.end.1, Some(0));
    let min = nodes.iter().map(|n| n.dist).min().unwrap();

    println!("12b: {}", min);
}
