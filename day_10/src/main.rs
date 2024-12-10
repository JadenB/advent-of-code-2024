use std::{collections::HashSet, fs};

struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let data: Vec<Vec<u8>> = s
            .lines()
            .map(|line| line.as_bytes().iter().map(|c| c - b'0').collect())
            .collect();
        Self { data }
    }

    fn at(&self, x: &i32, y: &i32) -> Option<u8> {
        self.data
            .get(*y as usize)
            .and_then(|row| row.get(*x as usize))
            .copied()
    }

    fn compute_reachable(&self, x: &i32, y: &i32, next: u8, result: &mut HashSet<(i32, i32)>) {
        let Some(v) = self.at(x, y) else {
            return;
        };

        if v != next {
            return;
        }

        if v == 9 {
            result.insert((*x, *y));
        } else {
            for p in [((x - 1), *y), ((x + 1), *y), (*x, (y - 1)), (*x, (y + 1))].iter() {
                self.compute_reachable(&p.0, &p.1, v + 1, result);
            }
        }
    }

    fn score(&self, x: &i32, y: &i32) -> i32 {
        if self.at(x, y).is_some_and(|v| v != 0) {
            return 0;
        }

        let mut reachable: HashSet<(i32, i32)> = HashSet::new();
        self.compute_reachable(x, y, 0, &mut reachable);

        reachable.len() as i32
    }

    fn rating(&self, x: &i32, y: &i32, next: u8) -> i32 {
        let Some(v) = self.at(x, y) else {
            return 0;
        };

        if v != next {
            0
        } else if v == 9 {
            1
        } else {
            [((x - 1), *y), ((x + 1), *y), (*x, (y - 1)), (*x, (y + 1))]
                .iter()
                .map(|p| self.rating(&p.0, &p.1, v + 1))
                .sum()
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = Grid::from_str(&input);

    // Part 1
    let result: i32 = (0..grid.data.len())
        .map(|y| {
            (0..grid.data.first().unwrap().len())
                .map(|x| grid.score(&(x as i32), &(y as i32)))
                .sum::<i32>()
        })
        .sum();
    println!("{result}");

    // Part 2
    let result: i32 = (0..grid.data.len())
        .map(|y| {
            (0..grid.data.first().unwrap().len())
                .map(|x| grid.rating(&(x as i32), &(y as i32), 0))
                .sum::<i32>()
        })
        .sum();
    println!("{result}");
}
