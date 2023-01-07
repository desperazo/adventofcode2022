use regex::Regex;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub fn solve() -> usize {
    let root = build_dir();
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut ans = 0;
    while queue.len() > 0 {
        let temp = queue.pop_front().unwrap();
        if temp.borrow().size <= 100000 {
            ans += temp.borrow().size;
        }
        for v in temp.borrow().children.iter() {
            queue.push_back(Rc::clone(&v));
        }
    }
    ans
}

pub fn solve_2() -> usize {
    let root = build_dir();
    let unused = 70000000 - root.borrow().size;
    let expected = 30000000 - unused;
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut sizes = vec![];
    while queue.len() > 0 {
        let temp = queue.pop_front().unwrap();
        for v in temp.borrow().children.iter() {
            queue.push_back(Rc::clone(&v));
            sizes.push(v.borrow().size);
        }
    }
    sizes.sort();
    let ans = sizes.iter().filter(|x| **x >= expected).take(1).nth(0);
    *ans.unwrap()
}

fn build_dir() -> Rc<RefCell<Folder>> {
    let input = super::utils::read("./src/input/day7.txt");
    let root = Rc::new(RefCell::new(Folder::new("/", None)));
    let mut current = Rc::clone(&root);
    let mut cmd = Command::Cd("/");
    for v in input.iter().skip(1) {
        match Command::new(v) {
            Some(c) => match c {
                Command::Cd(dir_name) => {
                    let temp = current.borrow().change_dir(dir_name);
                    if let Some(dir) = temp {
                        current = dir;
                    }
                }
                Command::Ls => cmd = c,
            },
            None => match cmd {
                Command::Ls => {
                    let par = Some(Rc::clone(&current));
                    current.borrow_mut().add(par, v);
                }
                _ => (),
            },
        }
    }
    root
}

#[derive(Debug)]
struct Folder {
    size: usize,
    name: String,
    children: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

impl Folder {
    pub fn new(s: &str, par: Option<Rc<RefCell<Folder>>>) -> Self {
        Folder {
            size: 0,
            name: s.to_owned(),
            children: vec![],
            parent: par,
        }
    }

    pub fn increase_size(&mut self, size: usize) {
        self.size += size;
        if self.parent.is_some() {
            self.parent
                .as_mut()
                .unwrap()
                .borrow_mut()
                .increase_size(size);
        }
    }

    pub fn add(&mut self, par: Option<Rc<RefCell<Folder>>>, arg: &str) {
        let dir_reg = Regex::new(r"dir (\w+)").unwrap();
        if let Some(fname) = dir_reg.captures(arg) {
            let dir_name = fname.get(1).unwrap().as_str();
            let new_dir = Rc::new(RefCell::new(Folder::new(dir_name, par)));
            self.children.push(Rc::clone(&new_dir));
            return;
        }
        let file_reg = Regex::new(r"(\d+) (.+)").unwrap();
        if let Some(fname) = file_reg.captures(arg) {
            let size: usize = fname.get(1).unwrap().as_str().parse().unwrap();
            self.increase_size(size);
        }
    }

    pub fn change_dir(&self, dir_name: &str) -> Option<Rc<RefCell<Folder>>> {
        if dir_name == ".." {
            if let Some(par) = self.parent.as_ref() {
                return Some(Rc::clone(par));
            }
            return None;
        }

        let mut temp: Option<Rc<RefCell<Folder>>> = None;
        for ch in self.children.iter() {
            if ch.borrow().name == dir_name {
                temp = Some(Rc::clone(&ch));
                break;
            }
        }
        Some(temp.unwrap())
    }
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
}

impl Command<'_> {
    pub fn new(s: &str) -> Option<Command> {
        if s == "$ ls" {
            return Some(Command::Ls);
        }
        let regex = Regex::new(r"\$ cd (.+)").unwrap();
        if let Some(m) = regex.captures(s) {
            return match m.get(1) {
                Some(c) => {
                    let fol = c.as_str();
                    Some(Command::Cd(fol))
                }
                _ => None,
            };
        }
        None
    }
}
