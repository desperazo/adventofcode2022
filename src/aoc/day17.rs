const MOST_RIGHT: usize = 7;

pub fn solve(expect_rock: usize) -> usize {
    let mut tunnel = Tunnel {
        hight: 0,
        acc_hight: 0,
        rock_count: 0,
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
    let mut count = 0;
    let mut base_acc_hight = 0;
    let mut base_rock_count = 0;
    let mut rock = Rock::new(tunnel.hight + 3, &rock_kinds[count]);

    while tunnel.rock_count < expect_rock {
        let c = chrs[count % chrs.len()];
        if c == '<' {
            rock.move_stone(Direction::Left, &tunnel.stones);
        } else {
            rock.move_stone(Direction::Right, &tunnel.stones);
        }
        if !rock.move_stone(Direction::Down, &tunnel.stones) {
            tunnel.add_rock(&rock);
            rock = Rock::new(
                tunnel.hight + 4,
                &rock_kinds[tunnel.rock_count % rock_kinds.len()],
            );
        }
        rock.drop_count += 1;
        count += 1;
        if count % chrs.len() == 0 && count > chrs.len() {
            if count / chrs.len() == 2 {
                base_acc_hight = tunnel.acc_hight;
                base_rock_count = tunnel.rock_count;
            } else {
                let dif_hight = tunnel.acc_hight - base_acc_hight;
                let dif_count = tunnel.rock_count - base_rock_count;
                let times = (expect_rock - tunnel.rock_count) / dif_count;
                let hight = dif_hight * times;
                let rock_count = dif_count * times;
                tunnel.acc_hight += hight;
                tunnel.rock_count += rock_count;
            }
        }
    }
    tunnel.acc_hight + tunnel.hight + 1
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Tunnel {
    hight: usize,
    acc_hight: usize,
    rock_count: usize,
    stones: Vec<Coord>,
}

impl Tunnel {
    fn add_rock(&mut self, r: &Rock) {
        self.rock_count += 1;
        let h = r.stones.iter().map(|x| x.y).max().unwrap();
        if h > self.hight {
            self.hight = h
        }
        for s in r.stones.iter() {
            self.stones.push(*s);
        }
        let mut min = usize::MAX;
        for i in 0..MOST_RIGHT {
            let tm = self.stones.iter().rev().filter(|x| x.x == i).nth(0);
            if tm.is_none() {
                return;
            }
            min = std::cmp::min(min, tm.unwrap().y);
        }
        self.acc_hight += min;
        self.hight -= min;
        let mut tmp = vec![];
        for vr in self.stones.iter().filter(|x| x.y >= min) {
            tmp.push(Coord {
                x: vr.x,
                y: vr.y - min,
            });
        }
        self.stones = Vec::from_iter(tmp);
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
    left: usize,
    right: usize,
    drop_count: usize,
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
                left: 2,
                right: 5,
                drop_count: 0,
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
                drop_count: 0,
                herizon: offset_hight,
                left: 2,
                right: 4,
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
                drop_count: 0,
                herizon: offset_hight,
                left: 2,
                right: 4,
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
                drop_count: 0,
                herizon: offset_hight,
                left: 2,
                right: 2,
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
                drop_count: 0,
                herizon: offset_hight,
                left: 2,
                right: 3,
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
        match d {
            Direction::Left => {
                if !self.touch_left(stones) {
                    self.stones.iter_mut().for_each(|s| s.x -= 1);
                    self.left -= 1;
                    self.right -= 1;
                    return true;
                }
                return false;
            }
            Direction::Right => {
                if !self.touch_right(stones) {
                    self.stones.iter_mut().for_each(|s| s.x += 1);
                    self.left += 1;
                    self.right += 1;
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
        if self.drop_count <= 2 && self.right < MOST_RIGHT - 1 {
            return false;
        }

        self.stones.iter().any(|b| {
            stones.iter().rev().any(|s| b.y == s.y && b.x + 1 == s.x) || b.x + 1 == MOST_RIGHT
        })
    }

    fn touch_left(&self, stones: &Vec<Coord>) -> bool {
        if self.drop_count <= 2 && self.left > 0 {
            return false;
        }
        self.stones
            .iter()
            .any(|b| stones.iter().rev().any(|s| b.y == s.y && b.x == s.x + 1) || b.x == 0)
    }

    fn touch_down(&self, stones: &Vec<Coord>) -> bool {
        if self.drop_count <= 2 {
            return false;
        }
        self.stones
            .iter()
            .any(|b| stones.iter().rev().any(|s| b.x == s.x && b.y == s.y + 1) || b.y == 0)
    }
}
