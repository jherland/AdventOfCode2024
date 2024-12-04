use std::io;

use aho_corasick::AhoCorasick;

fn right_to_left(lines: &Vec<String>) -> impl Iterator<Item = String> + use<'_> {
    lines.iter().map(|line| line.chars().rev().collect::<String>())
}

fn top_to_bottom(lines: &Vec<String>) -> impl Iterator<Item = String> + use<'_> {
    (0..lines[0].len())
        .map(|col| (0..lines.len())
            .map(|row| lines[row].as_bytes()[col] as char)
            .collect::<String>()
        )
}

fn topleft_to_bottomright(lines: &Vec<String>) -> impl Iterator<Item = String> + use<'_> {
    (-(lines[0].len() as i64)..lines[0].len() as i64)
        .map(|col| (0..lines.len())
            .filter(|row| 0 <= (col + *row as i64) && (col + *row as i64) < lines[0].len() as i64)
            .map(|row| lines[row].as_bytes()[(col + row as i64) as usize] as char)
            .collect::<String>()
        )
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect();

    let haystack: Vec<String> = lines
        .iter()
        .cloned()
        .chain(right_to_left(&lines))
        .chain(top_to_bottom(&lines))
        .chain(right_to_left(&top_to_bottom(&lines).collect::<Vec<String>>()))
        .chain(topleft_to_bottomright(&lines))
        .chain(right_to_left(&topleft_to_bottomright(&lines).collect::<Vec<String>>()))
        .chain(topleft_to_bottomright(&right_to_left(&lines).collect::<Vec<String>>()))
        .chain(right_to_left(&topleft_to_bottomright(&right_to_left(&lines).collect::<Vec<String>>()).collect::<Vec<String>>()))
        .collect();

    let needle = "XMAS";
    let matcher = AhoCorasick::new(&[needle]).unwrap();
    let part1: usize = haystack
        .iter()
        .map(|line| matcher.find_iter(line).map(|_| 1).sum::<usize>())
        .sum();
    println!("Part 1: {}", part1);

    let chars: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let part2: usize = (1..chars.len() - 1)
        .map(|row| (1..chars[0].len() - 1)
            .filter(|col| chars[row][*col] == 'A')
            .filter(|col|
                (chars[row - 1][col - 1] == 'M' && chars[row + 1][col + 1] == 'S') ||
                (chars[row - 1][col - 1] == 'S' && chars[row + 1][col + 1] == 'M')
            )
            .filter(|col|
                (chars[row - 1][col + 1] == 'M' && chars[row + 1][col - 1] == 'S') ||
                (chars[row - 1][col + 1] == 'S' && chars[row + 1][col - 1] == 'M')
            )
            .map(|_| 1)
            .sum::<usize>()
        )
        .sum();
    println!("Part 2: {}", part2);
}
