const BOARD_SIZE: usize = 202;
type Board = [[Field; BOARD_SIZE]; BOARD_SIZE];

pub fn solve() -> usize {
    let mut player = Player::new(50, 0);
    let (mut board, mut tok) = parse(0);
    while let Some(cmd) = tok.next() {
        match cmd {
            Command::Move(steps) => player.facing(steps, &mut board),
            _ => player.rotate(cmd),
        }
    }
    player.score(1)
}

pub fn solve2() -> usize {
    let mut player = Player::new(51, 1);
    let (mut board, mut tok) = parse(1);
    let borders = config_borders();

    while let Some(cmd) = tok.next() {
        match cmd {
            Command::Move(steps) => player.travel(steps, &mut board, &borders),
            _ => player.rotate(cmd),
        }
    }
    player.score(0)
}

fn config_borders() -> Vec<Border> {
    let b1 = Border::new(1, 50, 100, 100, true, Direction::Down, 2);
    let b2 = Border::new(50, 50, 51, 100, true, Direction::Right, 1);
    let b3 = Border::new(50, 50, 1, 50, false, Direction::Right, 14);
    let b4 = Border::new(51, 100, 0, 0, true, Direction::Down, 13);
    let b5 = Border::new(101, 150, 0, 0, true, Direction::Down, 12);

    let b6 = Border::new(151, 151, 1, 50, false, Direction::Left, 9);
    let b7 = Border::new(101, 150, 51, 51, true, Direction::Up, 8);
    let b8 = Border::new(101, 101, 51, 100, true, Direction::Left, 7);
    let b9 = Border::new(101, 101, 101, 150, false, Direction::Left, 6);

    let b10 = Border::new(51, 100, 151, 151, true, Direction::Up, 11);
    let b11 = Border::new(51, 51, 151, 200, true, Direction::Left, 10);
    let b12 = Border::new(1, 50, 201, 201, true, Direction::Up, 5);
    let b13 = Border::new(0, 0, 151, 200, true, Direction::Right, 4);
    let b14 = Border::new(0, 0, 101, 150, false, Direction::Right, 3);

    vec![b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14]
}

fn parse(offset: usize) -> (Board, Token) {
    let list = super::utils::read("./src/input/day22.txt");
    let mut board = [[Field::Space; BOARD_SIZE]; BOARD_SIZE];

    for (r, rv) in list.iter().enumerate() {
        if rv.is_empty() {
            break;
        }
        for (c, cv) in rv.chars().enumerate() {
            board[r + offset][c + offset] = match cv {
                '.' => Field::Tile(None),
                '#' => Field::Wall,
                _ => Field::Space,
            };
        }
    }
    let t = list.iter().last().unwrap().to_owned();
    (board, Token(t))
}

#[derive(Debug)]
struct Player {
    x: usize,
    y: usize,
    d: Direction,
}

impl Player {
    fn new(x: usize, y: usize) -> Player {
        Player {
            x,
            y,
            d: Direction::Right,
        }
    }

    fn travel(&mut self, steps: usize, board: &mut Board, borders: &[Border]) {
        let mut rem = steps;
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        let mut tile_x = x;
        let mut tile_y = y;
        while rem > 0 {
            let m = self.d.move_offset();
            x = (x + m.0).rem_euclid(BOARD_SIZE as i32);
            y = (y + m.1).rem_euclid(BOARD_SIZE as i32);
            match board[y as usize][x as usize] {
                Field::Space => {
                    let (wx, wy, wd) = self.warp_coord(x, y, borders);
                    let tm = wd.move_offset();
                    let tx = (wx as i32 + tm.0) as usize;
                    let ty = (wy as i32 + tm.1) as usize;
                    if board[ty][tx] == Field::Wall {
                        break;
                    }
                    x = wx as i32;
                    y = wy as i32;
                    self.d = wd;
                    continue;
                }
                Field::Wall => break,
                Field::Tile(_) => {
                    tile_x = x;
                    tile_y = y;
                    board[y as usize][x as usize] = Field::Tile(Some(self.d))
                }
            }
            rem -= 1;
        }
        self.x = tile_x as usize;
        self.y = tile_y as usize;
    }

