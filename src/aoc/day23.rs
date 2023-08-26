use std::collections::{HashMap, HashSet};

pub fn solve() -> usize {
    let mut map = parse();
    for i in 0..10 {
        map.distribute();
        println!("------{i}------");
    }
    map.score()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Elf {
    x: i32,
    y: i32,
}

impl Elf {
    fn purpose(&self, map: &Map) -> Option<Elf> {
        if self.is_idle(map) {
            return None;
        }
        for d in map.compass.current_directions().iter() {
            let offset = d.offset();
            let free = [offset.0, offset.1, offset.2]
                .iter()
                .map(|(x, y)| Elf {
                    x: self.x + x,
                    y: self.y + y,
                })
                .all(|elf| !map.elfs.contains(&elf));
            if free {
                return Some(Elf {
                    x: self.x + offset.1 .0,
                    y: self.y + offset.1 .1,
                });
            }
        }
        None
    }

    fn is_idle(&self, map: &Map) -> bool {
        Direction::all()
            .iter()
            .flat_map(|d| {
                let offset = d.offset();
                [offset.0, offset.1, offset.2]
            })
            .map(|(x, y)| Elf {
                x: self.x + x,
                y: self.y + y,
            })
            .all(|elf| !map.elfs.contains(&elf))
    }
}

struct Map {
    compass: Compass,
    elfs: HashSet<Elf>,
    purposes: HashMap<Elf, Vec<Elf>>,
}

impl Map {
    fn score(&self) -> usize {
        let min_x = self.elfs.iter().map(|elf| elf.x).min().unwrap();
        let max_x = self.elfs.iter().map(|elf| elf.x).max().unwrap();
        let min_y = self.elfs.iter().map(|elf| elf.y).min().unwrap();
        let max_y = self.elfs.iter().map(|elf| elf.y).max().unwrap();
        let width_x = min_x.abs_diff(max_x) as usize + 1;
        let width_y = min_y.abs_diff(max_y) as usize + 1;
        width_x * width_y - self.elfs.len()
    }

    fn purposes(&mut self) {
        for elf in self.elfs.iter() {
            if let Some(pro) = elf.purpose(self) {
                self.purposes
                    .entry(pro)
                    .and_modify(|v| v.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
    }

    fn distribute(&mut self) {
        self.purposes();
        self.print();
        for (pro, v) in self.purposes.iter() {
            if v.len() == 1 {
                let cur = v.get(0).unwrap();
                self.elfs.remove(cur);
                self.elfs.insert(*pro);
            }
        }
        self.purposes.clear();
        self.compass.rotate();
        self.print();
    }

    fn print(&self) {
        for y in 0..13 {
            for x in 0..15 {
                let elf = Elf { x, y };
                let c = match self.purposes.get(&elf) {
                    Some(v) => v.len(),
                    None => 0,
                };
                match self.elfs.contains(&elf) {
                    true => print!("#"),
                    false => match c {
                        0 => print!("."),
                        _ => print!("{c}"),
                    },
                }
            }
            println!();
        }
        println!();
    }
}

struct Compass {
    count: usize,
}

impl Compass {
    fn new() -> Self {
        Compass { count: 0 }
    }

    fn rotate(&mut self) -> Direction {
        let direction = match self.count % 4 {
            0 => Direction::N,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::E,
            _ => unreachable!(),
        };
        self.count += 1;
        direction
    }

    fn current_directions(&self) -> Vec<Direction> {
        let mut d = Direction::all();
        d.rotate_left(self.count % 4);
        d
    }
}

#[derive(Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn offset(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        match self {
            Direction::N => ((-1, -1), (0, -1), (1, -1)),
            Direction::S => ((-1, 1), (0, 1), (1, 1)),
            Direction::W => ((-1, -1), (-1, 0), (-1, 1)),
            Direction::E => ((1, -1), (1, 0), (1, 1)),
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::N, Direction::S, Direction::W, Direction::E]
    }
}

fn parse() -> Map {
    let list = super::utils::read("./src/input/day23.txt");
    let mut map = Map {
        elfs: HashSet::new(),
        purposes: HashMap::new(),
        compass: Compass { count: 0 },
    };
    for (y, rv) in list.iter().enumerate() {
        if rv.is_empty() {
            break;
        }
        for (x, c) in rv.chars().enumerate() {
            if c == '#' {
                map.elfs.insert(Elf {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    map
}
