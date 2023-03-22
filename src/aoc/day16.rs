use core::hash::Hash;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hasher,
};

pub fn solve() -> i32 {
    let (valves, map) = parse_data();
    higest_release(
        &Valve {
            name: "AA".to_string(),
            rate: 0,
        },
        &valves,
        &map,
        30,
        &mut HashMap::new(),
    )
}

pub fn solve_2() -> i32 {
    let (mut valves, map) = parse_data();
    two_higest_release(&mut valves, &map, 26)
}

fn two_higest_release(
    valves: &mut Vec<Valve>,
    map: &HashMap<String, Vec<String>>,
    minutes: i32,
) -> i32 {
    let src = &Valve {
        name: "AA".to_string(),
        rate: 0,
    };

    let mut set = Vec::new();
    let mut tmp = Vec::new();
    build_set(&valves, &mut tmp, &mut set);
    let mut mem = HashMap::new();
    let mut max_release = 0;
    for v in set.iter_mut() {
        if v.len() <= 5 || v.len() > valves.len() - 5 {
            continue;
        }
        let right: Vec<_> = valves
            .iter()
            .filter(|x| !v.contains(x))
            .map(|x| x.clone())
            .collect();
        let c1 = higest_release(&src, &v, map, minutes, &mut mem);
        let c2 = higest_release(&src, &right, map, minutes, &mut mem);
        max_release = max_release.max(c1 + c2);
    }
    max_release
}

fn build_set(valves: &Vec<Valve>, tmp: &mut Vec<Valve>, store: &mut Vec<Vec<Valve>>) {
    if tmp.len() > 0 {
        store.push(tmp.to_vec());
    }
    let mut f: Vec<_> = valves.iter().skip(1).map(|x| x.clone()).collect();
    for v in valves.iter() {
        let mut temp2 = tmp.clone();
        temp2.push(v.clone());
        build_set(&f, &mut temp2, store);
        f = f.iter().skip(1).map(|x| x.clone()).collect();
    }
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
    valves.iter().for_each(|dest| {
        let d = distance(src, dest, map, mem);
        let t = minutes - d - 1;
        if t > 0 {
            let mut rel = t * dest.rate;
            let f: Vec<Valve> = valves
                .iter()
                .filter(|x| x.name != dest.name)
                .map(|x| x.clone())
                .collect();
            rel += higest_release(dest, &f, map, t, mem);
            max_release = std::cmp::max(max_release, rel);
        }
    });
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

fn parse_data() -> (Vec<Valve>, HashMap<String, Vec<String>>) {
    let input = super::utils::read("./src/input/day16.txt");
    let reg = Regex::new(
        r"^Valve (?P<valve>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<adj>.+)$",
    )
    .unwrap();

    let mut maps = HashMap::new();
    let mut valves = Vec::new();
    for v in input.iter() {
        if let Some(cap) = reg.captures(v) {
            let valve: String = cap["valve"].parse().unwrap();
            let adj: String = cap["adj"].parse().unwrap();
            let edges: Vec<_> = adj.split(',').map(|x| x.trim().to_string()).collect();
            maps.insert(valve, edges);
            let v = Valve {
                name: cap["valve"].parse().unwrap(),
                rate: cap["rate"].parse().unwrap(),
            };
            if v.rate == 0 {
                continue;
            }
            valves.push(v);
        }
    }
    (valves, maps)
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
