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
