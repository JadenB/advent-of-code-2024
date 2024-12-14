use std::{fs, io::{self, Write}};

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

fn final_pos(r: &Robot, seconds: i32, w: i32, h: i32) -> (i32, i32) {
            let final_x = r.x + seconds * r.vx;
            let final_y = r.y + seconds * r.vy;

            let final_x = if final_x >= 0 {
                final_x % w
            } else {
                w + ((final_x + 1) % w) - 1
            };
            let final_y = if final_y >= 0 {
                final_y % h
            } else {
                h + ((final_y + 1) % h) - 1
            };

            (final_x, final_y)
}

fn final_positions(robots: &[Robot], seconds: i32, w: i32, h: i32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|r| final_pos(r, seconds, w, h))
        .collect()
}

fn safety_factor(robot_positions: &[(i32, i32)], w: i32, h: i32) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mid_x = w / 2;
    let mid_y = h / 2;

    for pos in robot_positions {
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
    let mut grid: Vec<Vec<u8>> = (0..h).map(|_| (0..w).map(|_| b'.').collect()).collect();

    let mut s = 0;
    loop {
        print!("{esc}[1;1H", esc = 27 as char);
        println!("===== {} =====", s);
        for r in robots {
            let (x, y) = final_pos(r, s, w, h);
            grid[y as usize][x as usize] = b'O';
        }

    let final_positions = final_positions(robots, s, w, h);
    let safety = safety_factor(&final_positions, w, h);
        for row in grid.iter_mut() {
            let _ = io::stdout().write(row);
            let _ = io::stdout().write(b"\n");
            row.fill(b'.');
        }

    if safety < 50019008 {
        return;
    }

        s += 1;
    }
}

fn main() {
    //let (input, w, h) = (fs::read_to_string("input/input_ex.txt").unwrap(), 11, 7);
    let (input, w, h) = (fs::read_to_string("input/input.txt").unwrap(), 101, 103);

    let robots: Vec<Robot> = input.lines().map(Robot::from_line).collect();

    let final_positions = final_positions(&robots, 100, w, h);
    println!("{final_positions:?}");
    let result = safety_factor(&final_positions, w, h);
    println!("{result}");

    print_grid(w, h, &robots);
}
