use std::str::FromStr;

pub fn solve() -> u64 {
    let input = super::utils::read("./src/input/day2.txt");
    let mut ans = 0;
    for v in input {
        let s: Vec<_> = v.split(' ').collect();
        let op = ShapeKind::from_str(s[0]).unwrap();
        let me = ShapeKind::from_str(s[1]).unwrap();
        ans += op.battle(me);
    }
    ans
}

pub fn solve_2() -> u64 {
    let input = super::utils::read("./src/input/day2.txt");
    let mut ans = 0;
    for v in input {
        let s: Vec<_> = v.split(' ').collect();
        let op = ShapeKind::from_str(s[0]).unwrap();
        if let Some(me) = op.suggest(s[1]) {
            ans += op.battle(me);
        }
    }
    ans
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShapeKind {
    Rock,
    Paper,
    Sciscors,
}

impl ShapeKind {
    pub fn battle(&self, vs: ShapeKind) -> u64 {
        if *self == vs {
            return 3 + vs.value();
        }
        if *self == ShapeKind::Rock && vs == ShapeKind::Paper {
            return 6 + vs.value();
        }
        if *self == ShapeKind::Paper && vs == ShapeKind::Sciscors {
            return 6 + vs.value();
        }
        if *self == ShapeKind::Sciscors && vs == ShapeKind::Rock {
            return 6 + vs.value();
        }
        vs.value()
    }

    pub fn suggest(&self, startegy: &str) -> Option<ShapeKind> {
        match startegy {
            "X" => match *self {
                ShapeKind::Rock => Some(ShapeKind::Sciscors),
                ShapeKind::Paper => Some(ShapeKind::Rock),
                ShapeKind::Sciscors => Some(ShapeKind::Paper),
            },
            "Y" => Some(*self),
            "Z" => match *self {
                ShapeKind::Rock => Some(ShapeKind::Paper),
                ShapeKind::Paper => Some(ShapeKind::Sciscors),
                ShapeKind::Sciscors => Some(ShapeKind::Rock),
            },
            _ => None,
        }
    }

    pub fn value(&self) -> u64 {
        match *self {
            ShapeKind::Rock => 1,
            ShapeKind::Paper => 2,
            ShapeKind::Sciscors => 3,
        }
    }
}

impl FromStr for ShapeKind {
    type Err = ();

    fn from_str(input: &str) -> Result<ShapeKind, Self::Err> {
        match input {
            "A" | "X" => Ok(ShapeKind::Rock),
            "B" | "Y" => Ok(ShapeKind::Paper),
            "C" | "Z" => Ok(ShapeKind::Sciscors),
            _ => Err(()),
        }
    }
}
