use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    y: i64,
    x: i64,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {y: self.y + other.y, x: self.x + other.x}
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {y: self.y - other.y, x: self.x - other.x}
    }
}

impl Coord {
    fn parse(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        Self { y: y.parse::<i64>().unwrap(), x: x.parse::<i64>().unwrap() }
    }

    fn wrap(&self, bounds: Self) -> Self {
        Self {
            y: ((self.y % bounds.y) + bounds.y) % bounds.y,
            x: ((self.x % bounds.x) + bounds.x) % bounds.x,
        }
    }
}

#[derive(Debug)]
struct Area {
    tl: Coord,
    br: Coord,
}

impl Area {
    fn from_bounds(bounds: Coord) -> Self {
        Self { tl: Coord { y: 0, x: 0 }, br: bounds }
    }

    fn quadrants(&self) -> [Self; 4] {
        let size = Coord {
            y: (self.br.y - self.tl.y) / 2,
            x: (self.br.x - self.tl.x) / 2,
        };
        // +---,+---,
        // | 1 || 2 |
        // `---+`---+
        // +---,+---,
        // | 3 || 4 |
        // `---+`---+
        [
            Self {
                tl: self.tl,
                br: self.tl + size,
            },
            Self {
                tl: Coord { y: self.tl.y, x: self.br.x - size.x },
                br: Coord { y: self.tl.y + size.y, x: self.br.x },
            },
            Self {
                tl: Coord { y: self.br.y - size.y, x: self.tl.x },
                br: Coord { y: self.br.y, x: self.tl.x + size.x },
            },
            Self {
                tl: self.br - size,
                br: self.br,
            },
        ]
    }

    fn contains(&self, p: Coord) -> bool {
        p.y >= self.tl.y && p.y < self.br.y && p.x >= self.tl.x && p.x < self.br.x
    }
}

#[derive(Clone, Debug)]
struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Robot {
    fn parse(line: &str) -> Self {
        let (pos_s, vel_s) = line.split_once(" ").unwrap();
        let pos = Coord::parse(pos_s.split_once("=").unwrap().1);
        let vel = Coord::parse(vel_s.split_once("=").unwrap().1);
        Self { pos, vel }
    }

    fn jump(&self, bounds: Coord) -> Self {
        let pos = (self.pos + self.vel).wrap(bounds);
        Self { pos, ..*self }
    }
}

fn safety_factor(bounds: Coord, robots: Vec<Robot>) -> usize {
    Area::from_bounds(bounds)
        .quadrants()
        .map(|quadrant| robots.iter().filter(|r| quadrant.contains(r.pos)).count())
        .iter()
        .product()
}

fn is_cluster(positions: &HashSet<Coord>, pos: Coord) -> bool {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
        .into_iter()
        .map(move |(y, x)| pos + Coord { y, x })
        .all(|p| positions.contains(&p))
}

fn find_xmas_tree(robots: &Vec<Robot>) -> Option<HashSet<Coord>> {
    let positions: HashSet<Coord> = robots.iter().map(|r| r.pos).collect();
    if positions.iter().any(|pos| is_cluster(&positions, *pos)) {
        return Some(positions);
    }
    None
}

fn render(positions: &HashSet<Coord>, bounds: Coord) {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            print!("{}", match positions.contains(&Coord { y, x }) {
                false => '.',
                true => '#',
            });
        }
        println!("");
    }
    println!("");
}

fn main() {
    let robots: Vec<Robot> = io::stdin().lines().map(|line| Robot::parse(&line.unwrap())).collect();
    let bounds = Coord { y: 103, x: 101 };

    let mut p1_robots = robots.to_vec();
    for _ in 0..100 {
        p1_robots = p1_robots.into_iter().map(|r| r.jump(bounds)).collect();
    }
    println!("Part 1: {}", safety_factor(bounds, p1_robots));

    // Part 2: Look for when robots line up symmetrically around the vertical center line 
    let mut p2_robots = robots.to_vec();
    let mut seconds = 0;
    let debug = false;
    loop {
        match find_xmas_tree(&p2_robots) {
            None => (),
            Some(positions) => {
                if debug {
                    println!("After {} seconds:", seconds);
                    render(&positions, bounds);
                }
                break;
            }
        }
        p2_robots = p2_robots.into_iter().map(|r| r.jump(bounds)).collect();
        seconds += 1;
    };
    println!("Part 2: {}", seconds);
}
