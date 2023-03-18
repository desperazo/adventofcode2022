use core::hash::Hash;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hasher,
};

pub fn solve() -> i32 {
    let input = super::utils::read("./src/input/day16.txt");
    let reg = Regex::new(
        r"^Valve (?P<valve>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<adj>.+)$",
    )
    .unwrap();

    let mut maps = HashMap::new();
    let mut valves = Vec::new();
    let mut current = 0;
    for (i, v) in input.iter().enumerate() {
        if let Some(cap) = reg.captures(v) {
            let valve: String = cap["valve"].parse().unwrap();
            let adj: String = cap["adj"].parse().unwrap();
            let edges: Vec<_> = adj.split(',').map(|x| x.trim().to_string()).collect();
            maps.insert(valve, edges);
            let v = Valve {
                name: cap["valve"].parse().unwrap(),
                rate: cap["rate"].parse().unwrap(),
            };
            if &v.name == "AA" {
                current = i;
            }
            valves.push(v);
        }
    }
    let src = valves.get(current).unwrap();
    let f: Vec<Valve> = valves
        .iter()
        .filter(|x| x.rate > 0)
        .map(|x| x.clone())
        .collect();
    higest_release(src, &f, &maps, 30, &mut HashMap::new())
}

fn higest_release(
    src: &Valve,
    valves: &Vec<Valve>,
    map: &HashMap<String, Vec<String>>,
    minutes: i32,
    mem: &mut HashMap<String, i32>,
) -> i32 {
    if minutes <= 0 || valves.len() == 0 {
        return 0;
    }
    let catch_key = catche_key(src, valves, minutes);
    if let Some(c) = mem.get(&catch_key) {
        return *c;
    }
    let mut max_release = 0;
    for dest in valves.iter() {
        let d = distance(src, dest, map, mem);
        let t = minutes - d - 1;
        if t <= 0 {
            continue;
        }
        let mut rel = t * dest.rate;
        let f: Vec<Valve> = valves
            .iter()
            .filter(|x| x.name != dest.name)
            .map(|x| x.clone())
            .collect();
        rel += higest_release(dest, &f, map, t, mem);
        max_release = std::cmp::max(max_release, rel);
    }
    mem.insert(catch_key, max_release);
    max_release
}

fn catche_key(src: &Valve, valves: &Vec<Valve>, minutes: i32) -> String {
    let mut path = format!("CATCHE-{}-MIN-{minutes}->", src.name);
    for v in valves.iter() {
        path.push_str(v.name.as_str());
        path.push_str("->");
    }
    path
}

fn distance(
    src: &Valve,
    dest: &Valve,
    map: &HashMap<String, Vec<String>>,
    mem: &mut HashMap<String, i32>,
) -> i32 {
    if dest.rate == 0 || src.name == dest.name {
        return 0;
    }
    let path = format!("{}->{}", src.name, dest.name);
    if let Some(v) = mem.get(&path) {
        return *v;
    }
    let mut seen = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((src.name.clone(), 0));
    while let Some((name, dist)) = queue.pop_front() {
        for v in map.get(&name).unwrap().iter() {
            if seen.contains(v) {
                continue;
            }
            if *v == dest.name {
                mem.insert(path, dist + 1);
                return dist + 1;
            }
            seen.push(v.to_string());
            queue.push_back((v.to_string(), dist + 1));
        }
    }
    mem.insert(path, 0);
    0
}

#[derive(Clone)]
struct Valve {
    name: String,
    rate: i32,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Valve {}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
