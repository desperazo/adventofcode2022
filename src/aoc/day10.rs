pub fn solve() -> i32 {
    let input = super::utils::read("./src/input/day10.txt");
    let mut ans = 0;
    let mut reg = 1;
    let mut cycle = 0;
    let signal_strange = |v: i32| -> bool { (v + 20) % 40 == 0 };
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

pub fn solve_2() -> Vec<String> {
    let input = super::utils::read("./src/input/day10.txt");
    let mut reg = 1;
    let mut cycle = 0;
    let mut pixels = vec![];
    let mut line = String::new();
    for v in input.iter() {
        let ins = Instruction::parse(v);
        match ins {
            Instruction::Addx(x) => {
                draw(&mut cycle, reg, &mut line, &mut pixels);
                draw(&mut cycle, reg, &mut line, &mut pixels);
                reg += x;
            }
            Instruction::Noop => {
                draw(&mut cycle, reg, &mut line, &mut pixels);
            }
        }
    }
    pixels
}

fn draw(cycle: &mut i32, reg: i32, line: &mut String, pixels: &mut Vec<String>) {
    *cycle += 1;
    line.push_str(get_pixel(*cycle, reg).as_str());
    if *cycle % 40 == 0 {
        println!("{line}");
        let tmp = line.clone();
        pixels.push(tmp);
        line.clear();
    }
}

fn get_pixel(cycle: i32, reg: i32) -> String {
    let pos = (cycle - 1) % 40;
    let tmp = reg - 1;
    if pos >= tmp && pos <= tmp + 2 {
        "#".to_string()
    } else {
        ".".to_string()
    }
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
