pub fn solve() -> String {
    let input = parse();
    let mut total: i128 = input
        .iter()
        .map(|v| {
            v.iter().enumerate().fold(0, |acc, (i, k)| {
                acc + 5_i128.pow(v.len() as u32 - i as u32 - 1) * *k as i128
            })
        })
        .sum();

    let mut digits = vec![];
    let mut carry = false;

    while total > 0 {
        let mut rem = total % 5;
        if carry {
            rem += 1;
        }
        let c = match rem {
            0 | 5 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("out of range"),
        };
        digits.push(c);
        carry = rem > 2;
        total /= 5;
    }

    if carry {
        digits.push('1');
    }
    digits.reverse();
    String::from_iter(digits)
}

fn parse() -> Vec<Vec<i32>> {
    let list = super::utils::read("./src/input/day25.txt");
    let res = list
        .iter()
        .map(|v| {
            v.trim()
                .chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("invalid chars"),
                })
                .collect()
        })
        .collect();
    res
}
