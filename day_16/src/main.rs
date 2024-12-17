use core::panic;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{stdout, Write},
    mem,
};

use priority_queue::PriorityQueue;

fn rotated(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("unknown dir!"),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    x: i32,
    y: i32,
    dir: (i32, i32),
}

impl Node {
    fn new(x: i32, y: i32, dir: (i32, i32)) -> Self {
        Node { x, y, dir }
    }
}

#[derive(Clone, Copy)]
struct NodeInfo {
    distance: i32,
    visited: bool,
}

#[derive(Clone)]
pub struct Grid {
    data: Vec<u8>,
    width: i32,
    height: i32,
    ex: i32,
    ey: i32,
    sx: i32,
    sy: i32,
}

impl Grid {
    fn from_string(s: &str) -> Self {
        let width = s.lines().next().map_or(0, |l| l.as_bytes().len()) as i32;

        let mut data: Vec<u8> = Vec::new();
        let mut height = 0;
        let mut ex = 0;
        let mut ey = 0;
        let mut sx = 0;
        let mut sy = 0;
        for line in s.lines().map(|l| l.as_bytes()) {
            assert!(line.len() as i32 == width);
            for (x, b) in line.iter().enumerate() {
                match b {
                    b'E' => {
                        ex = x as i32;
                        ey = height;
                    }
                    b'S' => {
                        sx = x as i32;
                        sy = height;
                    }
                    _ => (),
                }
            }
            data.extend_from_slice(line);
            height += 1;
        }

        let mut grid = Grid {
            data,
            width,
            height,
            ex,
            ey,
            sx,
            sy,
        };
        grid.set(ex, ey, b'.');
        grid.set(sx, sy, b'.');

        grid
    }

    fn at(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            b'#'
        } else {
            self.data
                .get((y * self.width + x) as usize)
                .copied()
                .unwrap_or(b'#')
        }
    }

    fn set(&mut self, x: i32, y: i32, v: u8) -> u8 {
        if x < 0 || y < 0 {
            b'#'
        } else {
            self.data
                .get_mut((y * self.width + x) as usize)
                .map(|inner| mem::replace(inner, v))
                .unwrap_or(b'#')
        }
    }

    fn print(&self) {
        let mut line_start: usize = 0;
        while line_start < (self.width * self.height) as usize {
            let _ = stdout().write(&self.data[line_start..(line_start + self.width as usize)]);
            let _ = stdout().write(b"\n");
            line_start += self.width as usize;
        }
    }

    fn neighbors<'a>(&'a self, node: &Node) -> impl Iterator<Item = (Node, i32)> + 'a {
        [
            (
                Node::new(node.x + node.dir.0, node.y + node.dir.1, node.dir),
                1,
            ),
            (Node::new(node.x, node.y, rotated(node.dir)), 1000),
            (
                Node::new(node.x, node.y, rotated(rotated(rotated(node.dir)))),
                1000,
            ),
        ]
        .into_iter()
        .filter(|(n, _)| self.at(n.x, n.y) == b'.')
    }

    fn neighbors_backwards<'a>(&'a self, node: &Node) -> impl Iterator<Item = (Node, i32)> + 'a {
        [
            (
                Node::new(node.x - node.dir.0, node.y - node.dir.1, node.dir),
                1,
            ),
            (Node::new(node.x, node.y, rotated(node.dir)), 1000),
            (
                Node::new(node.x, node.y, rotated(rotated(rotated(node.dir)))),
                1000,
            ),
        ]
        .into_iter()
        .filter(|(n, _)| self.at(n.x, n.y) == b'.')
    }

    fn make_distances_map(&self) -> HashMap<Node, NodeInfo> {
        let start = Node::new(self.sx, self.sy, (-1, 0));

        let mut nodes: HashMap<Node, NodeInfo> = HashMap::new();
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
            let cur_info = *nodes.get(&cur).unwrap();
            if cur_info.visited {
                continue;
            }
            nodes.get_mut(&cur).unwrap().visited = true;

            for (neighbor, cost) in self.neighbors(&cur) {
                let tentative_distance = cur_info.distance + cost;
                let neighbor_entry = nodes.entry(neighbor).or_insert(NodeInfo {
                    distance: i32::MAX,
                    visited: false,
                });

                if tentative_distance < neighbor_entry.distance {
                    neighbor_entry.distance = tentative_distance;
                }

                q.push(neighbor, Reverse(neighbor_entry.distance));
            }
        }

        nodes
    }

    fn count_shortest_path_tiles(&self, distances: &HashMap<Node, NodeInfo>, shortest: i32) -> usize {
        let mut best_path_tiles = HashSet::new();
        let mut q: VecDeque<(Node, i32)> = VecDeque::new();

        for (end_node, node_info) in distances
            .iter()
            .filter(|(n, ni)| (n.x, n.y) == (self.ex, self.ey) && ni.distance == shortest)
        {
            q.push_back((*end_node, node_info.distance));
        }

        while let Some((cur, dist)) = q.pop_front() {
            best_path_tiles.insert((cur.x, cur.y));
            for (neighbor, cost) in self.neighbors_backwards(&cur) {
                let neighbor_dist = distances.get(&neighbor).unwrap().distance;
                if neighbor_dist + cost == dist {
                    q.push_back((neighbor, neighbor_dist))
                }
            }
        }

        best_path_tiles.len()
    }

    fn solve(&self) -> (i32, usize) {
        let distances = self.make_distances_map();

        let shortest = distances
            .iter()
            .filter_map(|(k, v)| {
                if (k.x, k.y) == (self.ex, self.ey) {
                    Some(v.distance)
                } else {
                    None
                }
            })
            .min()
            .unwrap();

        (shortest, self.count_shortest_path_tiles(&distances, shortest))
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = Grid::from_string(&input);

    let result = grid.solve();
    println!("{result:?}");
}
