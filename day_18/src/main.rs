use std::{cmp::Reverse, collections::HashMap, fs};

use priority_queue::PriorityQueue;

#[derive(Debug)]
struct NodeInfo {
    distance: usize,
    visited: bool,
}

#[derive(Clone)]
struct Grid {
    width: i32,
    height: i32,
    bytes: HashMap<(i32, i32), usize>,
}

impl Grid {
    fn from_string(s: &str, width: i32, height: i32) -> Self {
        let bytes: HashMap<(i32, i32), usize> = s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let (x, y) = line.split_once(',').unwrap();
                let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
                ((x, y), i + 1)
            })
            .collect();

        Grid {
            bytes,
            width,
            height,
        }
    }

    fn at(&self, x: i32, y: i32, time: usize) -> u8 {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            b'#'
        } else {
            let byte_time = self.bytes.get(&(x, y)).unwrap_or(&usize::MAX);
            if *byte_time <= time {
                b'#'
            } else {
                b'.'
            }
        }
    }

    fn neighbors(&self, node: (i32, i32), time: usize) -> impl Iterator<Item = (i32, i32)> + '_ {
        [
            (node.0 - 1, node.1),
            (node.0 + 1, node.1),
            (node.0, node.1 - 1),
            (node.0, node.1 + 1),
        ]
        .into_iter()
        .filter(move |n| self.at(n.0, n.1, time) == b'.')
    }

    fn solve(&self, time: usize) -> Option<usize> {
        let start = (0, 0);

        let mut nodes: HashMap<(i32, i32), NodeInfo> = HashMap::new();
        nodes.insert(
            start,
            NodeInfo {
                distance: 0,
                visited: false,
            },
        );
        let mut q = PriorityQueue::new();
        q.push(start, Reverse(0));

        while let Some((cur, _)) = q.pop() {
            let cur_info = nodes.get_mut(&cur).unwrap();
            if cur_info.visited {
                continue;
            }
            cur_info.visited = true;
            let distance = cur_info.distance;

            for neighbor in self.neighbors(cur, time) {
                let tentative_distance = distance + 1;
                let neighbor_entry = nodes.entry(neighbor).or_insert(NodeInfo {
                    distance: usize::MAX,
                    visited: false,
                });

                if tentative_distance < neighbor_entry.distance {
                    neighbor_entry.distance = tentative_distance;
                }

                q.push(neighbor, Reverse(neighbor_entry.distance));
            }
        }

        nodes
            .get(&(self.width - 1, self.height - 1))
            .map(|n| n.distance)
    }
}

fn main() {
    //let input = fs::read_to_string("input/input_ex.txt").unwrap();
    //let grid = Grid::from_string(&input, 7, 7);

    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = Grid::from_string(&input, 71, 71);

    let result = grid.solve(1024).unwrap();
    println!("{result}");

    for i in 1025..grid.bytes.len() {
        if grid.solve(i).is_none() {
            let byte_location = grid
                .bytes
                .iter()
                .filter_map(|(k, v)| if *v == i { Some(*k) } else { None })
                .next()
                .unwrap();
            println!("{},{}", byte_location.0, byte_location.1);
            break;
        }
    }
}
