use std::collections::VecDeque;
use std::{collections::HashSet, fs::File, io::Write};

pub fn solve() -> usize {
    let input = super::utils::read("./src/input/day12.txt");
    let mut data = vec![];

    for v in input.iter() {
        data.push(v.chars().collect::<Vec<_>>());
    }
    let start = Node::new('S', 0, 20, 0);
    let mut queue = VecDeque::<Node>::new();
    queue.push_back(start);
    let mut visited = HashSet::with_capacity(7000);
    while let Some(node) = queue.pop_front() {
        if node.x == 138 && node.y == 20 {
            return node.depth + 1;
        }
        let mut nb = node.get_neighbors(&data);
        for n in nb.iter_mut() {
            if visited.insert((n.x, n.y)) {
                queue.push_back(*n);
            }
        }
    }
    panic!("route not found");
}

pub fn solve_2() -> usize {
    let input = super::utils::read("./src/input/day12.txt");
    let mut data = vec![];

    for v in input.iter() {
        data.push(v.chars().collect::<Vec<_>>());
    }

    let mut ans = usize::MAX;
    for (y, vy) in data.iter().enumerate() {
        for (x, vx) in vy.iter().enumerate() {
            if *vx != 'a' {
                continue;
            }
            let start = Node::new('S', x, y, 0);
            let mut queue = VecDeque::<Node>::new();
            queue.push_back(start);
            let mut visited = HashSet::with_capacity(7000);
            while let Some(node) = queue.pop_front() {
                if node.x == 138 && node.y == 20 {
                    if node.depth + 1 < ans {
                        ans = node.depth + 1;
                    }
                    break;
                }
                let mut nb = node.get_neighbors(&data);
                for n in nb.iter_mut() {
                    if visited.insert((n.x, n.y)) {
                        queue.push_back(*n);
                    }
                }
            }
        }
    }
    ans
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    c: char,
    depth: usize,
}

impl Node {
    fn new(c: char, x: usize, y: usize, depth: usize) -> Self {
        Node { c, x, y, depth }
    }

    fn get_neighbors(&self, data: &Vec<Vec<char>>) -> Vec<Node> {
        let edges: Vec<(i32, i32)> = vec![
            (self.x as i32 - 1, self.y as i32),
            (self.x as i32 + 1, self.y as i32),
            (self.x as i32, self.y as i32 - 1),
            (self.x as i32, self.y as i32 + 1),
        ];
        let max_row = data.len();
        let mut nb = vec![];
        for (x, y) in edges.iter() {
            if *x < 0 || *y < 0 || *x >= data[0].len() as i32 || *y >= max_row as i32 {
                continue;
            }
            let new_x = *x as usize;
            let new_y = *y as usize;
            let cv = match self.c {
                'S' => 'a' as i32,
                _ => self.c as i32,
            };
            let dif = (data[new_y][new_x] as i32) - cv;
            if dif < 0 || dif <= 1 {
                let new_node = Node::new(data[new_y][new_x], new_x, new_y, self.depth + 1);
                nb.push(new_node);
            }
        }
        nb
    }
}

fn log(node: &Node, file: &mut File, data: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>) {
    let mut count = 0;
    for (y, v) in data.iter().enumerate() {
        let mut line = String::new();
        for (x, v2) in v.iter().enumerate() {
            match visited.iter().any(|f| f.0 == x && f.1 == y) {
                true => {
                    if node.x == x && node.y == y {
                        line.push_str("@");
                    } else {
                        line.push_str("#");
                    }
                    count += 1;
                }
                false => line.push_str(&format!("{v2}")),
            }
        }
        file.write(line.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
    let end = format!(
        "visit: {count}-----x:{} y:{} c: {}-------------------\n",
        node.x, node.y, node.c
    );
    file.write(end.as_bytes()).unwrap();
}
