use std::{fs, usize};

#[derive(Debug)]
struct Region {
    area: i32,
    perimeter: i32,
    vertices: i32,
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

    fn equals(&self, x: i32, y: i32, v: u8) -> bool {
        self.at(x, y).is_some_and(|vv| vv == v)
    }

    fn get_regions(&self) -> Vec<Region> {
        let mut visited: Vec<Vec<u8>> = self.data.clone();
        let mut result = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(region) = self.get_region_at(x, y, &mut visited) {
                    result.push(region);
                }
            }
        }
        result
    }

    fn get_region_at(&self, x: i32, y: i32, visited: &mut Vec<Vec<u8>>) -> Option<Region> {
        let val = self.at(x, y)?;
        let result = self.search_region_at(x, y, val, visited);
        if result.as_ref().is_some_and(|r| r.area > 0) {
            result
        } else {
            None
        }
    }

    fn search_region_at(&self, x: i32, y: i32, value: u8, visited: &mut Vec<Vec<u8>>) -> Option<Region> {
        let v = self.at(x, y)?;
        if v != value {
            return None;
        }

        if visited[y as usize][x as usize] == b'@' {
            return Some(Region { perimeter: 0, area: 0, vertices: 0 });
        }
        visited[y as usize][x as usize] = b'@';

        let mut result = Region { area: 1, perimeter: 0, vertices: 0 };
        for next_point in [
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
        ].iter() {
            let region = self.search_region_at(next_point.0, next_point.1, value, visited);
            match region {
                Some(r) => {
                    result.area += r.area;
                    result.perimeter += r.perimeter;
                    result.vertices += r.vertices;
                },
                None => {
                    result.perimeter += 1;
                }
            }
        }

        result.vertices += self.vertices_at(x, y);

        Some(result)
    }

    fn vertices_at(&self, x: i32, y: i32) -> i32 {
        let Some(v) = self.at(x, y) else {
            return 0;
        };

        let mut result = 0;

        // Outer corners
        for (c1, c2) in [
            ((x - 1, y), (x, y - 1)),
            ((x + 1, y), (x, y - 1)),
            ((x - 1, y), (x, y + 1)),
            ((x + 1, y), (x, y + 1)),
        ] {
            if !self.equals(c1.0, c1.1, v) && !self.equals(c2.0, c2.1, v) {
                result += 1;
            }
        }

        for (c1, c2, c3) in [
            ((x - 1, y), (x, y - 1), (x - 1, y - 1)),
            ((x + 1, y), (x, y - 1), (x + 1, y - 1)),
            ((x - 1, y), (x, y + 1), (x - 1, y + 1)),
            ((x + 1, y), (x, y + 1), (x + 1, y + 1)),
        ] {
            if self.equals(c1.0, c1.1, v) && self.equals(c2.0, c2.1, v) && !self.equals(c3.0, c3.1, v) {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = Grid::from_str(&input);

    let regions = grid.get_regions();
    let result: i32 = regions.iter().map(|r| r.area * r.perimeter).sum();
    println!("{result}");

    let result: i32 = regions.iter().map(|r| r.area * r.vertices).sum();
    println!("{result}");
}
