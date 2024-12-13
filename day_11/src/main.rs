use std::{collections::HashMap, fs};

fn stone_count(s: u64, blinks: i32, m: &mut HashMap<(u64, i32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    } else if let Some(cached) = m.get(&(s, blinks)) {
        return *cached;
    }

    let result = if s == 0 {
        stone_count(1, blinks - 1, m)
    } else if (s.ilog10() + 1) % 2 == 0 {
        let s_str = s.to_string();
        let (left, right) = s_str.split_at(s_str.len() / 2);
        stone_count(left.parse().unwrap(), blinks - 1, m)
            + stone_count(right.parse().unwrap(), blinks - 1, m)
    } else {
        stone_count(
            s * 2024,
            blinks - 1,
            m,
        )
    };

    m.insert((s, blinks), result);
    result
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let stones: Vec<String> = input.split_whitespace().map(String::from).collect();

    let mut m = HashMap::new();
    let result: u64 = stones.iter().map(|s| stone_count(s.parse().unwrap(), 25, &mut m)).sum();
    println!("{result}");

    let result: u64 = stones.iter().map(|s| stone_count(s.parse().unwrap(), 75, &mut m)).sum();
    println!("{result}");
}
