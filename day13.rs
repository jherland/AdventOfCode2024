use std::io;
use std::io::Read;

use regex::Regex;

fn float_solve(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> (f64, f64) {
    // 2 eqs with 2 unknowns (a, b):
    //   Eq1: a * ax + b * bx == px
    //   Eq2: a * ay + b * by == py
    // Re-arrange Eq2 in terms of a:
    //   a = (py - b * by) / ay
    // Insert for a in Eq1:
    //   (py - b * by) / ay * ax + b * bx == px
    // Re-arrange in terms of b:
    //   ax * py - b * ax * by + b * bx * ay == px * ay
    //   b * (bx * ay - ax * by) == px * ay - py * ax
    //   b = (px * ay - py * ax) / (bx * ay - ax * by)
    let b = (px * ay - py * ax) / (bx * ay - ax * by);
    let a = (py - b * by) / ay;
    (a, b)
}

fn solve(ax: u64, ay: u64, bx: u64, by: u64, px: u64, py: u64, a_cost: u64, b_cost: u64) -> Option<u64> {
    let (fa, fb) = float_solve(ax as f64, ay as f64, bx as f64, by as f64, px as f64, py as f64);
    let (a, b) = (fa as u64, fb as u64);
    if a * ax + b * bx != px || a * ay + b * by != py { // check solution
        return None;
    }
    Some(a * a_cost + b * b_cost)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut parsed: Vec<[u64; 6]> = vec![];
    for (_, parts) in re.captures_iter(&input).map(|c| c.extract::<6>()) {
        parsed.push(
            parts
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .try_into()
                .unwrap());
    }
    let part1 = parsed
        .to_vec()
        .into_iter()
        .map(|[ax, ay, bx, by, px, py]| solve(ax, ay, bx, by, px, py, 3, 1))
        .flatten()
        .sum::<u64>();
    println!("Part 1: {}", part1);

    let p_add: u64 = 10000000000000;
    let part2 = parsed
        .to_vec()
        .into_iter()
        .map(|[ax, ay, bx, by, px, py]| solve(ax, ay, bx, by, p_add + px, p_add + py, 3, 1))
        .flatten()
        .sum::<u64>();
    println!("Part 2: {}", part2);
}
