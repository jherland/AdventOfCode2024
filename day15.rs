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
struct Part1Map {
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
}

impl Part1Map {
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

#[derive(Clone, Debug)]
struct Part2Map {
    walls: HashSet<Pos>,
    box_lefts: HashSet<Pos>,
    box_rights: HashSet<Pos>,
    robot: Pos,
}

impl Part2Map {
    fn extend(p1map: &Part1Map) -> Self {
        let walls: HashSet<Pos> = p1map.walls
            .iter()
            .flat_map(|p| [ Pos { y: p.y, x: p.x * 2 }, Pos { y: p.y, x: p.x * 2 + 1 }, ])
            .collect();
        let box_lefts: HashSet<Pos> = p1map.boxes
            .iter()
            .map(|p| Pos { y: p.y, x: p.x * 2 })
            .collect();
        let box_rights: HashSet<Pos> = p1map.boxes
            .iter()
            .map(|p| Pos { y: p.y, x: p.x * 2 + 1 })
            .collect();
        let robot = Pos { y: p1map.robot.y, x: p1map.robot.x * 2 };
        Self { walls, box_lefts, box_rights, robot }
    }

    fn can_move(&self, pos: Pos, dir: Dir) -> bool {
        let nbor = pos + dir.pos();
        match (dir, self.box_lefts.contains(&nbor), self.box_rights.contains(&nbor)) {
            (Dir::Up, true, _) => {
                assert!(self.box_rights.contains(&(nbor + Dir::Right.pos())));
                return self.can_move(nbor, dir) &&
                    self.can_move(nbor + Dir::Right.pos(), dir);
            },
            (Dir::Up, _, true) => {
                assert!(self.box_lefts.contains(&(nbor + Dir::Left.pos())));
                return self.can_move(nbor + Dir::Left.pos(), dir) &&
                    self.can_move(nbor, dir);
            },
            (Dir::Down, true, _) => {
                assert!(self.box_rights.contains(&(nbor + Dir::Right.pos())));
                return self.can_move(nbor, dir) &&
                    self.can_move(nbor + Dir::Right.pos(), dir);
            },
            (Dir::Down, _, true) => {
                assert!(self.box_lefts.contains(&(nbor + Dir::Left.pos())));
                return self.can_move(nbor + Dir::Left.pos(), dir) &&
                    self.can_move(nbor, dir);
            },
            (Dir::Right, true, _) => {
                assert!(self.box_rights.contains(&(nbor + dir.pos())));
                return self.can_move(nbor + dir.pos(), dir);
            },
            (Dir::Left, _, true) => {
                assert!(self.box_lefts.contains(&(nbor + dir.pos())));
                return self.can_move(nbor + dir.pos(), dir);
            },
            (Dir::Right, false, _) => {
                assert!(!self.box_rights.contains(&(nbor + dir.pos())));
                return !self.walls.contains(&nbor);
            },
            (Dir::Left, _, false) => {
                assert!(!self.box_lefts.contains(&(nbor + dir.pos())));
                return !self.walls.contains(&nbor);
            },
            (_, false, false) => {
                return !self.walls.contains(&nbor);
            },
        }
    }

    fn push_box_lefts(&self, pos: Pos, dir: Dir) -> (HashSet<Pos>, HashSet<Pos>) {
        assert!(!self.walls.contains(&pos));
        assert!(!self.box_rights.contains(&pos));
        if !self.box_lefts.contains(&pos) { // nothing in the way
            return (self.box_lefts.clone(), self.box_rights.clone());
        }
        // There is a box_left at pos and we need to push it
        let nbor = pos + dir.pos();
        let (mut box_lefts, mut box_rights) = match dir {
            Dir::Left => self.push_boxes(nbor, dir),
            Dir::Right => self.push_boxes(nbor + dir.pos(), dir),
            Dir::Up | Dir::Down => {
                let (tmp_lefts, tmp_rights) = self.push_boxes(nbor, dir);
                let tmp = Self { box_lefts: tmp_lefts, box_rights: tmp_rights, ..self.clone() };
                tmp.push_boxes(nbor + Dir::Right.pos(), dir)
            },
        };
        assert!(!box_lefts.contains(&nbor) && !box_rights.contains(&(nbor + Dir::Right.pos())));
        box_lefts.remove(&pos);
        box_lefts.insert(nbor);
        box_rights.remove(&(pos + Dir::Right.pos()));
        box_rights.insert(nbor + Dir::Right.pos());
        (box_lefts, box_rights)
    }
 
    fn push_boxes(&self, pos: Pos, dir: Dir) -> (HashSet<Pos>, HashSet<Pos>) {
        if dir == Dir::Right { // never try to push a box_right rightwards
            assert!(!self.box_rights.contains(&pos));
        } else if dir == Dir::Left { // or a box_left leftwards
            assert!(!self.box_lefts.contains(&pos));
        }

        let left_pos = match self.box_rights.contains(&pos) {
            true => pos + Dir::Left.pos(),
            false => pos,
        };
        if self.box_lefts.contains(&left_pos) {
            return self.push_box_lefts(left_pos, dir);
        }
        (self.box_lefts.clone(), self.box_rights.clone())
    }

    fn move_robot(&self, dir: Dir) -> Self {
        if !self.can_move(self.robot, dir) {
            return self.clone();
        }
        let nbor = self.robot + dir.pos();
        let (box_lefts, box_rights) = self.push_boxes(nbor, dir);
        assert!(!self.walls.contains(&nbor));
        assert!(!box_lefts.contains(&nbor));
        assert!(!box_rights.contains(&nbor));
        assert_eq!(box_lefts.intersection(&box_rights).count(), 0);
        assert_eq!(box_lefts.intersection(&self.walls).count(), 0);
        assert_eq!(box_rights.intersection(&self.walls).count(), 0);
        assert_eq!(box_lefts.iter().map(|p| *p + Dir::Right.pos()).collect::<HashSet<Pos>>(), box_rights);
        Self { box_lefts, box_rights, robot: nbor, ..self.clone() }
    }

    fn boxes_gps(&self) -> i32 {
        self.box_lefts.iter().map(|p| p.gps()).sum()
    }

    fn render(&self) {
        let max_y = self.walls.iter().map(|p| p.y).max().unwrap();
        let max_x = self.walls.iter().map(|p| p.x).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos { y, x };
                if self.robot == pos {
                    print!("@");
                } else if self.box_lefts.contains(&pos) {
                    print!("[");
                } else if self.box_rights.contains(&pos) {
                    print!("]");
                } else if self.walls.contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    const DEBUG: bool = false;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (map_s, moves_s) = input.split_once("\n\n").unwrap();
    let map = Part1Map::parse(map_s.split("\n").into_iter().map(|s| s.to_string()));
    let moves: Vec<Dir> = moves_s.chars().filter(|c| *c != '\n').map(|c| Dir::parse(c)).collect();

    let mut part1 = map.clone();
    for dir in moves.iter() {
        part1 = part1.move_robot(*dir);
    }
    println!("Part 1: {}", part1.boxes_gps());

    let mut part2 = Part2Map::extend(&map);
    for dir in moves.iter() {
        if DEBUG {
            part2.render();
            dbg!(part2.robot, dir);
        }
        part2 = part2.move_robot(*dir);
    }
    println!("Part 2: {}", part2.boxes_gps());
}
