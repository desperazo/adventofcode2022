use std::{cell::RefCell, collections::HashMap, rc::Rc};
static NODE_ROOT: &str = "root";
static NODE_HUMAN: &str = "humn";

pub fn solve() -> i64 {
    let mut node = parse();
    node.calculate();
    node.value.unwrap()
}

pub fn solve_2() -> i64 {
    let mut node = parse();
    node.calculate();
    let left_value = node.left.as_ref().unwrap().borrow().value.unwrap();
    let right_value = node.right.as_ref().unwrap().borrow().value.unwrap();
    let humn_value = node.node_value(NODE_HUMAN).unwrap();

    node.left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .calculate_yell(right_value);
    let new_humn_value = node.node_value(NODE_HUMAN).unwrap();
    if humn_value != new_humn_value {
        return new_humn_value;
    }

    node.right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .calculate_yell(left_value);
    node.node_value(NODE_HUMAN).unwrap()
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

    fn calculate_yell(&mut self, expected_value: i64) {
        if self.op == Operator::None {
            if self.name == NODE_HUMAN {
                self.value = Some(expected_value);
            }
            return;
        }
        let mut left = self.left.as_mut().unwrap().borrow_mut();
        let mut right = self.right.as_mut().unwrap().borrow_mut();
        let left_value = left.value.unwrap();
        let right_value = right.value.unwrap();
        match self.op {
            Operator::None => (),
            Operator::Plus => {
                left.calculate_yell(expected_value - right_value);
                right.calculate_yell(expected_value - left_value);
            }
            Operator::Minus => {
                left.calculate_yell(expected_value + right_value);
                right.calculate_yell(left_value - expected_value);
            }
            Operator::Multiply => {
                if right_value > 0 {
                    left.calculate_yell(expected_value / right_value);
                }
                if left_value > 0 {
                    right.calculate_yell(expected_value / left_value);
                }
            }
            Operator::Devide => {
                left.calculate_yell(expected_value * right_value);
                if expected_value > 0 {
                    right.calculate_yell(left_value / expected_value);
                }
            }
        }
    }

    fn node_value(&self, node: &str) -> Option<i64> {
        if self.name == node {
            return self.value;
        }
        if self.left.is_some() {
            if let Some(val) = self.left.as_ref().unwrap().borrow().node_value(node) {
                return Some(val);
            }
        }
        if self.right.is_some() {
            if let Some(val) = self.right.as_ref().unwrap().borrow().node_value(node) {
                return Some(val);
            }
        }
        None
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
    build_tree(NODE_ROOT, &hm)
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

#[derive(PartialEq)]
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
