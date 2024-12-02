use std::io;

fn is_safe(nums: &Vec<i32>) -> bool {
    fn inner(nums: &Vec<i32>) -> bool {
        nums.windows(2).all(|win| 1 <= (win[1] - win[0]) && (win[1] - win[0]) <= 3)
    }

    let rev: Vec<i32> = nums.iter().rev().cloned().collect();
    inner(nums) || inner(&rev)
}

fn is_safe_with_dampener(nums: &Vec<i32>) -> bool {
    if is_safe(nums) {
        return true;
    }
    for i in 0..nums.len() {
        let mut new = nums.to_vec();
        new.remove(i);
        if is_safe(&new) {
            return true;
        }
    }
    return false;
}

fn main() {
    let lines: Vec<Vec<i32>> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    let part1: usize = lines
        .iter()
        .map(|nums| is_safe(nums))
        .filter(|b| *b)
        .count();
    println!("Part 1: {}", part1);

    let part2: usize = lines
        .iter()
        .map(|nums| is_safe_with_dampener(nums))
        .filter(|b| *b)
        .count();
    println!("Part 2: {}", part2);
}
