use std::collections::HashSet;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn is_adjacent(&self, other: Pos) -> bool {
        if self.y == other.y {
            return other.x == self.x + 1 || self.x == other.x + 1;
        }
        else if self.x == other.x {
            return other.y == self.y + 1 || self.y == other.y + 1;
        }
        false
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<HashSet<Pos>>,
}

impl Map {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut map: Vec<HashSet<Pos>> = vec![HashSet::new(); 10];
        for (y, line) in lines.enumerate() {
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                let num = (*byte - b'0') as usize;
                assert!(num < 10);
                // let mut set = map[num];
                map[num].insert(Pos { y, x });
            }
        }
        Self { map }
    }

    fn trailheads(&self) -> HashSet<Pos> {
        self.map[0].clone()
    }

    fn next(&self, height: usize, from: Pos) -> impl Iterator<Item = Pos> + use<'_> {
        assert!(height < 10);
        self.map[height]
            .iter()
            .cloned()
            .filter(move |p| from.is_adjacent(*p))
    }

    fn score(&self, trailhead: Pos) -> usize {
        let mut paths: HashSet<Pos> = HashSet::new();
        paths.insert(trailhead);
        for level in 1..10 {
            let mut next_paths: HashSet<Pos> = HashSet::new();
            for path in paths {
                next_paths.extend(self.next(level, path));
            }
            paths = next_paths
        }
        paths.len()
    }

    fn rate(&self, trailhead: Pos) -> usize {
        let mut paths: HashSet<Vec<Pos>> = HashSet::new();
        paths.insert(vec![trailhead]);
        for level in 1..10 {
            let mut next_paths: HashSet<Vec<Pos>> = HashSet::new();
            for path in paths {
                for next in self.next(level, path[path.len() - 1]) {
                    let mut new_path = path.to_vec();
                    new_path.push(next);
                    next_paths.insert(new_path);
                }
            }
            paths = next_paths
        }
        paths.len()
    }
}

fn main() {
    let map = Map::parse(io::stdin().lines().map(Result::unwrap));
    println!("Part 1: {}", map.trailheads().iter().map(|head| map.score(*head)).sum::<usize>());
    println!("Part 1: {}", map.trailheads().iter().map(|head| map.rate(*head)).sum::<usize>());
}
