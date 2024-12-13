use std::fs;

fn is_antenna(b: u8) -> bool {
    b.is_ascii_alphabetic() || b.is_ascii_digit()
}

struct Grid {
    data: Vec<Vec<u8>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let data: Vec<Vec<u8>> = s.lines().map(|line| line.bytes().collect()).collect();
        let height = data.len() as i32;
        let width = data.first().map(|row| row.len()).unwrap_or(0) as i32;
        Self {
            data,
            width,
            height,
        }
    }

    fn at(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 {
            None
        } else {
            self.data
                .get(y as usize)
                .and_then(|row| row.get(x as usize).copied())
        }
    }

    fn has_value_at_distance(&self, x: i32, y: i32, value: u8, distance: (i32, i32)) -> bool {
        [
            (x + distance.0, y + distance.1),
            (x - distance.0, y - distance.1),
        ]
        .into_iter()
        .any(|p| self.at(p.0, p.1).is_some_and(|v| v == value))
    }

    fn is_antinode_1(&self, x: i32, y: i32) -> bool {
        for x1 in 0..self.width {
            for y1 in 0..self.height {
                let v = self.at(x1, y1).unwrap();
                let other_dist = ((x1 - x) * 2, (y1 - y) * 2);
                if other_dist != (0, 0)
                    && is_antenna(v)
                    && self.has_value_at_distance(x, y, v, other_dist)
                {
                    return true;
                }
            }
        }

        false
    }

    fn count_antinodes_2(&self) -> usize {
        let mut antinode_grid = self.data.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                self.mark_antinodes_2(x, y, &mut antinode_grid);
            }
        }

        antinode_grid
            .iter()
            .map(|row| row.iter().copied().filter(|b| *b == b'%').count())
            .sum()
    }

    fn mark_antinodes_2(&self, x: i32, y: i32, grid: &mut [Vec<u8>]) {
        let Some(v) = self.at(x, y) else {
            return;
        };

        if !is_antenna(v) {
            return;
        }

        for x1 in 0..self.width {
            for y1 in 0..self.height {
                if (x1, y1) != (x, y) && self.at(x1, y1).is_some_and(|v1| v1 == v) {
                    self.mark_antinodes_for_pair_2((x, y), (x1, y1), grid);
                }
            }
        }
    }

    fn mark_antinodes_for_pair_2(&self, a1: (i32, i32), a2: (i32, i32), grid: &mut [Vec<u8>]) {
        let dist = (a2.0 - a1.0, a2.1 - a1.1);
        let mut mark_dist: (i32, i32) = (0, 0);
        while mark_dist.0.abs() < self.width && mark_dist.1.abs() < self.height {
            let mx = a1.0 - mark_dist.0;
            let my = a1.1 - mark_dist.1;
            if self.at(mx, my).is_some() {
                grid[my as usize][mx as usize] = b'%';
            }

            let mx = a2.0 + mark_dist.0;
            let my = a2.1 + mark_dist.1;
            if self.at(mx, my).is_some() {
                grid[my as usize][mx as usize] = b'%';
            }

            mark_dist = (mark_dist.0 + dist.0, mark_dist.1 + dist.1);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = Grid::from_str(&input);

    let result: usize = (0..grid.width)
        .map(|x| {
            (0..grid.height)
                .filter(|y| grid.is_antinode_1(x, *y))
                .count()
        })
        .sum();
    println!("{result}");

    let result: usize = grid.count_antinodes_2();
    println!("{result}");
}
