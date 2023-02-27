use regex::Regex;
pub fn solve() -> i32 {
    let input = super::utils::read("./src/input/day15.txt");
    let reg = Regex::new(r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$").unwrap();
    let row_check = 2_000_000;
    let mut rnk = vec![];
    let mut b_points = vec![];
    for v in input.iter() {
        if let Some(cap) = reg.captures(v) {
            let pos = Position::new(
                cap["sx"].parse().unwrap(),
                cap["sy"].parse().unwrap(),
                cap["bx"].parse().unwrap(),
                cap["by"].parse().unwrap(),
            );
            if let Some(c) = pos.collapse(row_check) {
                rnk.push(c);
            }
            if pos.by == row_check as i32 {
                b_points.push(pos.bx);
            }
        }
    }
    rnk.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut left, mut right) = (rnk[0].0, rnk[0].1);
    let mut tmp = vec![];
    for (l, r) in rnk.iter().skip(1) {
        if *l > right {
            tmp.push((left, right));
            left = *l;
            right = *r;
        } else if *r > right {
            right = *r;
        }
    }
    tmp.push((left, right));
    let mut count = 0;
    b_points.dedup();
    for (l, r) in tmp.iter() {
        let sub = b_points.iter().filter(|f| *l <= **f && *r >= **f).count();
        count += *r - *l + 1 - sub as i32;
    }
    count
}

#[derive(Debug)]
struct Position {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
    distance: usize,
}

impl Position {
    fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Self {
        let x = sx.abs_diff(bx);
        let y = sy.abs_diff(by);
        Position {
            sx,
            sy,
            bx,
            by,
            distance: (x + y) as usize,
        }
    }

    fn collapse(&self, row_check: usize) -> Option<(i32, i32)> {
        let dist = row_check.abs_diff(self.sy as usize);
        if dist <= self.distance {
            let d = dist.abs_diff(self.distance) as i32;
            return Some((self.sx - d, self.sx + d));
        }
        None
    }
}
