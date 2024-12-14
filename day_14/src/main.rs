use std::{
    borrow::Borrow,
    fs,
    io::{self, Write},
};

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn from_line(s: &str) -> Self {
        let (ps, vs) = s.split_once(' ').unwrap();
        let (x, y) = ps.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let (vx, vy) = vs.strip_prefix("v=").unwrap().split_once(',').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            vx: vx.parse().unwrap(),
            vy: vy.parse().unwrap(),
        }
    }
}

fn wrapped(v: i32, len: i32) -> i32 {
    if v >= 0 {
        v % len
    } else {
        len + ((v + 1) % len) - 1
    }
}

fn final_pos(r: &Robot, seconds: i32, w: i32, h: i32) -> (i32, i32) {
    (
        wrapped(r.x + seconds * r.vx, w),
        wrapped(r.y + seconds * r.vy, h),
    )
}

fn safety_factor<T, I>(robot_positions: I, w: i32, h: i32) -> i32
where
    T: Borrow<(i32, i32)>,
    I: IntoIterator<Item = T>,
{
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mid_x = w / 2;
    let mid_y = h / 2;

    for pos in robot_positions {
        let pos = pos.borrow();
        if pos.0 < mid_x && pos.1 < mid_y {
            q1 += 1;
        } else if pos.0 > mid_x && pos.1 < mid_y {
            q2 += 1;
        } else if pos.0 < mid_x && pos.1 > mid_y {
            q3 += 1;
        } else if pos.0 > mid_x && pos.1 > mid_y {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

fn print_grid(w: i32, h: i32, robots: &[Robot]) {
    let mut s = 0;
    loop {
        let safety = safety_factor(robots.iter().map(|r| final_pos(r, s, w, h)), w, h);
        if safety < 50000000 {
            let mut grid: Vec<Vec<u8>> = (0..h).map(|_| (0..w).map(|_| b'.').collect()).collect();
            for r in robots {
                let (x, y) = final_pos(r, s, w, h);
                grid[y as usize][x as usize] = b'O';
            }

            for row in grid.iter_mut() {
                let _ = io::stdout().write(row);
                let _ = io::stdout().write(b"\n");
            }

            return;
        }

        s += 1;
    }
}

fn main() {
    // let (input, w, h) = (fs::read_to_string("input/input_ex.txt").unwrap(), 11, 7);
    let (input, w, h) = (fs::read_to_string("input/input.txt").unwrap(), 101, 103);
    let robots: Vec<Robot> = input.lines().map(Robot::from_line).collect();

    let result = safety_factor(robots.iter().map(|r| final_pos(r, 100, w, h)), w, h);
    println!("{result}");

    print_grid(w, h, &robots);
}
