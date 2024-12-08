use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: usize,
    x: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn jump(&self, pos: Pos) -> Option<Pos> {
        match self {
            Self::Up => if pos.y == 0 { None } else { Some(Pos { y: pos.y - 1, x: pos.x}) },
            Self::Right => Some(Pos { y: pos.y, x: pos.x + 1}),
            Self::Down => Some(Pos { y: pos.y + 1, x: pos.x}),
            Self::Left => if pos.x == 0 { None } else { Some(Pos { y: pos.y, x: pos.x - 1}) },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Step {
    pos: Pos,
    dir: Dir,
}


#[derive(Debug)]
struct World {
    map: Vec<Vec<bool>>,
    size: Pos,
    start: Pos
}

impl World {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut width = None;
        let mut start = Pos { y: 0, x: 0 };
        let mut map = Vec::new();
        for (y, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                row.push(*byte == b'#');
                if *byte == b'^' {  // found start point
                    start = Pos { y, x };
                }
            }
            match width {
                None => width = Some(row.len()),
                Some(n) => assert!(n == row.len()),
            }
            map.push(row);
        }
        let size = Pos { y: map.len(), x: width.unwrap() };
        Self { map, size, start }
    }

    fn contains(&self, p: Pos) -> bool {
        p.y < self.size.y && p.x < self.size.x
    }

    fn occupied(&self, p: Pos) -> Option<bool> {
        if ! self.contains(p) { return None; }
        Some(self.map[p.y][p.x])
    }

    fn next_step(&self, step: Step) -> Option<Step> {
        match step.dir.jump(step.pos) {
            None => None,
            Some(pos) => match self.occupied(pos) {
                None => None,
                Some(true) => Some(Step { pos: step.pos, dir: step.dir.turn() }),
                Some(false) => Some(Step { pos, dir: step.dir }),
            }
        }
    }

    fn patrol(&self, start: Step) -> impl Iterator<Item = Result<Step, ()>> + use<'_> {
        let mut seen: HashSet<Step> = HashSet::new();
        let mut next = Some(start);

        std::iter::from_fn(move || {
            let cur = match next {
                None => return None,  // Out of bounds -- patrol is finished
                Some(step) => step,
            };
            if seen.contains(&cur) { // Loop detected -- signal by yielding Err
                return Some(Err(()));
            }
            seen.insert(cur);
            next = self.next_step(cur);
            Some(Ok(cur))
        })
    }

    fn add_obstruction(&self, pos: Pos) -> Self {
        let mut new_map = self.map.to_vec();
        new_map[pos.y][pos.x] = true;
        Self { map: new_map, size: self.size, start: self.start }
    }
}

fn main() {
    let world = World::parse(io::stdin().lines().map(Result::unwrap));
    let start = Step { pos: world.start, dir: Dir::Up };
    let path: HashMap<Step, usize> = world
        .patrol(start)
        .flatten()
        .enumerate()
        .map(|(n, step)| (step, n))
        .collect();
    println!("Part 1: {}", path.keys().unique_by(|step| step.pos).count());

    // For each step on the path, add an obstruction and see if it causes a loop
    let part2 = path
        .iter()
        .map(|(step, _)| step.pos)
        .unique()
        .filter(|pos| world.add_obstruction(*pos).patrol(start).any(|step| step.is_err()))
        .count();
    println!("Part 2: {}", part2); // 2262
}
