use std::fs;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

fn parse_machine(s: &str) -> Machine {
    let mut lines = s.lines();

    let mut a = lines.next().unwrap().split_whitespace();
    let ax = a
        .nth(2)
        .unwrap()
        .strip_prefix("X+")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse()
        .unwrap();
    let ay = a
        .next()
        .unwrap()
        .strip_prefix("Y+")
        .unwrap()
        .parse()
        .unwrap();

    let mut b = lines.next().unwrap().split_whitespace();
    let bx = b
        .nth(2)
        .unwrap()
        .strip_prefix("X+")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse()
        .unwrap();
    let by = b
        .next()
        .unwrap()
        .strip_prefix("Y+")
        .unwrap()
        .parse()
        .unwrap();

    let mut p = lines.next().unwrap().split_whitespace();
    let px = p
        .nth(1)
        .unwrap()
        .strip_prefix("X=")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse()
        .unwrap();
    let py = p
        .next()
        .unwrap()
        .strip_prefix("Y=")
        .unwrap()
        .parse()
        .unwrap();

    Machine {
        a: (ax, ay),
        b: (bx, by),
        p: (px, py),
    }
}

fn parse_input(s: &str) -> Vec<Machine> {
    s.split("\n\n").map(parse_machine).collect()
}

fn min_tokens_to_win(m: &Machine, offset: i64) -> Option<i64> {
    let (ax, ay) = m.a;
    let (bx, by) = m.b;
    let (px, py) = (m.p.0 + offset, m.p.1 + offset);

    let b_presses = (ay * px - ax * py) / (ay * bx - ax * by);
    let a_presses = (px - b_presses * bx) / ax;
    if (
        a_presses * ax + b_presses * bx,
        a_presses * ay + b_presses * by,
    ) == (px, py)
    {
        Some(3 * a_presses + b_presses)
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let machines = parse_input(&input);

    let result: i64 = machines.iter().filter_map(|m| min_tokens_to_win(m, 0)).sum();
    println!("{result}");

    let result: i64 = machines
        .iter()
        .filter_map(|m| min_tokens_to_win(m, 10000000000000))
        .sum();
    println!("{result}");
}
