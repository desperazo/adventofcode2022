pub fn solve(size: usize) -> usize {
    let input = super::utils::read("./src/input/day6.txt");
    let chrs: Vec<_> = input[0].chars().collect();
    for i in 0..chrs.len() - size {
        let mut v: Vec<_> = chrs.iter().skip(i).take(size).collect();
        v.sort();
        v.dedup();
        if v.len() == size {
            return i + size;
        }
    }
    0
}
