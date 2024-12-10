use std::fs;

enum Op {
    Add,
    Mult,
    Concat,
}

fn concat(a: &i64, b: &i64) -> i64 {
    let mut pow = 10;
    while *b >= pow {
        pow *= 10;
    }
    
    a * pow + b
}

fn apply_op(op: &Op, a: &i64, b: &i64) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Mult => a * b,
        Op::Concat => concat(a, b),
    }
}

#[derive(Debug)]
struct Equation {
    nums: Vec<i64>,
    result: i64,
}

impl Equation {
    fn from_str(s: &str) -> Self {
        let (result, nums) = s.split_once(':').unwrap();
        let result: i64 = result.parse().unwrap();
        let nums = nums
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self { nums, result }
    }
}

fn has_solution_1(first: &i64, rest: &[i64], solution: &i64) -> bool {
    if rest.is_empty() {
        first == solution
    } else if first > solution {
        false
    } else {
        [Op::Add, Op::Mult]
            .iter()
            .any(|op| has_solution_1(&apply_op(op, first, &rest[0]), &rest[1..], solution))
    }
}

fn has_solution_2(first: &i64, rest: &[i64], solution: &i64) -> bool {
    if rest.is_empty() {
        first == solution
    } else if first > solution {
        false
    } else {
        [Op::Add, Op::Mult, Op::Concat]
            .iter()
            .any(|op| has_solution_2(&apply_op(op, first, &rest[0]), &rest[1..], solution))
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let eqs: Vec<Equation> = input.lines().map(Equation::from_str).collect();

    let result: i64 = eqs
        .iter()
        .filter(|e| match e.nums.split_first() {
            Some((first, rest)) => has_solution_1(first, rest, &e.result),
            None => false,
        })
        .map(|e| e.result)
        .sum();
    println!("{result}");

    let result: i64 = eqs
        .iter()
        .filter(|e| match e.nums.split_first() {
            Some((first, rest)) => has_solution_2(first, rest, &e.result),
            None => false,
        })
        .map(|e| e.result)
        .sum();
    println!("{result}");
}
