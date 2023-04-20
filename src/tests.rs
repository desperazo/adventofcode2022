use super::*;

#[test]
fn day7_test() {
    let result = aoc::day7::solve();
    assert_eq!(result, 1350966);
}

#[test]
fn day7_2_test() {
    let result = aoc::day7::solve_2();
    assert_eq!(result, 6296435);
}

#[test]
fn day8_test() {
    let result = aoc::day8::solve();
    assert_eq!(result, 1695);
}

#[test]
fn day8_2_test() {
    let result = aoc::day8::solve_2();
    assert_eq!(result, 287040);
}

#[test]
fn day9_test() {
    let result = aoc::day9::solve();
    println!("answer {}", result);
    assert_eq!(result, 6044);
}

#[test]
fn day9_2test() {
    let result = aoc::day92::solve();
    println!("answer {}", result);
    assert_eq!(result, 2384);
}

#[test]
fn day10_test() {
    let result = aoc::day10::solve();
    println!("answer {}", result);
    assert_eq!(result, 16020);
}

#[test]
fn day10_2test() {
    aoc::day10::solve_2();
    /*
    ####..##..####.#..#.####..##..#....###..
    #....#..#....#.#..#....#.#..#.#....#..#.
    ###..#......#..#..#...#..#..#.#....#..#.
    #....#.....#...#..#..#...####.#....###..
    #....#..#.#....#..#.#....#..#.#....#.#..
    ####..##..####..##..####.#..#.####.#..#.
    */
}

#[test]
fn day11_test() {
    let result = aoc::day11::solve();
    println!("answer {}", result);
    assert_eq!(result, 110264);
}

#[test]
fn day11_2test() {
    let result = aoc::day11::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 23612457316);
}

#[test]
fn day12_test() {
    let result = aoc::day12::solve();
    println!("answer {}", result);
    assert_eq!(result, 437);
}

#[test]
fn day12_2test() {
    let result = aoc::day12::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 430);
}

#[test]
fn day13_test() {
    let result = aoc::day13::solve();
    println!("answer {}", result);
    assert_eq!(result, 5760);
}

#[test]
fn day13_2_test() {
    let result = aoc::day13::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 26670);
}

#[test]
fn day14_test() {
    let result = aoc::day14::solve();
    println!("answer {}", result);
    assert_eq!(result, 964);
}

#[test]
fn day14_2_test() {
    let result = aoc::day14::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 32041);
}

#[test]
fn day15_test() {
    let result = aoc::day15::solve();
    println!("answer {}", result);
    assert_eq!(result, 5142231);
}

#[test]
fn day15_2_test() {
    let result = aoc::day15::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 10884459367718);
}

#[test]
fn day16_test() {
    let result = aoc::day16::solve();
    println!("answer {}", result);
    assert_eq!(result, 1767);
}

#[test]
fn day16_2_test() {
    let result = aoc::day16::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 2528);
}

#[test]
fn day17_test() {
    let result = aoc::day17::solve(2022);
    println!("answer {}", result);
    assert_eq!(result, 3153);
}

#[test]
fn day17_2_test() {
    let result = aoc::day17::solve(1000000000000);
    println!("answer {}", result);
    assert_eq!(result, 1553665689155);
}

#[test]
fn day18_test() {
    let result = aoc::day18::solve();
    println!("answer {}", result);
    assert_eq!(result, 4628);
}

#[test]
fn day18_2_test() {
    let result = aoc::day18::solve_2();
    println!("answer {}", result);
    assert_eq!(result, 2582);
}

#[test]
fn day19_test() {
    let result = aoc::day19::solve();
    println!("answer {}", result);
    assert_eq!(result, 4628);
}
