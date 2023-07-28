pub fn solve() -> i64 {
    let arr = parse(1);
    let mut tmp = arr.clone();
    for (i, distance) in arr.iter() {
        shuffle(&mut tmp, i, distance);
    }
    calcuate(tmp)
}

pub fn solve_2() -> i64 {
    let arr = parse(811_589_153);
    let mut tmp = arr.clone();
    for _ in 0..10 {
        for (i, distance) in arr.iter() {
            shuffle(&mut tmp, i, distance);
        }
    }
    calcuate(tmp)
}

fn calcuate(tmp: Vec<(usize, i64)>) -> i64 {
    let mut ans = 0;
    let zero_pos = tmp.iter().position(|(_k, v)| *v == 0).unwrap();
    ans += tmp[(zero_pos + 1000) % tmp.len()].1;
    ans += tmp[(zero_pos + 2000) % tmp.len()].1;
    ans += tmp[(zero_pos + 3000) % tmp.len()].1;
    ans
}

fn shuffle(tmp: &mut Vec<(usize, i64)>, i: &usize, distance: &i64) {
    let pos = tmp.iter().position(|(k, _v)| k == i).unwrap();
    let m = tmp.remove(pos);

    let walk = pos as i64 + distance;
    let new_pos = ((walk % tmp.len() as i64) + tmp.len() as i64) as usize % tmp.len();
    tmp.insert(new_pos, m);
}

fn parse(m: i64) -> Vec<(usize, i64)> {
    super::utils::read("./src/input/day20.txt")
        .iter()
        .enumerate()
        .map(|(k, v)| (k, v.parse::<i64>().unwrap() * m))
        .collect()
}
