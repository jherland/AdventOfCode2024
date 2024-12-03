use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut memory = String::new();
    io::stdin().read_to_string(&mut memory).unwrap();

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(d)(o)n't\(\)|(d)(o)\(\)").unwrap();

    let mut part1: Vec<u64> = vec![];
    for (all, [a, b]) in re.captures_iter(&memory).map(|c| c.extract()) {
        if all.starts_with("mul") {
            part1.push(
                a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
            );
        }
    }
    println!("Part 1: {}", part1.iter().sum::<u64>());

    let mut part2: Vec<u64> = vec![];
    let mut on = true;
    for (all, [a, b]) in re.captures_iter(&memory).map(|c| c.extract()) {
        if all == "don't()" {
            on = false;
        }
        else if all == "do()" {
            on = true;
        }
        else if all.starts_with("mul") && on {
            part2.push(
                a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
            );
        }
    }
    println!("Part 2: {}", part2.iter().sum::<u64>());
}
