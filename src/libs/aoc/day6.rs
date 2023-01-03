pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day6.txt");
    let chrs: Vec<_> = input[0].chars().collect();
    for i in 0..chrs.len() - 4 {
        let mut v: Vec<_> = chrs.iter().skip(i).take(4).collect();
        v.sort();
        v.dedup();
        if v.len() == 4 {
            return i + 4;
        }
    }
    0
}
