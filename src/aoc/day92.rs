use regex::Regex;
const GRID_SIZE: usize = 500;

type Grid = [[bool; GRID_SIZE]; GRID_SIZE];
pub fn solve() -> usize {
    let mut table: Grid = [[false; GRID_SIZE]; GRID_SIZE];
    table[GRID_SIZE - 1][0] = true;
    let input = super::utils::read("./src/input/day9.txt");

    let start = Position { x: 200, y: 400 };
    let mut knot = Knot::new(start);

    for v in input.iter() {
        let (m, n) = Move::parse(v);
        for _ in 0..n {
            knot.move_h(m);
            knot.move_childs(&mut table);
        }
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

struct Knot {
    childs: Vec<Position>,
}

impl Knot {
    fn move_h(&mut self, m: Move) {
        match m {
            Move::R => self.childs[0].x += 1,
            Move::L => self.childs[0].x -= 1,
            Move::D => self.childs[0].y += 1,
            Move::U => self.childs[0].y -= 1,
        }
    }

    fn move_childs(&mut self, g: &mut Grid) {
        for i in 1..10 {
            if self.childs[i].is_touch(&self.childs[i - 1]) {
                return;
            }
            let x = self.childs[i - 1].x as i32 - self.childs[i].x as i32;
            let y = self.childs[i - 1].y as i32 - self.childs[i].y as i32;
            self.childs[i].follow(x, y);
            if i == 9 {
                g[self.childs[i].x][self.childs[i].y] = true;
            }
        }
    }

    fn debug_print(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some((i, _)) = self
                    .childs
                    .iter()
                    .enumerate()
                    .filter(|v| v.1.x == x && v.1.y == y)
                    .nth(0)
                {
                    print!(" {} ", i);
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }

    fn new(start: Position) -> Self {
        Knot {
            childs: vec![
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
                start.clone(),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn is_touch(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn follow(&mut self, x: i32, y: i32) {
        match x {
            0 => (),
            1.. => self.x += 1,
            _ => self.x -= 1,
        }
        match y {
            0 => (),
            1.. => self.y += 1,
            _ => self.y -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    R,
    L,
    D,
    U,
}

impl Move {
    pub fn parse(s: &str) -> (Self, usize) {
        let regex = Regex::new(r"(\w) (\d+)").unwrap();
        let m = regex.captures(s).unwrap();
        let x: usize = m.get(2).unwrap().as_str().parse().unwrap();
        match m.get(1).unwrap().as_str() {
            "R" => (Move::R, x),
            "L" => (Move::L, x),
            "D" => (Move::D, x),
            _ => (Move::U, x),
        }
    }
}
