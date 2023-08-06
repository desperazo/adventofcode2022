const BOARD_SIZE: usize = 200;
type Board = [[Field; BOARD_SIZE]; BOARD_SIZE];
pub fn solve() -> usize {
    let mut player = Player::new(8, 0);
    let (mut board, mut tok) = parse();
    while let Some(cmd) = tok.next() {
        //println!("cmd: {:?}", cmd);
        //println!("player bef: {:?}", player);
        match cmd {
            Command::Move(steps) => player.facing(steps, &mut board),
            _ => player.rotate(cmd),
        }
        //println!("player aft: {:?}", player);
        //print(&board);
        println!("--------");
    }
    player.score()
}

fn parse() -> (Board, Token) {
    let list = super::utils::read("./src/input/day22.txt");
    let mut board = [[Field::Space; BOARD_SIZE]; BOARD_SIZE];

    for (r, rv) in list.iter().enumerate() {
        // println!("{r} {}", rv);
        if rv.is_empty() {
            break;
        }
        for (c, cv) in rv.chars().enumerate() {
            board[r][c] = match cv {
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

    fn facing(&mut self, steps: usize, board: &mut Board) {
        let mut rem = steps;
        let mut x = self.x as i32;
        let mut y = self.y as i32;
        let mut tile_x = x;
        let mut tile_y = y;
        while rem > 0 {
            let m = match self.d {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
                Direction::Up => (0, -1),
            };
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

    fn score(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + self.d as usize
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

fn print(board: &Board) {
    board.iter().for_each(|l| {
        l.iter().for_each(|c| {
            let ch = match c {
                Field::Space => " ",
                Field::Wall => "#",
                Field::Tile(v) => match v {
                    Some(v) => match v {
                        Direction::Left => "<",
                        Direction::Right => ">",
                        Direction::Down => "v",
                        Direction::Up => "^",
                    },
                    None => ".",
                },
            };
            print!("{}", ch);
        });
        println!();
    });
}

#[derive(PartialEq, Clone, Copy)]
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
