use regex::Regex;

pub fn solve() -> String {
    let input = super::utils::read("./src/input/day5.txt");
    let mut create = vec![
        vec!["B", "Z", "T"],
        vec!["V", "H", "T", "D", "N"],
        vec!["B", "F", "M", "D"],
        vec!["T", "J", "G", "W", "V", "Q", "L"],
        vec!["W", "D", "G", "P", "V", "F", "Q", "M"],
        vec!["V", "Z", "Q", "G", "H", "F", "S"],
        vec!["Z", "S", "N", "R", "L", "T", "C", "W"],
        vec!["Z", "H", "W", "D", "J", "N", "R", "M"],
        vec!["M", "Q", "L", "F", "D", "S"],
    ];
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for text in input {
        let cap = re.captures(text.as_str()).unwrap();
        let mov: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = cap.get(3).unwrap().as_str().parse().unwrap();

        for _ in 0..mov {
            let f = create.get_mut(from - 1).unwrap();
            if let Some(v) = f.pop() {
                let t = create.get_mut(to - 1).unwrap();
                t.push(v);
            }
        }
    }
    let mut ans = String::from("");
    for v in create.iter_mut() {
        if let Some(v) = v.pop() {
            ans.push_str(v);
        }
    }
    ans
}

pub fn solve_2() -> String {
    let input = super::utils::read("./src/input/day5.txt");
    let mut create = vec![
        vec!["B", "Z", "T"],
        vec!["V", "H", "T", "D", "N"],
        vec!["B", "F", "M", "D"],
        vec!["T", "J", "G", "W", "V", "Q", "L"],
        vec!["W", "D", "G", "P", "V", "F", "Q", "M"],
        vec!["V", "Z", "Q", "G", "H", "F", "S"],
        vec!["Z", "S", "N", "R", "L", "T", "C", "W"],
        vec!["Z", "H", "W", "D", "J", "N", "R", "M"],
        vec!["M", "Q", "L", "F", "D", "S"],
    ];
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for text in input {
        let cap = re.captures(text.as_str()).unwrap();
        let mov: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = cap.get(3).unwrap().as_str().parse().unwrap();

        let mut tmp = Vec::new();
        let f = create.get_mut(from - 1).unwrap();
        for _ in 0..mov {
            if let Some(v) = f.pop() {
                tmp.push(v);
            }
        }
        tmp.reverse();
        let t = create.get_mut(to - 1).unwrap();
        for v in tmp.iter() {
            t.push(v);
        }
    }
    let mut ans = String::from("");
    for v in create.iter_mut() {
        if let Some(v) = v.pop() {
            ans.push_str(v);
        }
    }
    ans
}
