const MOST_RIGHT: usize = 7;

pub fn solve() -> usize {
    let mut tunnel = Tunnel {
        hight: 0,
        stones: Vec::new(),
    };
    let input = super::utils::read("./src/input/day17.txt");
    let chrs: Vec<_> = input[0].chars().collect();
    let rock_kinds = vec![
        RockKind::Minus,
        RockKind::Plus,
        RockKind::L,
        RockKind::I,
        RockKind::Square,
    ];
    let (mut count, mut stone_count) = (0, 0);
    let mut rock = Rock::new(tunnel.hight + 3, &rock_kinds[count]);
    //debug_print(&tunnel, &rock);
    while stone_count < 2022 {
        let c = chrs[count % chrs.len()];
        //println!("moving {c}");
        if c == '<' {
            rock.move_stone(Direction::Left, &tunnel.stones);
        } else {
            rock.move_stone(Direction::Right, &tunnel.stones);
        }
        if !rock.move_stone(Direction::Down, &tunnel.stones) {
            tunnel.add_rock(&rock);
            stone_count += 1;
            rock = Rock::new(
                tunnel.hight + 4,
                &rock_kinds[stone_count % rock_kinds.len()],
            );
        }
        count += 1;
        //debug_print(&tunnel, &rock);
    }
    tunnel.hight + 1
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

struct Tunnel {
    hight: usize,
    stones: Vec<Coord>,
}

impl Tunnel {
    fn add_rock(&mut self, r: &Rock) {
        let h = r.stones.iter().map(|x| x.y).max().unwrap();
        if h > self.hight {
            self.hight = h
        }
        for s in r.stones.iter() {
            self.stones.push(*s);
        }
    }
}

enum RockKind {
    Minus,
    Plus,
    L,
    I,
    Square,
}

enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Rock {
    hight: usize,
    herizon: usize,
    stones: Vec<Coord>,
}

fn debug_print(t: &Tunnel, r: &Rock) {
    let h = r.herizon + r.hight;
    for y in (0..h).rev() {
        print!("|");
        for x in 0..MOST_RIGHT {
            if r.stones.iter().any(|s| s.x == x && s.y == y) {
                print!("@");
            } else if t.stones.iter().any(|s| s.x == x && s.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!();
    println!();
}

impl Rock {
    fn new(offset_hight: usize, kind: &RockKind) -> Self {
        match kind {
            RockKind::Minus => Rock {
                hight: 1,
                herizon: offset_hight,
                stones: vec![
                    Coord {
                        x: 2,
                        y: offset_hight,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight,
                    },
                    Coord {
                        x: 4,
                        y: offset_hight,
                    },
                    Coord {
                        x: 5,
                        y: offset_hight,
                    },
                ],
            },
            RockKind::Plus => Rock {
                hight: 3,
                herizon: offset_hight,
                stones: vec![
                    Coord {
                        x: 2,
                        y: offset_hight + 1,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 1,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 2,
                    },
                    Coord {
                        x: 4,
                        y: offset_hight + 1,
                    },
                ],
            },
            RockKind::L => Rock {
                hight: 3,
                herizon: offset_hight,
                stones: vec![
                    Coord {
                        x: 2,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 4,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 4,
                        y: offset_hight + 1,
                    },
                    Coord {
                        x: 4,
                        y: offset_hight + 2,
                    },
                ],
            },
            RockKind::I => Rock {
                hight: 4,
                herizon: offset_hight,
                stones: vec![
                    Coord {
                        x: 2,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 2,
                        y: offset_hight + 1,
                    },
                    Coord {
                        x: 2,
                        y: offset_hight + 2,
                    },
                    Coord {
                        x: 2,
                        y: offset_hight + 3,
                    },
                ],
            },
            RockKind::Square => Rock {
                hight: 2,
                herizon: offset_hight,
                stones: vec![
                    Coord {
                        x: 2,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 2,
                        y: offset_hight + 1,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 0,
                    },
                    Coord {
                        x: 3,
                        y: offset_hight + 1,
                    },
                ],
            },
        }
    }

    fn move_stone(&mut self, d: Direction, stones: &Vec<Coord>) -> bool {
        //println!("{:?}", self);
        match d {
            Direction::Left => {
                if !self.touch_left(stones) {
                    self.stones.iter_mut().for_each(|s| s.x -= 1);
                    return true;
                }
                return false;
            }
            Direction::Right => {
                if !self.touch_right(stones) {
                    self.stones.iter_mut().for_each(|s| s.x += 1);
                    return true;
                }
                return false;
            }
            Direction::Down => {
                if !self.touch_down(stones) {
                    self.stones.iter_mut().for_each(|s| s.y -= 1);
                    return true;
                }
                return false;
            }
        }
    }

    fn touch_right(&self, stones: &Vec<Coord>) -> bool {
        self.stones.iter().any(|b| {
            stones.iter().rev().any(|s| b.y == s.y && b.x + 1 == s.x) || b.x + 1 == MOST_RIGHT
        })
    }

    fn touch_left(&self, stones: &Vec<Coord>) -> bool {
        self.stones
            .iter()
            .any(|b| stones.iter().rev().any(|s| b.y == s.y && b.x == s.x + 1) || b.x == 0)
    }

    fn touch_down(&self, stones: &Vec<Coord>) -> bool {
        self.stones
            .iter()
            .any(|b| stones.iter().rev().any(|s| b.x == s.x && b.y == s.y + 1) || b.y == 0)
    }
}
