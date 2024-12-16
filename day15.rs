use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::Read;
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
    fn gps(&self) -> i32 {
        self.y * 100 + self.x
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
    fn parse(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!(),
        }
    }

    fn pos(&self) -> Pos {
        match self {
            Self::Up => Pos { y: -1, x: 0 },
            Self::Right => Pos { y: 0, x: 1 },
            Self::Down => Pos { y: 1, x: 0 },
            Self::Left => Pos { y: 0, x: -1 },
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
}

impl Map {
    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut bounds = Pos { y: 0, x: 0 };
        let mut walls: HashSet<Pos> = HashSet::new();
        let mut boxes: HashSet<Pos> = HashSet::new();
        let mut robot = Pos { y: 0, x: 0 };
        for (y, line) in lines.enumerate() {
            bounds = Pos {
                y: y as i32 + 1,
                x: match bounds.x {
                    0 => line.len() as i32,
                    n => { assert!(n == line.len() as i32); n },
                },
            };
            for (x, byte) in line.as_bytes().into_iter().enumerate() {
                let pos = Pos { y: y as i32, x: x as i32 };
                match *byte as char {
                    '#' => { walls.insert(pos); },
                    'O' => { boxes.insert(pos); },
                    '@' => { robot = pos; },
                    _ => (),
                }
            }
        }
        Self { walls, boxes, robot }
    }

    fn can_move(&self, pos: Pos, dir: Dir) -> bool {
        let nbor = pos + dir.pos();
        if self.boxes.contains(&nbor) {
            return self.can_move(nbor, dir);
        } else if self.walls.contains(&nbor) {
            return false;
        }
        true
    }

    fn push_boxes(&self, pos: Pos, dir: Dir) -> HashSet<Pos> {
        if !self.boxes.contains(&pos) {
            assert!(!self.walls.contains(&pos));
            return self.boxes.clone();
        }
        let nbor = pos + dir.pos();
        let mut boxes = self.push_boxes(nbor, dir);
        assert!(!boxes.contains(&nbor));
        boxes.remove(&pos);
        boxes.insert(nbor);
        boxes
    }
 
    fn move_robot(&self, dir: Dir) -> Self {
        if !self.can_move(self.robot, dir) {
            return self.clone();
        }
        // dbg!(self.robot, dir);
        let nbor = self.robot + dir.pos();
        let boxes = self.push_boxes(nbor, dir);
        assert!(!self.walls.contains(&nbor));
        assert!(!boxes.contains(&nbor));
        Self { boxes, robot: nbor, ..self.clone() }
    }

    fn boxes_gps(&self) -> i32 {
        self.boxes.iter().map(|p| p.gps()).sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map_s, moves_s) = input.split_once("\n\n").unwrap();

    let map = Map::parse(map_s.split("\n").into_iter().map(|s| s.to_string()));
    // dbg!(&map);

    let moves: Vec<Dir> = moves_s.chars().filter(|c| *c != '\n').map(|c| Dir::parse(c)).collect();
    // dbg!(&moves);

    let mut part1 = map.clone();
    for dir in moves {
        part1 = part1.move_robot(dir);
    }
    println!("Part 1: {}", part1.boxes_gps());
}
