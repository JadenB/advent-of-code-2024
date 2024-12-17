use core::panic;
use std::fs;

fn num_from_reg_string(s: &str) -> i64 {
    s.split_whitespace().last().unwrap().parse().unwrap()
}

#[derive(Clone)]
struct Computer {
    ip: i64,
    a: i64,
    b: i64,
    c: i64,
}

impl Computer {
    fn do_instr(&mut self, instr: i64, op: i64) -> Option<i64> {
        let combo_op = match op {
            0..=3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        };

        match instr {
            0 => {
                self.a /= 2_i64.pow(combo_op.try_into().unwrap());
            } // adv: division
            1 => {
                self.b ^= op;
            } // bxl: xor
            2 => {
                self.b = combo_op % 8;
            } // bst: modulo 8
            3 => {
                if self.a != 0 {
                    self.ip = op - 2;
                }
            } // jnz: jump not zero
            4 => {
                self.b ^= self.c;
            } // bxc: xor
            5 => {
                return Some(combo_op % 8);
            } // out: ouput
            6 => {
                self.b = self.a / 2_i64.pow(combo_op.try_into().unwrap_or(0));
            } // bdv: adv with B reg
            7 => {
                self.c = self.a / 2_i64.pow(combo_op.try_into().unwrap_or(0));
            } // cdv: adv with C reg
            _ => panic!("unknown instruction {instr}"),
        }

        None
    }

    fn into_run_iter(self, program: &[i64]) -> ComputerIterator {
        ComputerIterator {
            program,
            computer: self,
        }
    }
}

struct ComputerIterator<'a> {
    program: &'a [i64],
    computer: Computer,
}

impl<'a> Iterator for ComputerIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            if self.computer.ip + 1 < self.program.len() as i64 {
                let out = self.computer.do_instr(
                    self.program[self.computer.ip as usize],
                    self.program[(self.computer.ip + 1) as usize],
                );
                self.computer.ip += 2;
                if out.is_some() {
                    break out;
                }
            } else {
                break None;
            }
        }
    }
}

fn find_a_that_outputs_program(comp: &Computer, program: &[i64]) -> usize {
    let mut output = Vec::new();
    let mut a = 0;
    loop {
        let mut c = comp.clone();
        c.a = a as i64;

        output.clear();
        output.extend(c.into_run_iter(program));
        if program.ends_with(&output) {
            if program.len() == output.len() {
                return a;
            }
            a *= 8;
        } else {
            a += 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/input_test.txt").unwrap();

    let mut lines = input.lines();
    let a: i64 = num_from_reg_string(lines.next().unwrap());
    let b: i64 = num_from_reg_string(lines.next().unwrap());
    let c: i64 = num_from_reg_string(lines.next().unwrap());

    let _ = lines.next(); // blank
    let program: Vec<i64> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let comp = Computer { ip: 0, a, b, c };

    // Part 1
    let output = comp
        .clone()
        .into_run_iter(&program)
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{output}");

    // Part 2
    let result = find_a_that_outputs_program(&comp, &program);
    println!("{result}");
}
