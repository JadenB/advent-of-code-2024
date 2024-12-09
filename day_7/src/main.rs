use std::fs;

enum Op {
    Add,
    Mult,
    Concat,
}

fn apply_op(op: &Op, a: &i64, b: &i64) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Mult => a * b,
        Op::Concat => (a.to_string() + &b.to_string()).parse().unwrap(),
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

fn has_solution(eq: &Equation, ops: &[Op]) -> bool {
    let Some(first) = eq.nums.first() else {
        return true;
    };
    let mut stack: Vec<i64> = vec![*first];

    for num in eq.nums.iter().skip(1) {
        stack = stack
            .iter()
            .flat_map(|x| ops.iter().map(|op| apply_op(op, x, num)))
            .collect();
    }

    stack.contains(&eq.result)
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let eqs: Vec<Equation> = input.lines().map(Equation::from_str).collect();

    let result: i64 = eqs
        .iter()
        .filter(|e| has_solution(e, &[Op::Add, Op::Mult]))
        .map(|e| e.result)
        .sum();
    println!("{result}");

    let result: i64 = eqs
        .iter()
        .filter(|e| has_solution(e, &[Op::Add, Op::Mult, Op::Concat]))
        .map(|e| e.result)
        .sum();
    println!("{result}");
}
