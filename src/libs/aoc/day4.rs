pub fn solve() -> u64 {
    let input = super::utils::read("./src/input/day4.txt");
    let mut ans = 0;
    for v in input {
        let g: Vec<_> = v.split(",").collect();
        let left: Vec<_> = g[0].split("-").collect();
        let right: Vec<_> = g[1].split("-").collect();
        let l1 = left[0].parse::<u64>().unwrap();
        let l2 = left[1].parse::<u64>().unwrap();
        let r1 = right[0].parse::<u64>().unwrap();
        let r2 = right[1].parse::<u64>().unwrap();

        if (l1 <= r1 && l2 >= r2) || (r1 <= l1 && r2 >= l2) {
            ans += 1;
        }
    }
    ans
}

pub fn solve_2() -> u64 {
    let input = super::utils::read("./src/input/day4.txt");
    let mut ans = 0;
    for v in input {
        let g: Vec<_> = v.split(",").collect();
        let left: Vec<_> = g[0].split("-").collect();
        let right: Vec<_> = g[1].split("-").collect();
        let l1 = left[0].parse::<u64>().unwrap();
        let l2 = left[1].parse::<u64>().unwrap();
        let r1 = right[0].parse::<u64>().unwrap();
        let r2 = right[1].parse::<u64>().unwrap();

        if (l1 <= r1 && l2 >= r2)
            || (r1 <= l1 && r2 >= l2)
            || (l2 >= r1 && l1 <= r2)
            || (l1 <= r2 && l2 >= r1)
            || (r2 >= l1 && r1 <= l2)
        {
            ans += 1;
        }
    }
    ans
}
