use std::collections::{HashSet, VecDeque};

const MAX_ROW: usize = 22;
const MAX_COL: usize = 152;
pub fn solve() -> usize {
    let (mut map, mut bizs) = parse();
    let mut best_steps = 500;
    let mut queue = VecDeque::from_iter(vec![Elf {
        x: 1,
        y: 0,
        steps: 0,
    }]);

    let mut visited = HashSet::new();
    while !queue.is_empty() {
        bizs.iter_mut().for_each(|b| b.flow(&mut map));

        let mut moved_elfs = Vec::new();
        while let Some(mut elf) = queue.pop_back() {
            if elf.x == MAX_COL - 2 && elf.y == MAX_ROW - 1 {
                best_steps = best_steps.min(elf.steps);
                continue;
            }
            if elf.steps >= best_steps {
                continue;
            }

            elf.walk(&map).iter().for_each(|e| {
                if e.steps < best_steps && visited.insert(*e) {
                    moved_elfs.push(*e);
                }
            });
        }
        queue.extend(moved_elfs);
    }
    best_steps
}

type Map = [[usize; MAX_COL]; MAX_ROW];
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Elf {
    x: usize,
    y: usize,
    steps: usize,
}

impl Elf {
    fn walk(&mut self, map: &Map) -> Vec<Elf> {
        self.steps += 1;
        let mut elfs = Direction::all()
            .iter()
            .filter_map(|d| {
                let (x, y) = d.next_position(self.x, self.y);
                match map[y][x] {
                    0 => Some(Elf {
                        x,
                        y,
                        steps: self.steps,
                    }),
                    _ => None,
                }
            })
            .collect::<Vec<Elf>>();
        if map[self.y][self.x] == 0 {
            elfs.push(Elf {
                x: self.x,
                y: self.y,
                steps: self.steps,
            })
        }
        elfs
    }
}

struct Blizzard {
    dir: Direction,
    x: usize,
    y: usize,
}

impl Blizzard {
    fn flow(&mut self, map: &mut Map) {
        map[self.y][self.x] -= self.dir.value();
        loop {
            (self.x, self.y) = self.dir.next_position(self.x, self.y);
            if map[self.y][self.x] != usize::MAX {
                break;
            }
        }
        map[self.y][self.x] += self.dir.value();
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn value(&self) -> usize {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }

    fn next_position(&self, x: usize, y: usize) -> (usize, usize) {
        let (ox, oy) = self.offset();
        let new_x = (x as i32 + ox).rem_euclid(MAX_COL as i32) as usize;
        let new_y = (y as i32 + oy).rem_euclid(MAX_ROW as i32) as usize;
        (new_x, new_y)
    }

    fn all() -> Vec<Direction> {
        use Direction::*;
        vec![Up, Right, Down, Left]
    }
}

fn parse() -> (Map, Vec<Blizzard>) {
    let list = super::utils::read("./src/input/day24.txt");
    let mut map = [[0; MAX_COL]; MAX_ROW];
    let mut bilizzards = vec![];
    for (y, rv) in list.iter().enumerate() {
        if rv.is_empty() {
            break;
        }
        for (x, c) in rv.chars().enumerate() {
            let dir = match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                '^' => Direction::Up,
                'v' => Direction::Down,
                '#' => {
                    map[y][x] = usize::MAX;
                    continue;
                }
                _ => continue,
            };
            map[y][x] = dir.value();
            bilizzards.push(Blizzard { dir, y, x })
        }
    }
    (map, bilizzards)
}

fn print(map: &Map, bils: &[Blizzard], elf: &Elf, isprint: bool) {
    if !isprint {
        return;
    }
    for y in 0..MAX_ROW {
        for x in 0..MAX_COL {
            if elf.x == x && elf.y == y {
                print!("E");
                continue;
            }
            match map[y][x] {
                0 => print!("."),
                1 => {
                    let biz = bils.iter().find(|b| b.x == x && b.y == y).unwrap();
                    match biz.dir {
                        Direction::Up => print!("^"),
                        Direction::Down => print!("v"),
                        Direction::Left => print!("<"),
                        Direction::Right => print!(">"),
                    }
                }
                v if v == usize::MAX => print!("#"),
                v if v > 1 => print!("M"),
                _ => (),
            }
        }
        println!();
    }
    println!();
    println!();
}
