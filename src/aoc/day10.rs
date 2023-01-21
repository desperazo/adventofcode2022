pub fn solve() -> i32 {
    let input = super::utils::read("./src/input/day10.txt");
    let mut ans = 0;
    let mut reg = 1;
    let mut cycle = 0;
    let signal_strange = |v: i32| -> bool { v == 20 || (v - 20) % 40 == 0 };
    for v in input.iter() {
        let ins = Instruction::parse(v);
        match ins {
            Instruction::Addx(x) => {
                cycle += 1;
                if signal_strange(cycle) {
                    ans += reg * cycle;
                }
                cycle += 1;
                if signal_strange(cycle) {
                    ans += reg * cycle;
                }
                reg += x;
            }
            Instruction::Noop => {
                cycle += 1;
                if signal_strange(cycle) {
                    ans += reg * cycle;
                }
            }
        }
    }
    ans
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        match s {
            "noop" => Instruction::Noop,
            _ => {
                let v: i32 = s.split(" ").nth(1).unwrap().parse().unwrap();
                Instruction::Addx(v)
            }
        }
    }
}
