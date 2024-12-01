use std::io;

use itertools::Itertools;

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let (left, right): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();
    let left_nums: Vec<usize> = left
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .sorted()
        .collect();
    let right_nums: Vec<usize> = right
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .sorted()
        .collect();
    assert_eq!(left_nums.len(), right_nums.len());

    let diff: usize = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("Part 1: {}", diff);

    let similarity: usize = left_nums
        .iter()
        .map(|num| num * right_nums.iter().filter(|x| *x == num).count())
        .sum();
    println!("Part 2: {}", similarity);
}
