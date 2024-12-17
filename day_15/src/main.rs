use std::{
    fs,
    io::{stdout, Write},
    mem,
};

#[derive(Clone)]
pub struct Grid {
    data: Vec<u8>,
    width: i32,
    height: i32,
    rx: i32,
    ry: i32,
}

impl Grid {
    fn from_string(s: &str) -> Self {
        let width = s.lines().next().map_or(0, |l| l.as_bytes().len()) as i32;

        let mut data: Vec<u8> = Vec::new();
        let mut height = 0;
        let mut rx = 0;
        let mut ry = 0;
        for line in s.lines().map(|l| l.as_bytes()) {
            assert!(line.len() as i32 == width);
            if let Some(x) = line.iter().position(|b| *b == b'@') {
                rx = x as i32;
                ry = height;
            }
            data.extend_from_slice(line);
            height += 1;
        }

        Self {
            data,
            width,
            height,
            rx,
            ry,
        }
    }

    fn to_wide(&self) -> Self {
        let mut data: Vec<u8> = Vec::new();
        for b in self.data.iter() {
            let new_b: [u8; 2] = match b {
                b'#' => [b'#', b'#'],
                b'O' => [b'[', b']'],
                b'.' => [b'.', b'.'],
                b'@' => [b'@', b'.'],
                _ => panic!("unknown entity {b}"),
            };
            data.extend(new_b);
        }

        Self {
            data,
            width: self.width * 2,
            height: self.height,
            rx: self.rx * 2,
            ry: self.ry,
        }
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

    fn move_robot(&mut self, m: u8) {
        let (dx, dy) = match m {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => panic!("unknown move {m}!"),
        };

        if self.can_move(self.rx, self.ry, dx, dy) {
            self.do_move(self.rx, self.ry, dx, dy);
            self.set(self.rx, self.ry, b'.');
            self.rx += dx;
            self.ry += dy;
        }
    }

    fn shift(&mut self, x: i32, y: i32, dx: i32, dy: i32) {
        assert!(self.at(x + dx, y + dy) == b'.');

        let this = self.at(x, y);
        self.set(x, y, b'.');
        self.set(x + dx, y + dy, this);
    }

    fn do_move(&mut self, x: i32, y: i32, dx: i32, dy: i32) {
        let this = self.at(x, y);
        let (next_x, next_y) = (x + dx, y + dy);
        match this {
            b'#' => (),
            b'.' => (),
            b'@' | b'O' => {
                self.do_move(next_x, next_y, dx, dy);
                self.shift(x, y, dx, dy);
            }
            b'[' => {
                self.do_move(next_x + 1, next_y, dx, dy);
                self.shift(x + 1, y, dx, dy);

                if dx != 1 {
                    self.do_move(next_x, next_y, dx, dy);
                }
                self.shift(x, y, dx, dy);
            }
            b']' => {
                self.do_move(next_x - 1, next_y, dx, dy);
                self.shift(x - 1, y, dx, dy);

                if dx != -1 {
                    self.do_move(next_x, next_y, dx, dy);
                }
                self.shift(x, y, dx, dy);
            }
            _ => panic!("unknown entity!"),
        };
    }

    fn can_move(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let (next_x, next_y) = (x + dx, y + dy);
        match self.at(next_x, next_y) {
            b'#' => false,
            b'.' => true,
            b'O' => self.can_move(next_x, next_y, dx, dy),
            b'[' => {
                if dx == 1 {
                    self.can_move(next_x + 1, next_y, dx, dy)
                } else {
                    self.can_move(next_x, next_y, dx, dy)
                        && self.can_move(next_x + 1, next_y, dx, dy)
                }
            }
            b']' => {
                if dx == -1 {
                    self.can_move(next_x - 1, next_y, dx, dy)
                } else {
                    self.can_move(next_x, next_y, dx, dy)
                        && self.can_move(next_x - 1, next_y, dx, dy)
                }
            }
            _ => panic!("unknown entity!"),
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

    fn gps_sum(&self) -> i32 {
        let mut result = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                result += match self.at(x, y) {
                    b'O' | b'[' => 100 * y + x,
                    _ => 0,
                };
            }
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut map = Grid::from_string(map_str);
    let mut map_wide = map.clone().to_wide();

    for line in moves_str.lines() {
        for m in line.trim().as_bytes() {
            map.move_robot(*m);
        }
    }
    let result = map.gps_sum();
    println!("{result}");

    for line in moves_str.lines() {
        for m in line.trim().as_bytes() {
            map_wide.move_robot(*m);
        }
    }
    let result = map_wide.gps_sum();
    println!("{result}");
}
