use std::collections::HashMap;
use std::io;

type Stones = HashMap<usize, usize>;

fn blink_one(num: usize) -> Vec<usize> {
    match num {
        0 => vec![1],
        n if ((n as f64).log10() as usize + 1) % 2 == 0 => {
            let s = num.to_string();
            vec![
                s[..s.len() / 2].parse::<usize>().unwrap(),
                s[s.len() / 2..].parse::<usize>().unwrap(),
            ]
        },
        _ => vec![num * 2024],
    }
}

fn blink(stones: &Stones) -> Stones {
    let mut ret: Stones = HashMap::new();
    for (stone, count) in stones.iter() {
        for new_stone in blink_one(*stone) {
            *ret.entry(new_stone).or_insert(0) += count;
        }
    }
    ret
}

fn main() {
    let mut stones: Stones = HashMap::new();
    for s in io::stdin().lines().map(Result::unwrap).next().unwrap().split(" ") {
        *stones.entry(s.parse::<usize>().unwrap()).or_insert(0) += 1;
    }

    for _ in 0..25 {
        stones = blink(&stones);
    }
    println!("Part 1: {}", stones.values().sum::<usize>());

    for _ in 25..75 {
        stones = blink(&stones);
    }
    println!("Part 2: {}", stones.values().sum::<usize>());
}