    fn warp_coord(&mut self, x: i32, y: i32, borders: &[Border]) -> Coord {
        let m = self.d.move_offset();
        let mut tx = x;
        let mut ty = y;
        loop {
            for b in borders.iter() {
                if let Some(t) = b.relative_coord(tx as usize, ty as usize, self.d, borders) {
                    return t;
                }
            }
            tx = (tx + m.0).rem_euclid(BOARD_SIZE as i32);
            ty = (ty + m.1).rem_euclid(BOARD_SIZE as i32);
        }
    }

    fn facing(&mut self, steps: usize, board: &mut Board) {
        let mut rem = steps;
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        let mut tile_x = x;
        let mut tile_y = y;
        while rem > 0 {
            let m = self.d.move_offset();
            x = (x + m.0).rem_euclid(BOARD_SIZE as i32);
            y = (y + m.1).rem_euclid(BOARD_SIZE as i32);
            match board[y as usize][x as usize] {
                Field::Space => continue,
                Field::Wall => break,
                Field::Tile(_) => {
                    tile_x = x;
                    tile_y = y;
                    board[y as usize][x as usize] = Field::Tile(Some(self.d))
                }
            }
            rem -= 1;
        }
        self.x = tile_x as usize;
        self.y = tile_y as usize;
    }

    fn rotate(&mut self, cmd: Command) {
        let r = match cmd {
            Command::RotateLeft => -1,
            Command::RotateRight => 1,
            _ => panic!("rotate error"),
        };
        let e = (self.d as i32 + r).rem_euclid(4);
        let new_d = Direction::try_from(e).unwrap();
        self.d = new_d;
    }

    fn score(&self, offset: usize) -> usize {
        1000 * (self.y + offset) + 4 * (self.x + offset) + self.d as usize
    }
}

type Coord = (usize, usize, Direction);

#[derive(Debug, Clone, Copy)]
struct Border {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    relative_transfer: bool,
    direction: Direction,
    next_border: usize,
}

impl Border {
    fn new(
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
        rel: bool,
        d: Direction,
        next_border: usize,
    ) -> Self {
        Border {
            x1,
            x2,
            y1,
            y2,
            relative_transfer: rel,
            direction: d,
            next_border,
        }
    }

    fn hit(&self, x: usize, y: usize, d: Direction) -> bool {
        match d {
            Direction::Right | Direction::Left => {
                x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 && self.x1 == self.x2
            }
            Direction::Down | Direction::Up => {
                x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 && self.y1 == self.y2
            }
        }
    }

    fn relative_coord(
        &self,
        x: usize,
        y: usize,
        d: Direction,
        borders: &[Border],
    ) -> Option<Coord> {
        if !self.hit(x, y, d) {
            return None;
        }
        let dif = (y - self.y1) + (x - self.x1);
        let tar = borders[self.next_border - 1];
        match tar.direction {
            Direction::Right | Direction::Left => {
                if self.relative_transfer {
                    return Some((tar.x1, tar.y1 + dif, tar.direction));
                }
                Some((tar.x2, tar.y2 - dif, tar.direction))
            }
            Direction::Down | Direction::Up => {
                if self.relative_transfer {
                    return Some((tar.x1 + dif, tar.y1, tar.direction));
                }
                Some((tar.x2 - dif, tar.y2, tar.direction))
            }
        }
    }
}

struct Token(String);

#[derive(Debug)]
enum Command {
    Move(usize),
    RotateLeft,
    RotateRight,
}

impl Token {
    fn next(&mut self) -> Option<Command> {
        if self.0.is_empty() {
            return None;
        }
        let mut t = String::new();
        for c in self.0.chars() {
            match c {
                'L' | 'R' if t.is_empty() => {
                    self.0 = self.0[1..].to_owned();
                    if c == 'L' {
                        return Some(Command::RotateLeft);
                    }
                    return Some(Command::RotateRight);
                }
                'L' | 'R' => break,
                _ => t.push(c),
            };
        }
        self.0 = self.0[t.len()..].to_owned();
        Some(Command::Move(t.parse::<usize>().unwrap()))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Field {
    Space,
    Wall,
    Tile(Option<Direction>),
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn move_offset(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        }
    }
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Direction::Right),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
