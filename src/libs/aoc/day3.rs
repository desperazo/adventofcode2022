use std::collections::HashMap;

pub fn solve() -> u64 {
    let input = super::utils::read("./src/input/day3.txt");
    let mut ans = 0;
    for v in input {
        if v.len() == 0 {
            break;
        }
        let store: Vec<_> = v.chars().into_iter().collect();
        let tmp: Vec<_> = v.chars().into_iter().skip(v.len() / 2).collect();
        for i in 0..v.len() / 2 {
            let c = store[i];
            if tmp.contains(&c) {
                let v = c as u32;
                if v > 95 {
                    ans += v - 96;
                } else {
                    ans += v - 38;
                }
                break;
            }
        }
    }
    ans.into()
}
