use std::io;
use std::io::Read;

use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"\
Button A: X\+(\d+), Y\+(\d+)
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
    dbg!(&parsed);
}
