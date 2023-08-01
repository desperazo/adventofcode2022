use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn solve() -> i64 {
    let mut node = parse();
    node.calculate();
    node.value.unwrap()
}

impl Node {
    fn calculate(&mut self) {
        let mut lv = 0;
        let mut rv = 0;
        if self.left.is_some() {
            let mut l = self.left.as_mut().unwrap().borrow_mut();
            l.calculate();
            lv = l.value.unwrap();
        }
        if self.right.is_some() {
            let mut r = self.right.as_mut().unwrap().borrow_mut();
            r.calculate();
            rv = r.value.unwrap();
        }
        let v = match self.op {
            Operator::None => self.value,
            Operator::Plus => Some(lv + rv),
            Operator::Minus => Some(lv - rv),
            Operator::Multiply => Some(lv * rv),
            Operator::Devide => Some(lv / rv),
        };
        self.value = v;
    }
}

fn parse() -> Node {
    let list = super::utils::read("./src/input/day21.txt");
    let hm = list
        .iter()
        .map(|v| {
            let lv = v.split(':').map(|k| k.trim()).collect::<Vec<_>>();
            (lv[0].to_string(), lv[1].to_string())
        })
        .collect::<HashMap<String, String>>();
    build_tree("root", &hm)
}

fn build_tree(name: &str, map: &HashMap<String, String>) -> Node {
    let value = map.get(name).unwrap();
    let sp = value.split(' ').collect::<Vec<_>>();
    match sp.len() {
        1 => Node {
            name: name.to_string(),
            left: None,
            right: None,
            value: Some(sp[0].parse().unwrap()),
            op: Operator::None,
        },
        _ => Node {
            name: name.to_string(),
            left: Some(Rc::new(RefCell::new(build_tree(sp[0], map)))),
            right: Some(Rc::new(RefCell::new(build_tree(sp[2], map)))),
            value: None,
            op: Operator::new(sp[1]),
        },
    }
}

struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    value: Option<i64>,
    op: Operator,
}

enum Operator {
    None,
    Plus,
    Minus,
    Multiply,
    Devide,
}

impl Operator {
    fn new(op: &str) -> Self {
        match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Devide,
            _ => Operator::None,
        }
    }
}
