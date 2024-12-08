use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

use gcd::Gcd;
use itertools::Either::{Left, Right};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: i64,
    x: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {y: self.y + other.y, x: self.x + other.x}
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {y: self.y - other.y, x: self.x - other.x}
    }
}

impl Pos {
    fn scale(self, n: i64) -> Self {
        Self { y: self.y * n, x: self.x * n}
    }
}

#[derive(Debug)]
struct Map {
    size: Pos,
    antennas: HashMap<char, HashSet<Pos>>,
}

impl Map {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut size = Pos { y: 0, x: 0 };
        let mut antennas: HashMap<char, HashSet<Pos>> = HashMap::new();
        for (y, line) in lines.enumerate() {
            size = Pos {
                y: y as i64 + 1,
                x: match size.x {
                    0 => line.len() as i64,
                    n => { assert!(n == line.len() as i64); n },
                },
            };
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                if *byte != b'.' {
                    antennas
                        .entry(*byte as char)
                        .or_default()
                        .insert(Pos { y: y as i64, x: x as i64 });
                }
            }
        }
        Self { size, antennas }
    }

    fn contains(&self, p: Pos) -> bool {
        p.y >= 0 && p.y < self.size.y && p.x >= 0 && p.x < self.size.x
    }

    fn part1_antinodes(&self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> {
        [(a, b), (b, a)].into_iter().map(|(a, b)| b + (b - a))
    }

    fn part2_antinodes(&self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> {
        let diff = b - a;
        let gcd= (diff.y.abs() as u64).gcd(diff.x.abs() as u64) as i64;
        let base = Pos { y: diff.y / gcd, x: diff.x / gcd};
        (-self.size.y..self.size.y).map(move |n| a + base.scale(n))
    }

    fn antinodes_for_freq(&self, freq: char, part2: bool) -> impl Iterator<Item = Pos> + use<'_> {
        let antennas = &self.antennas[&freq];
        antennas
            .iter()
            .combinations(2)
            .map(move |pair| {
                match part2 {
                    false => Left(self.part1_antinodes(*pair[0], *pair[1])),
                    true => Right(self.part2_antinodes(*pair[0], *pair[1])),
                }
            })
            .flatten()
            .filter(move |pos| {
                match part2 {
                    false => !antennas.contains(pos),  // must not overlap with same-freq antenna
                    true => true,
                }
            })
            .filter(|pos| self.contains(*pos))  // must be within bounds
    }

    fn all_antinodes(&self, part2: bool) -> impl Iterator<Item = Pos> + use<'_> {
        self.antennas
            .keys()
            .map(move |c| self.antinodes_for_freq(*c, part2))
            .flatten()
    }
}

fn main() {
    let map = Map::parse(io::stdin().lines().map(Result::unwrap));
    println!("Part 1: {}", map.all_antinodes(false).unique().count());
    println!("Part 2: {}", map.all_antinodes(true).unique().count());
}
