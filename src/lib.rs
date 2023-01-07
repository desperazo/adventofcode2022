mod aoc;

pub fn solve_day1() -> u64 {
    aoc::day1::solve()
}

pub fn solve_day1_2() -> u64 {
    aoc::day1::solve_2()
}

pub fn solve_day2() -> u64 {
    aoc::day2::solve()
}

pub fn solve_day2_2() -> u64 {
    aoc::day2::solve_2()
}

pub fn solve_day3() -> u64 {
    aoc::day3::solve()
}

pub fn solve_day3_2() -> u64 {
    aoc::day3::solve_2()
}

pub fn solve_day4() -> u64 {
    aoc::day4::solve()
}

pub fn solve_day4_2() -> u64 {
    aoc::day4::solve_2()
}

pub fn solve_day5() -> String {
    aoc::day5::solve()
}

pub fn solve_day5_2() -> String {
    aoc::day5::solve_2()
}

pub fn solve_day6(size: usize) -> usize {
    aoc::day6::solve(size)
}

pub fn solve_day7() -> usize {
    aoc::day7::solve()
}

pub fn solve_day7_2() -> usize {
    aoc::day7::solve_2()
}

#[cfg(test)]
mod tests;
