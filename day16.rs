use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::io;
use std::ops::Add;

use itertools::Itertools;

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

type Score = usize;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pos: Pos,  // Where are we
    dir: Dir,  // Which direction are we facing upon entering this location
}

impl State {
    fn adjacents(&self) -> impl Iterator<Item = (Self, Score)> {
        let cw_dir = self.dir.turn_cw();
        let ccw_dir = self.dir.turn_ccw();
        [
            (State { pos: self.pos + self.dir.pos(), dir: self.dir }, 1), // Move one space
            (State { pos: self.pos + cw_dir.pos(), dir: cw_dir }, 1001), // Turn CW + move
            (State { pos: self.pos + ccw_dir.pos(), dir: ccw_dir }, 1001), // Turn CCw + move
        ].into_iter()
    }
}

type Path = Vec<State>;

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

    fn next_moves(&self, player: State) -> impl Iterator<Item = (State, Score)> + use<'_> {
        player.adjacents().filter(|(p, _)| self.spaces.contains(&p.pos))
    }

    fn shortest_paths(&self, start: State, end: Pos) -> Option<(Score, Vec<Path>)> {
        let mut min_scores: HashMap<State, Score> = HashMap::new();
        let mut next_moves: BinaryHeap<Reverse<(Score, State)>> = BinaryHeap::new();
        next_moves.push(Reverse((0, start)));
        let mut prev_states: HashMap<State, HashSet<State>> = HashMap::new();
        let mut end_state = None;
        while let Some(Reverse((score, current))) = next_moves.pop() {
            if current.pos == end {
                end_state = Some((current, score));
                break;
            }
            for (next, d_score) in self.next_moves(current) {
                let old_score = *min_scores.get(&next).unwrap_or(&Score::MAX);
                let new_score = score + d_score;
                if new_score < old_score {
                    min_scores.insert(next, new_score);
                    next_moves.push(Reverse((new_score, next)));
                    prev_states.insert(next, HashSet::new());
                }
                if new_score <= old_score {
                    prev_states.entry(next).and_modify(|set| { set.insert(current); });
                }
            }
        }

        fn build_rev_paths(state: State, start: State, prev_states: &HashMap<State, HashSet<State>>) -> Vec<Path> {
            if state == start {
                return vec![vec![state]];
            }
            let mut ret = vec![];
            for prev in &prev_states[&state] {
                for path in build_rev_paths(*prev, start, prev_states) {
                    let mut new_path = path.to_vec();
                    new_path.push(state);
                    ret.push(new_path)
                }
            }
            ret
        }

        match end_state {
            None => None,
            Some((state, score)) => Some(
                (score, build_rev_paths(state, start, &prev_states))
            ),
        }
    }

}

fn main() {
    let maze = Maze::parse(io::stdin().lines().map(Result::unwrap));
    let start = State { pos: maze.start, dir: Dir::East };

    let (min_score, min_paths) = maze.shortest_paths(start, maze.end).unwrap();
    println!("Part 1: {}", min_score);
    println!("Part 2: {}", min_paths
        .iter()
        .map(|path| path.iter())
        .flatten()
        .map(|state| state.pos)
        .unique()
        .count()
    );
}
