use std::cmp::Ordering;
use std::io;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct Rule {
    before: usize,
    after: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (first, second) = input.trim().split_once("\n\n").unwrap();
    let rules: Vec<Rule> = first
        .split("\n")
        .map(|line| {
            let words = line.split_once("|").unwrap();
            Rule {
                before: words.0.parse().unwrap(),
                after: words.1.parse().unwrap(),
            }
        })
        .collect();

    let updates: Vec<Vec<usize>> = second
        .split("\n")
        .map(|line| line.split(",").map(|w| w.parse::<usize>().unwrap()).collect())
        .collect();

    let is_sorted_by_rules = |a: &usize, b: &usize| rules.contains(&Rule {before: *a, after: *b});

    let part1: usize = updates
        .iter()
        .filter(|update| update.is_sorted_by(is_sorted_by_rules))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 1: {}", part1);

    let compare_by_rules = |a: &usize, b: &usize| {
        if rules.contains(&Rule {before: *a, after: *b}) { Ordering::Less }
        else { Ordering::Greater }
    };

    let part2: usize = updates
        .iter()
        .filter(|update| !update.is_sorted_by(is_sorted_by_rules))
        .map(|update| {
            let mut fix_update = update.clone();
            fix_update.sort_by(compare_by_rules);
            fix_update
        })
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 2: {}", part2);
}
