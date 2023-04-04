const GRID_ZIZE: usize = 22;
pub fn solve() -> i32 {
    let input = super::utils::read("./src/input/day18.txt");
    let mut grid = ThreeDGrid::new();
    for v in input.iter() {
        let vals: Vec<_> = v.split(',').collect();
        let x = vals.get(0).unwrap().parse::<usize>().unwrap();
        let y = vals.get(1).unwrap().parse::<usize>().unwrap();
        let z = vals.get(2).unwrap().parse::<usize>().unwrap();
        grid.pos[x][y][z].open_sides = 6;
    }
    grid.calculate()
}

struct ThreeDGrid {
    pos: [[[Cube; GRID_ZIZE]; GRID_ZIZE]; GRID_ZIZE],
}

impl ThreeDGrid {
    fn new() -> Self {
        let pos = [[[Cube {
            open_sides: i32::MIN,
        }; GRID_ZIZE]; GRID_ZIZE]; GRID_ZIZE];
        ThreeDGrid { pos }
    }

    fn calculate(&mut self) -> i32 {
        for x in 0..GRID_ZIZE {
            for y in 0..GRID_ZIZE {
                for z in 0..GRID_ZIZE {
                    if z > 0
                        && self.pos[x][y][z].open_sides != i32::MIN
                        && self.pos[x][y][z - 1].open_sides != i32::MIN
                    {
                        self.pos[x][y][z].open_sides -= 1;
                        self.pos[x][y][z - 1].open_sides -= 1;
                    }
                    if x > 0
                        && self.pos[x][y][z].open_sides != i32::MIN
                        && self.pos[x - 1][y][z].open_sides != i32::MIN
                    {
                        self.pos[x][y][z].open_sides -= 1;
                        self.pos[x - 1][y][z].open_sides -= 1;
                    }
                    if y > 0
                        && self.pos[x][y][z].open_sides != i32::MIN
                        && self.pos[x][y - 1][z].open_sides != i32::MIN
                    {
                        self.pos[x][y][z].open_sides -= 1;
                        self.pos[x][y - 1][z].open_sides -= 1;
                    }
                }
            }
        }
        self.pos
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| {
                        y.iter()
                            .map(|z| match z.open_sides {
                                i32::MIN => 0,
                                _ => z.open_sides,
                            })
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}

#[derive(Clone, Copy)]
struct Cube {
    open_sides: i32,
}
