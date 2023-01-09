const SQUARE_SIZE: usize = 99;

pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day8.txt");
    let mut table = [[false; SQUARE_SIZE]; SQUARE_SIZE];
    let mut top = [' '; SQUARE_SIZE];
    let mut bottom = [' '; SQUARE_SIZE];
    let mut left = [' '; SQUARE_SIZE];
    let mut right = [' '; SQUARE_SIZE];
    let mut data = vec![];

    for v in input.iter() {
        let r: Vec<_> = v.chars().collect();
        data.push(r);
    }

    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            if data[x][y] > top[y] {
                top[y] = data[x][y];
                table[x][y] = true;
            }
            let bx = SQUARE_SIZE - x - 1;
            if data[bx][y] > bottom[y] {
                bottom[y] = data[bx][y];
                table[bx][y] = true;
            }
            if data[y][x] > left[y] {
                left[y] = data[y][x];
                table[y][x] = true;
            }
            let rx = SQUARE_SIZE - x - 1;
            if data[y][rx] > right[y] {
                right[y] = data[y][rx];
                table[y][rx] = true;
            }
        }
    }
    let mut count = 0;
    for x in 0..SQUARE_SIZE {
        for y in 0..SQUARE_SIZE {
            if table[x][y] {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_2() -> usize {
    let input = super::utils::read("./src/input/day8.txt");
    let mut data = vec![];

    for v in input.iter() {
        let r: Vec<_> = v.chars().collect();
        data.push(r);
    }

    let mut max_score = 0;
    for x in 1..SQUARE_SIZE - 1 {
        for y in 1..SQUARE_SIZE - 1 {
            let (mut u_max, mut l_max, mut r_max, mut d_max) = (0, 0, 0, 0);
            let (mut l, mut r, mut u, mut d) = (y, y, x, x);
            while l > 0 {
                l_max += 1;
                if data[x][l - 1] >= data[x][y] {
                    break;
                }
                l = l - 1;
            }

            while r < SQUARE_SIZE - 1 {
                r_max += 1;
                if data[x][r + 1] >= data[x][y] {
                    break;
                }
                r = r + 1;
            }

            while u > 0 {
                u_max += 1;
                if data[u - 1][y] >= data[x][y] {
                    break;
                }
                u = u - 1;
            }

            while d < SQUARE_SIZE - 1 {
                d_max += 1;
                if data[d + 1][y] >= data[x][y] {
                    break;
                }
                d = d + 1;
            }

            let score = r_max * l_max * d_max * u_max;
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}
