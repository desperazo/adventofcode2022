use regex::Regex;
const GRID_SIZE: usize = 500;

type Grid = [[bool; GRID_SIZE]; GRID_SIZE];
pub fn solve() -> usize {
    let mut table: Grid = [[false; GRID_SIZE]; GRID_SIZE];
    table[GRID_SIZE - 1][0] = true;
    let input = super::utils::read("./src/input/day9.txt");
    let mut knot = Knot {
        h: Position { x: 200, y: 400 },
        t: Position { x: 200, y: 400 },
    };

    for v in input.iter() {
        let m = Move::parse(v);
        knot.move_h(&m);
        knot.move_t(&mut table, &m);
    }
    let mut count = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if table[x][y] {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug)]
struct Knot {
    h: Position,
    t: Position,
}

impl Knot {
    fn move_h(&mut self, m: &Move) {
        match m {
            Move::R(d) => self.h.x += d,
            Move::L(d) => self.h.x -= d,
            Move::D(d) => self.h.y += d,
            Move::U(d) => self.h.y -= d,
        }
    }

    fn move_t(&mut self, table: &mut Grid, m: &Move) {
        match self.t.is_touch(&self.h) {
            false => match m {
                Move::R(_) => self.t = self.h.touch_left(table, &self.t),
                Move::L(_) => self.t = self.h.touch_right(table, &self.t),
                Move::D(_) => self.t = self.h.touch_up(table, &self.t),
                Move::U(_) => self.t = self.h.touch_down(table, &self.t),
            },
            true => (),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn is_touch(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn touch_left(&self, table: &mut Grid, from: &Position) -> Self {
        for i in from.x + 1..self.x {
            table[self.y][i] = true;
        }
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn touch_right(&self, table: &mut Grid, from: &Position) -> Self {
        for i in self.x + 1..from.x {
            table[self.y][i] = true;
        }
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn touch_up(&self, table: &mut Grid, from: &Position) -> Self {
        for i in from.y + 1..self.y {
            table[i][self.x] = true;
        }
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn touch_down(&self, table: &mut Grid, from: &Position) -> Self {
        for i in self.y + 1..from.y {
            table[i][self.x] = true;
        }
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
enum Move {
    R(usize),
    L(usize),
    D(usize),
    U(usize),
}

impl Move {
    pub fn parse(s: &str) -> Self {
        let regex = Regex::new(r"(\w) (\d+)").unwrap();
        let m = regex.captures(s).unwrap();
        let x: usize = m.get(2).unwrap().as_str().parse().unwrap();
        match m.get(1).unwrap().as_str() {
            "R" => Move::R(x),
            "L" => Move::L(x),
            "D" => Move::D(x),
            _ => Move::U(x),
        }
    }
}
