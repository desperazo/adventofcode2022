use std::collections::VecDeque;

pub fn solve() -> u128 {
    let mut monkeys = new_monkies();
    for i in 0..20 {
        for j in 0..monkeys.len() {
            let mut store = vec![];
            let mut m = monkeys.get_mut(j).unwrap();
            while let Some(v) = m.items.pop_front() {
                let worry = (m.operation)(v);
                let releif = worry / 3;
                let id = (m.test)(releif);
                store.push((id, releif));
                m.inspect_count += 1;
            }
            for (id, v) in store.iter() {
                m = monkeys.get_mut(*id).unwrap();
                m.items.push_back(*v);
            }
        }
    }
    top_inspector(monkeys)
}

pub fn solve_2() -> u128 {
    let mut monkeys = new_monkies();
    let common_diviser = 5 * 11 * 2 * 13 * 7 * 3 * 17 * 19;
    for i in 0..10_000 {
        for j in 0..monkeys.len() {
            let mut store = vec![];
            let mut m = monkeys.get_mut(j).unwrap();
            while let Some(v) = m.items.pop_front() {
                let worry = (m.operation)(v) % common_diviser;
                let id = (m.test)(worry);
                store.push((id, worry));
                m.inspect_count += 1;
            }
            for (id, v) in store.iter() {
                m = monkeys.get_mut(*id).unwrap();
                m.items.push_back(*v);
            }
        }
    }
    top_inspector(monkeys)
}

fn top_inspector(mut monkeys: Vec<Monkey>) -> u128 {
    monkeys.sort_by(|a, b| a.inspect_count.cmp(&b.inspect_count));
    let top: Vec<_> = monkeys.iter().rev().take(2).collect();
    top[0].inspect_count * top[1].inspect_count
}

fn new_monkies() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from(vec![65, 78]),
            operation: |x| x * 3,
            test: |x| match x % 5 == 0 {
                true => 2,
                false => 3,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![54, 78, 86, 79, 73, 64, 85, 88]),
            operation: |x| x + 8,
            test: |x| match x % 11 == 0 {
                true => 4,
                false => 7,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![69, 97, 77, 88, 87]),
            operation: |x| x + 2,
            test: |x| match x % 2 == 0 {
                true => 5,
                false => 3,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![99]),
            operation: |x| x + 4,
            test: |x| match x % 13 == 0 {
                true => 1,
                false => 5,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![60, 57, 52]),
            operation: |x| x * 19,
            test: |x| match x % 7 == 0 {
                true => 7,
                false => 6,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![91, 82, 85, 73, 84, 53]),
            operation: |x| x + 5,
            test: |x| match x % 3 == 0 {
                true => 4,
                false => 1,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![88, 74, 68, 56]),
            operation: |x| x * x,
            test: |x| match x % 17 == 0 {
                true => 0,
                false => 2,
            },
            inspect_count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![54, 82, 72, 71, 53, 99, 67]),
            operation: |x| x + 1,
            test: |x| match x % 19 == 0 {
                true => 6,
                false => 0,
            },
            inspect_count: 0,
        },
    ]
}

struct Monkey {
    items: VecDeque<usize>,
    operation: fn(usize) -> usize,
    test: fn(usize) -> usize,
    inspect_count: u128,
}
