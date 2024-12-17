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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn pos(&self) -> Pos {
        match self {
            Self::North => Pos { y: -1, x: 0 },
            Self::East => Pos { y: 0, x: 1 },
            Self::South => Pos { y: 1, x: 0 },
            Self::West => Pos { y: 0, x: -1 },
        }
    }

    fn turn_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Player {
    score: usize,
    pos: Pos,
    dir: Dir,
}

impl Player {
    fn adjacents(&self) -> impl Iterator<Item = Self> {
        let cw_dir = self.dir.turn_cw();
        let ccw_dir = self.dir.turn_ccw();
        [ // 3 possible movements:
            Player { score: self.score + 1, pos: self.pos + self.dir.pos(), dir: self.dir }, // Move one space
            Player { score: self.score + 1001, pos: self.pos + cw_dir.pos(), dir: cw_dir }, // Turn CW
            Player { score: self.score + 1001, pos: self.pos + ccw_dir.pos(), dir: ccw_dir }, // Turn CCw
        ].into_iter()
    }
}

#[derive(Debug)]
struct Maze {
    spaces: HashSet<Pos>,
    start: Pos,
    end: Pos,
}

impl Maze {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut spaces: HashSet<Pos> = HashSet::new();
        let mut start = Pos { y: 0, x: 0 };
        let mut end = Pos { y: 0, x: 0 };
        for (y, line) in lines.enumerate() {
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                if *byte == b'#' {  // wall
                    continue;
                }
                let pos = Pos { y: y as i32, x: x as i32 };
                spaces.insert(pos);
                match *byte {
                    b'S' => start = pos,
                    b'E' => end = pos,
                    _ => (),
                }
            }
        }
        Self { spaces, start, end }
    }

    fn adjacents(&self, player: Player) -> impl Iterator<Item = Player> + use<'_> {
        player.adjacents().filter(|p| self.spaces.contains(&p.pos))
    }

    fn shortest_path<F>(&self, start: Player, end: F) -> Player
    where
        F: FnOnce(Pos) -> bool + Copy,
    {
        // Dijkstra's algorithm!
        let mut unvisited: HashSet<Pos> = self.spaces.clone();
        let mut dist: HashMap<Pos, Player> = HashMap::new();
        dist.insert(start.pos, start);

        loop {
            let candidates: HashSet<Pos> = unvisited
                .intersection(&dist.keys().copied().collect())
                .copied()
                .collect();
            let current = *candidates
                .iter()
                .min_by_key(|&p| dist.get(p).unwrap().score)
                .unwrap();
            let cur_player = *dist.get(&current).unwrap();
            if end(current) {
                return cur_player;
            }
            for nbor in self
                .adjacents(cur_player)
                .filter(|player| unvisited.contains(&player.pos))
            {
                let old_nbor = dist.get(&nbor.pos);
                if old_nbor.is_none() || nbor.score < old_nbor.unwrap().score {
                    dist.insert(nbor.pos, nbor);
                }
            }
            unvisited.remove(&current);
        }
    }
}

fn main() {
    let maze = Maze::parse(io::stdin().lines().map(Result::unwrap));
    let start = Player { pos: maze.start, dir: Dir::East, score: 0 };

    let part1 = maze.shortest_path(start, |pos| pos == maze.end);
    println!("Part 1: {}", part1.score);
    // 79412 is too high

    // println!("Part 2: {}", garden.regions.iter().map(|r| r.area() * r.sides()).sum::<usize>());
}
