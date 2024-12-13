use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    y: i32,
    x: i32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {y: self.y + other.y, x: self.x + other.x}
    }
}

impl Pos {
    fn adjacents(&self) -> [Self; 4] {
        [
            Self { y: self.y - 1, x: self.x },
            Self { y: self.y, x: self.x - 1 },
            Self { y: self.y, x: self.x + 1 },
            Self { y: self.y + 1, x: self.x },
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn pos(&self) -> Pos {
        match self {
            Self::Up => Pos { y: -1, x: 0 },
            Self::Right => Pos { y: 0, x: 1 },
            Self::Down => Pos { y: 1, x: 0 },
            Self::Left => Pos { y: 0, x: -1 },
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Default)]
struct Region {
    coords: HashSet<Pos>,
}

impl Region {
    fn grow_from(map: &HashMap<Pos, char>, start: Pos) -> Self {
        let name = map[&start];
        let mut coords: HashSet<Pos> = HashSet::new();
        let mut adjs: HashSet<Pos> = HashSet::from([start]);
        while coords.len() < adjs.len() {
            let cur = *adjs.difference(&coords).next().unwrap();
            coords.insert(cur);
            let nbors: HashSet<Pos> = cur.adjacents().into_iter()
                .filter(|p| map.get(p) == Some(&name)).collect();
            adjs = adjs.union(&nbors).cloned().collect();
        }
        Self { coords }
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn contains(&self, pos: &Pos) -> bool {
        self.coords.contains(pos)
    }

    fn perimeter_one(&self, pos: Pos) -> usize {
        4 - pos.adjacents().iter().filter(|p| self.contains(p)).count()
    }

    fn perimeter(&self) -> usize {
        self.coords.iter().map(|pos| self.perimeter_one(*pos)).sum()
    }

    fn outer_sides(&self) -> (usize, HashSet<Pos>) {
        let mut sides: usize = 0;
        let mut outside: HashSet<Pos> = HashSet::new();
        // Start in top, left corner, and go right
        let start = *self.coords.iter().min().unwrap();
        let mut dir = Dir::Right;
        let mut cur = start;
        outside.insert(cur + Dir::Up.pos());
        loop {
            let next = cur + dir.pos();
            let [a, b] = match dir {
                Dir::Up => [next + Dir::Left.pos(), next],
                Dir::Right => [next + Dir::Up.pos(), next],
                Dir::Down => [next + Dir::Right.pos(), next],
                Dir::Left => [next + Dir::Down.pos(), next],
            }.map(|p| self.contains(&p));
            match (a, b) {
                (_, false) => { dir = dir.turn_right(); sides += 1; outside.insert(next); },
                (false, true) => { cur = next; outside.insert(next + dir.turn_left().pos()); },
                (true, true) => { dir = dir.turn_left(); sides += 1; cur = next + dir.pos(); },
            }
            if (cur, dir) == (start , Dir::Right) {  // finished!
                break;
            }
        }
        (sides, outside)
    }

    fn inner_hole(&self, hole_pos: Pos) -> (usize, HashSet<Pos>) {
        let mut sides: usize = 0;
        let mut hole: HashSet<Pos> = HashSet::new();
        hole.insert(hole_pos);
        // Start above hole, and go right
        // let start = *hole_pos.adjacents().iter().filter(|p| self.contains(p)).min().unwrap();
        let start = hole_pos + Dir::Up.pos();
        assert!(self.contains(&start));
        let mut dir = Dir::Right;
        let mut cur = start;
        loop {
            let next = cur + dir.pos();
            let [a, b] = match dir {
                Dir::Up => [next, next + Dir::Right.pos()],
                Dir::Right => [next, next + Dir::Down.pos()],
                Dir::Down => [next, next + Dir::Left.pos()],
                Dir::Left => [next, next + Dir::Up.pos()],
            }.map(|p| self.contains(&p));
            match (a, b) {
                (false, _) => { dir = dir.turn_left(); sides += 1; hole.insert(next); },
                (true, false) => { cur = next; hole.insert(next + dir.turn_right().pos()); },
                (true, true) => { dir = dir.turn_right(); sides += 1; cur = next + dir.pos(); },
            }
            if (cur, dir) == (start, Dir::Right) {  // finished!
                break;
            }
        }
        (sides, hole)
    }

    fn sides(&self) -> usize {
        let mut nbors: HashSet<Pos> = self.coords
            .iter()
            .flat_map(|p| p.adjacents())
            .filter(|p| !self.contains(p))
            .collect();
        let (mut all_sides, outer_nbors) = self.outer_sides();
        nbors = nbors.difference(&outer_nbors).cloned().collect();
        while !nbors.is_empty() {
            let nbor = *nbors.iter().min().unwrap();
            let (inner_sides, inner_nbors) = self.inner_hole(nbor);
            all_sides += inner_sides;
            nbors = nbors.difference(&inner_nbors).cloned().collect();
        }
        all_sides
    }
}

#[derive(Debug)]
struct Garden {
    regions: Vec<Region>,
}

impl Garden {
    fn _find_regions(map: &HashMap<Pos, char>) -> Self {
        let mut remaining: HashSet<Pos> = map.keys().cloned().collect();
        let mut regions: Vec<Region> = Vec::new();
        while !remaining.is_empty() {
            let start = remaining.iter().next().unwrap();
            let region = Region::grow_from(map, *start);
            remaining = remaining.difference(&region.coords).cloned().collect();
            regions.push(region);
        }
        Self { regions }
    }

    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut bounds = Pos { y: 0, x: 0 };
        let mut map: HashMap<Pos, char> = HashMap::new();
        for (y, line) in lines.enumerate() {
            bounds = Pos {
                y: y as i32 + 1,
                x: match bounds.x {
                    0 => line.len() as i32,
                    n => { assert!(n == line.len() as i32); n },
                },
            };
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                let name = *byte as char;
                let pos = Pos { y: y as i32, x: x as i32 };
                map.insert(pos, name);
            }
        }
        Self::_find_regions(&map)
    }
}

fn main() {
    let garden = Garden::parse(io::stdin().lines().map(Result::unwrap));
    println!("Part 1: {}", garden.regions.iter().map(|r| r.area() * r.perimeter()).sum::<usize>());
    println!("Part 2: {}", garden.regions.iter().map(|r| r.area() * r.sides()).sum::<usize>());
}
