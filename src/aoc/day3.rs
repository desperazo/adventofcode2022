pub fn solve() -> u64 {
    let input = super::utils::read("./src/input/day3.txt");
    let mut ans = 0;
    for v in input {
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

pub fn solve_2() -> u64 {
    let input = super::utils::read("./src/input/day3.txt");
    let mut ans = 0;
    let mut count = 0;
    let mut score: [i32; 54] = [0; 54];
    for v in input {
        let store: Vec<_> = v.chars().into_iter().collect();
        let mut dup = Vec::new();
        for a in store {
            if dup.contains(&a) {
                continue;
            }
            dup.push(a);
            let mut sc = a as u32;
            if sc > 95 {
                sc = sc - 96;
            } else {
                sc = sc - 38;
            }
            score[sc as usize] += 1;
            if score[sc as usize] == 3 {
                ans += sc;
                score = [0; 54];
                break;
            }
        }
        count += 1;
        if count % 3 == 0 {
            score = [0; 54];
        }
    }
    ans.into()
}
