use std::cmp::Ordering;

pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day13.txt");
    let mut count = 0;
    let mut ans = 0;
    let mut score = 0;
    while count < input.len() {
        score += 1;
        let left = Package::new(input[count].clone());
        let right = Package::new(input[count + 1].clone());
        if compare(left, right).is_lt() {
            ans += score;
        }
        count += 3;
    }
    ans
}

pub fn solve_2() -> usize {
    let tmp = super::utils::read("./src/input/day13.txt");
    let mut input: Vec<_> = tmp
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();
    input.append(&mut vec!["[[2]]".to_string(), "[[6]]".to_string()]);
    input.sort_by(|a, b| compare(Package::new(a.to_string()), Package::new(b.to_string())));
    let mut ans = 0;
    for i in 0..input.len() {
        if input[i] == "[[2]]" {
            ans = i + 1;
        }
        if input[i] == "[[6]]" {
            ans = ans * (i + 1);
            break;
        }
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

fn compare(mut left: Package, mut right: Package) -> Ordering {
    loop {
        let l = left.next();
        let r = right.next();
        let ord = match (l, r) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(lv), Some(rv)) => match (lv, rv) {
                (Signal::Int(lv1), Signal::Int(rv1)) => lv1.cmp(&rv1),
                (Signal::Int(lv1), Signal::Mix(rv1)) => {
                    compare(Package::from_int(lv1), Package::new(rv1))
                }
                (Signal::Mix(lv1), Signal::Int(rv1)) => {
                    compare(Package::new(lv1), Package::from_int(rv1))
                }
                (Signal::Mix(lv1), Signal::Mix(rv1)) => {
                    compare(Package::new(lv1), Package::new(rv1))
                }
            },
        };
        if ord.is_ne() {
            return ord;
        }
    }
}

impl Package {
    fn new(text: String) -> Self {
        Package { curr: 0, text }
    }
    fn from_int(val: i32) -> Self {
        let text = String::from_iter(vec!["[", &val.to_string(), "]"]);
        Package { curr: 0, text }
    }
}

impl Iterator for Package {
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        if self.curr >= self.text.len() {
            return None;
        }
        let mut bracket = 0;
        let chrs: Vec<_> = self.text.chars().collect();
        let mut txt = String::new();
        while self.curr < self.text.len() - 1 {
            let ch = chrs[self.curr];
            match ch {
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
            _ => Some(Signal::Int(txt.parse().unwrap())),
        }
    }
}
