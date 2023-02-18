use std::cmp;

pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day13.txt");
    let mut count = 0;
    let mut ans = 0;
    let mut score = 0;
    while count < input.len() {
        score += 1;
        let left = Package::new(input[count].clone());
        let right = Package::new(input[count + 1].clone());
        if compare(left, right) == -1 {
            ans += score;
        }
        count += 3;
    }
    ans
}

enum Signal {
    Int(i32),
    Mix(String),
}

struct Package {
    curr: usize,
    text: String,
}

fn compare(mut left: Package, mut right: Package) -> i32 {
    loop {
        let l = left.next();
        let r = right.next();
        let cmp = match (l, r) {
            (None, None) => return 0,
            (None, Some(_)) => return -1,
            (Some(_), None) => return 1,
            (Some(lv), Some(rv)) => match (lv, rv) {
                (Signal::Int(lv1), Signal::Int(rv1)) => {
                    if lv1 < rv1 {
                        return -1;
                    }
                    if lv1 > rv1 {
                        return 1;
                    }
                    0
                }
                (Signal::Int(lv1), Signal::Mix(rv1)) => {
                    compare(Package::new_int(lv1), Package::new(rv1))
                }
                (Signal::Mix(lv1), Signal::Int(rv1)) => {
                    compare(Package::new(lv1), Package::new_int(rv1))
                }
                (Signal::Mix(lv1), Signal::Mix(rv1)) => {
                    compare(Package::new(lv1), Package::new(rv1))
                }
            },
        };
        if cmp != 0 {
            return cmp;
        }
    }
}

impl Package {
    fn new(text: String) -> Self {
        Package { curr: 0, text }
    }
    fn new_int(val: i32) -> Self {
        let mut text = "[".to_string();
        text.push_str(&val.to_string());
        text.push_str("]");
        Package { curr: 0, text }
    }
}

impl Iterator for Package {
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bracket = 0;
        let chrs: Vec<_> = self.text.chars().collect();
        let mut txt = String::new();
        self.curr += 1;
        if self.curr >= self.text.len() {
            return None;
        }
        while self.curr < self.text.len() - 1 {
            let ch = chrs[self.curr];
            match ch {
                ',' if txt.len() == 0 => {
                    self.curr += 1;
                    continue;
                }
                ',' if bracket == 0 => break,
                '[' => {
                    bracket += 1;
                    txt.push(ch);
                }
                ']' => {
                    txt.push(ch);
                    bracket -= 1;
                    if bracket == 0 {
                        break;
                    }
                }
                ch => txt.push(ch),
            }
            self.curr += 1;
        }
        match txt.chars().nth(0) {
            None => Some(Signal::Int(-1)),
            Some('[') => Some(Signal::Mix(txt)),
            _ => Some(Signal::Int(match txt.parse() {
                Ok(v) => v,
                Err(_) => -1,
            })),
        }
    }
}
