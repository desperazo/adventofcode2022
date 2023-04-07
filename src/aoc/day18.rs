const GRID_SIZE: usize = 24;
const EDGE_POS: usize = GRID_SIZE - 1;
pub fn solve() -> i32 {
    let mut grid = parse_grid();
    grid.droppet_size()
}

pub fn solve_2() -> usize {
    let mut grid = parse_grid();
    grid.droppet_external_size()
}

fn parse_grid() -> ThreeDGrid {
    let input = super::utils::read("./src/input/day18.txt");
    let mut grid = ThreeDGrid::new();
    for v in input.iter() {
        let vals: Vec<_> = v.split(',').collect();
        let x = vals.get(0).unwrap().parse::<usize>().unwrap() + 1;
        let y = vals.get(1).unwrap().parse::<usize>().unwrap() + 1;
        let z = vals.get(2).unwrap().parse::<usize>().unwrap() + 1;
        grid.pos[x][y][z].exposed_sides = 6;
        grid.pos[x][y][z].kind = CubeKind::Lava;
    }
    grid
}

struct ThreeDGrid {
    pos: [[[Cube; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
}

impl ThreeDGrid {
    fn new() -> Self {
        let pos = [[[Cube {
            exposed_sides: i32::MIN,
            kind: CubeKind::Undefined,
        }; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
        ThreeDGrid { pos }
    }

    fn render_lava(&mut self) {
        for x in 1..GRID_SIZE {
            for y in 1..GRID_SIZE {
                for z in 1..GRID_SIZE {
                    if self.pos[x][y][z].kind != CubeKind::Lava {
                        continue;
                    }
                    if self.pos[x][y][z - 1].kind == CubeKind::Lava {
                        self.pos[x][y][z].exposed_sides -= 1;
                        self.pos[x][y][z - 1].exposed_sides -= 1;
                    }
                    if self.pos[x - 1][y][z].kind == CubeKind::Lava {
                        self.pos[x][y][z].exposed_sides -= 1;
                        self.pos[x - 1][y][z].exposed_sides -= 1;
                    }
                    if self.pos[x][y - 1][z].kind == CubeKind::Lava {
                        self.pos[x][y][z].exposed_sides -= 1;
                        self.pos[x][y - 1][z].exposed_sides -= 1;
                    }
                }
            }
        }
    }

    fn droppet_size(&mut self) -> i32 {
        self.render_lava();
        self.pos
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| {
                        y.iter()
                            .map(|z| match z.exposed_sides {
                                i32::MIN => 0,
                                _ => z.exposed_sides,
                            })
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }

    fn render_surface(&mut self) {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for z in 0..GRID_SIZE {
                    let rev = GRID_SIZE - z - 1;
                    self.render_air(x, y, z);
                    self.render_air(x, y, rev);
                    self.render_air(y, x, z);
                    self.render_air(y, x, rev);

                    self.render_air(x, z, y);
                    self.render_air(x, rev, y);
                    self.render_air(y, z, x);
                    self.render_air(y, rev, x);

                    self.render_air(z, x, y);
                    self.render_air(rev, x, y);
                    self.render_air(z, y, x);
                    self.render_air(rev, y, x);
                }
            }
        }
    }

    fn render_air(&mut self, x: usize, y: usize, z: usize) {
        if self.pos[x][y][z].kind == CubeKind::Air {
            return;
        }
        if is_edge_pos(x, y, z) {
            self.pos[x][y][z].kind = CubeKind::Air;
            return;
        }
        let map = vec![
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ];
        let connect_air = map
            .iter()
            .any(|(x, y, z)| self.pos[*x][*y][*z].kind == CubeKind::Air);
        if connect_air {
            if self.pos[x][y][z].kind == CubeKind::Undefined {
                self.pos[x][y][z].kind = CubeKind::Air;
            }
            if self.pos[x][y][z].kind == CubeKind::Lava {
                self.pos[x][y][z].kind = CubeKind::Surface;
            }
        }
    }

    fn droppet_external_size(&mut self) -> usize {
        self.render_surface();
        let mut sum = 0;
        for x in 1..GRID_SIZE {
            for y in 1..GRID_SIZE {
                for z in 1..GRID_SIZE {
                    if self.pos[x][y][z].kind != CubeKind::Surface {
                        continue;
                    }
                    let map = vec![
                        (x - 1, y, z),
                        (x + 1, y, z),
                        (x, y - 1, z),
                        (x, y + 1, z),
                        (x, y, z - 1),
                        (x, y, z + 1),
                    ];
                    let cnt = map
                        .iter()
                        .filter(|(x, y, z)| self.pos[*x][*y][*z].kind == CubeKind::Air)
                        .count();
                    sum += cnt;
                }
            }
        }
        sum
    }
}

fn is_edge_pos(x: usize, y: usize, z: usize) -> bool {
    x == 0 || y == 0 || z == 0 || x == EDGE_POS || y == EDGE_POS || z == EDGE_POS
}

#[derive(Clone, Copy)]
struct Cube {
    exposed_sides: i32,
    kind: CubeKind,
}

#[derive(Clone, Copy, PartialEq)]
enum CubeKind {
    Undefined,
    Air,
    Lava,
    Surface,
}
