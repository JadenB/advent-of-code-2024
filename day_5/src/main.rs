use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct OrderingRules {
    after_to_before: HashMap<i32, HashSet<i32>>,
}

impl OrderingRules {
    fn from_string(string: &str) -> Self {
        let rules_iter = string.lines().map(|line| {
            let mut split_iter = line.split('|');
            (
                split_iter.next().unwrap().parse::<i32>().unwrap(),
                split_iter.next().unwrap().parse::<i32>().unwrap(),
            )
        });

        let mut after_to_before: HashMap<i32, HashSet<i32>> = HashMap::new();
        for rule in rules_iter {
            after_to_before.entry(rule.1).or_default().insert(rule.0);
        }

        Self {
            after_to_before,
        }
    }
}

fn update_is_correct(update: &[i32], rules: &OrderingRules) -> bool {
    update.iter().enumerate().all(|(i, p0)| {
        update[i..update.len()].iter().all(|p1| {
            !rules
                .after_to_before
                .get(p0)
                .is_some_and(|after_p0| after_p0.contains(p1))
        })
    })
}

fn sorted_update(update: &[i32], rules: &OrderingRules) -> Vec<i32> {
    let mut result = update.to_vec();
    sort_update(&mut result, rules);

    result
}

fn sort_update(update: &mut [i32], rules: &OrderingRules) {
    if update.len() <= 1 {
        return;
    }
    let pivot = *update.last().unwrap();

    let mut final_pivot_index = 0;
    for i in 0..update.len() {
        let page = update[i];
        if rules.after_to_before.get(&pivot).is_some_and(|a| a.contains(&page)) {
            update.swap(final_pivot_index, i);
            final_pivot_index += 1;
        }
    }

    update.swap(final_pivot_index, update.len() - 1);

    let left_range = 0..final_pivot_index;
    let right_range = (final_pivot_index+1)..(update.len());
    sort_update(&mut update[left_range], rules);
    sort_update(&mut update[right_range], rules);
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let (rules_string, update_string) = input.split_once("\n\n").unwrap();

    let rules = OrderingRules::from_string(rules_string);
    let updates = update_string
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Part 1
    let result = updates
        .iter()
        .filter(|u| update_is_correct(u, &rules))
        .map(|u| u[u.len() / 2])
        .sum::<i32>();
    println!("{result}");

    // Part 2
    let result = updates
        .iter()
        .filter(|u| !update_is_correct(u, &rules))
        .map(|u| sorted_update(u, &rules))
        .inspect(|u| assert!(update_is_correct(u, &rules)))
        .map(|u| u[u.len() / 2])
        .sum::<i32>();
    println!("{result}");
}
