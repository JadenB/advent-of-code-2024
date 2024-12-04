use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const PATTERNS: [[(i32, i32); 4]; 4] = [
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(0, 3), (1, 2), (2, 1), (3, 0)],
];
const PATTERN_LETTERS: [u8; 4] = [b'X', b'M', b'A', b'S'];

const PATTERNS_2: [[(i32, i32); 5]; 2] = [
    [(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)],
    [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
];
const PATTERN_LETTERS_2: [u8; 5] = [b'M', b'M', b'A', b'S', b'S'];

struct Grid {
    data: Vec<Vec<u8>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn from_buf(reader: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let data = reader
            .lines()
            .map(|line| line.map(|l| Vec::from(l.as_bytes())))
            .collect::<Result<Vec<Vec<u8>>, _>>()?;
        let width = data
            .iter()
            .map(|row| row.len() as i32)
            .max()
            .ok_or("No rows found")?;
        let height = data.len() as i32;

        Ok(Self {
            data,
            width,
            height,
        })
    }

    fn at(&self, x: i32, y: i32) -> Option<u8> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        self.data.get(y).and_then(|row| row.get(x)).cloned()
    }
}

fn count_matches(grid: &Grid, x: i32, y: i32) -> usize {
    PATTERNS
        .iter()
        .filter(|pattern| {
            PATTERN_LETTERS
                .iter()
                .zip(pattern.iter())
                .all(|(letter, point)| {
                    grid.at(x + point.0, y + point.1)
                        .is_some_and(|x| x == *letter)
                })
                || PATTERN_LETTERS
                    .iter()
                    .rev()
                    .zip(pattern.iter())
                    .all(|(letter, point)| {
                        grid.at(x + point.0, y + point.1)
                            .is_some_and(|x| x == *letter)
                    })
        })
        .count()
}

fn search(grid: &Grid) -> usize {
    (-3..=grid.width)
        .map(|x| {
            (-3..=grid.height)
                .map(|y| count_matches(grid, x, y))
                .sum::<usize>()
        })
        .sum()
}

fn count_matches_2(grid: &Grid, x: i32, y: i32) -> usize {
    PATTERNS_2
        .iter()
        .filter(|pattern| {
            PATTERN_LETTERS_2
                .iter()
                .zip(pattern.iter())
                .all(|(letter, point)| {
                    grid.at(x + point.0, y + point.1)
                        .is_some_and(|x| x == *letter)
                })
                || PATTERN_LETTERS_2
                    .iter()
                    .rev()
                    .zip(pattern.iter())
                    .all(|(letter, point)| {
                        grid.at(x + point.0, y + point.1)
                            .is_some_and(|x| x == *letter)
                    })
        })
        .count()
}

fn search_2(grid: &Grid) -> usize {
    (-4..=grid.width)
        .map(|x| {
            (-4..=grid.height)
                .map(|y| count_matches_2(grid, x, y))
                .sum::<usize>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/input.txt")?);
    let grid = Grid::from_buf(reader)?;

    let result = search(&grid);
    println!("{result}");

    let result = search_2(&grid);
    println!("{result}");

    Ok(())
}
