pub fn solve() -> i16 {
    let arr = parse();
    let mut tmp = arr.clone();
    let mut ans = 0;
    for (i, distance) in arr.iter() {
        let pos = tmp.iter().position(|(k, _v)| k == i).unwrap();
        let m = tmp.remove(pos);

        let walk = ((pos as i16) + *distance);
        let new_pos = ((walk % tmp.len() as i16) + tmp.len() as i16) as usize % tmp.len();
        tmp.insert(new_pos, m);
    }
    let zero_pos = tmp.iter().position(|(_k, v)| *v == 0).unwrap();
    ans += tmp[(zero_pos + 1000) % tmp.len()].1;
    ans += tmp[(zero_pos + 2000) % tmp.len()].1;
    ans += tmp[(zero_pos + 3000) % tmp.len()].1;
    ans
}

fn parse() -> Vec<(usize, i16)> {
    super::utils::read("./src/input/day20.txt")
        .iter()
        .enumerate()
        .map(|(k, v)| (k, v.parse().unwrap()))
        .collect()
}
