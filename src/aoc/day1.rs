use super::utils;

pub fn solve() -> u64 {
    let input = utils::read("./src/input/day1.txt");
    let mut ans = 0;
    let mut tmp = 0;
    for v in input {
        if let Ok(m) = v.parse::<u64>() {
            tmp += m;
        } else {
            ans = std::cmp::max(ans, tmp);
            tmp = 0;
        }
    }
    ans
}

pub fn solve_2() -> u64 {
    let input = utils::read("./src/input/day1.txt");
    let mut ans = Vec::new();
    let mut tmp = 0;
    for v in input {
        if let Ok(m) = v.parse::<u64>() {
            tmp += m;
        } else {
            ans.push(tmp);
            tmp = 0;
        }
    }
    ans.sort();
    ans.iter().skip(ans.len() - 3).sum()
}
