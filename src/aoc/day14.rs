use std::{cmp, collections::VecDeque};

const SQUARE_SIZE: usize = 1000;

pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day14.txt");
    let mut data = vec![];
    for _y in 0..SQUARE_SIZE {
        let mut row = Vec::with_capacity(SQUARE_SIZE);
        for _x in 0..SQUARE_SIZE {
            row.push(Kind::Air);
        }
        data.push(row);
    }
    for line in input.iter() {
        let mut queue = VecDeque::new();
        for pos in line.split(" -> ").into_iter() {
            let kv: Vec<&str> = pos.split(',').collect();
            let x: usize = kv[0].parse().unwrap();
            let y: usize = kv[1].parse().unwrap();
            queue.push_back((x, y));
        }
        let mut prev = queue.pop_front().unwrap();
        while let Some(next) = queue.pop_front() {
            if prev.0 == next.0 {
                for i in cmp::min(prev.1, next.1)..=cmp::max(prev.1, next.1) {
                    data[i][prev.0] = Kind::Rock;
                }
            } else {
                for i in cmp::min(prev.0, next.0)..=cmp::max(prev.0, next.0) {
                    data[prev.1][i] = Kind::Rock;
                }
            }
            prev = next;
        }
    }
    let mut g = Grid {
        data,
        count: 0,
        x: 500,
    };
    // g.print();
    g.fill_sands();
    g.count
}

struct Grid {
    data: Vec<Vec<Kind>>,
    count: usize,
    x: usize,
}

impl Grid {
    fn fill_sands(&mut self) {
        loop {
            let mut y = 0;
            let mut x = self.x;
            loop {
                if y + 1 == SQUARE_SIZE {
                    return;
                }
                if self.data[y + 1][x] == Kind::Air {
                    y += 1;
                } else if self.data[y + 1][x - 1] == Kind::Air {
                    y += 1;
                    x -= 1;
                } else if self.data[y + 1][x + 1] == Kind::Air {
                    y += 1;
                    x += 1;
                } else {
                    self.count += 1;
                    break;
                }
            }
            self.data[y][x] = Kind::Occupied;
            // self.print();
        }
    }

    fn print(&self) {
        for row in self.data.iter() {
            let mut line = String::new();
            for v in row.iter() {
                line.push_str(match v {
                    Kind::Air => ".",
                    Kind::Rock => "#",
                    Kind::Occupied => "O",
                });
            }
            println!("{}", line);
        }
        println!("---------------");
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Air,
    Rock,
    Occupied,
}
