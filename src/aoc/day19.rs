use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap},
};

pub fn solve() -> usize {
    let mut blue = BluePrint {
        id: 1,
        costs: HashMap::from([
            (Mine::Ore, HashMap::from([(Mine::Ore, 4)])),
            (Mine::Clay, HashMap::from([(Mine::Ore, 2)])),
            (
                Mine::Obsidian,
                HashMap::from([(Mine::Ore, 3), (Mine::Clay, 14)]),
            ),
            (
                Mine::Geode,
                HashMap::from([(Mine::Ore, 2), (Mine::Obsidian, 7)]),
            ),
        ]),
        // costs: HashMap::from([
        //     (Mine::Ore, HashMap::from([(Mine::Ore, 2)])),
        //     (Mine::Clay, HashMap::from([(Mine::Ore, 3)])),
        //     (
        //         Mine::Obsidian,
        //         HashMap::from([(Mine::Ore, 3), (Mine::Clay, 8)]),
        //     ),
        //     (
        //         Mine::Geode,
        //         HashMap::from([(Mine::Ore, 3), (Mine::Obsidian, 12)]),
        //     ),
        // ]),
        plans: vec![Plan {
            minutes: 0,
            mines: BTreeMap::from([
                (Mine::Ore, 0),
                (Mine::Clay, 0),
                (Mine::Obsidian, 0),
                (Mine::Geode, 0),
            ]),
            robots: BTreeMap::from([
                (Mine::Ore, 1),
                (Mine::Clay, 0),
                (Mine::Obsidian, 0),
                (Mine::Geode, 0),
            ]),
            distance: BTreeMap::from([
                (Mine::Ore, 0),
                (Mine::Clay, 0),
                (Mine::Obsidian, 0),
                (Mine::Geode, 0),
            ]),
        }],
    };
    blue.crack()
    //let mut cache = HashMap::new();
    // for i in 0..24 {
    //     println!("------------------ start round {} --------------", i + 1);
    //     blue.drill(&mut cache);
    //     println!("------------------ end round {} --------------", i + 1);
    // }
    //blue.max_geode()
}

struct BluePrint {
    id: usize,
    costs: HashMap<Mine, HashMap<Mine, usize>>,
    plans: Vec<Plan>,
}

#[derive(Debug, Eq, PartialEq)]
struct Plan {
    mines: BTreeMap<Mine, usize>,
    robots: BTreeMap<Mine, usize>,
    distance: BTreeMap<Mine, usize>,
    minutes: usize,
}

impl Clone for Plan {
    fn clone(&self) -> Self {
        Self {
            mines: self.mines.clone(),
            robots: self.robots.clone(),
            minutes: self.minutes,
            distance: self.distance.clone(),
        }
    }
}

impl Plan {
    fn calculate_distance(&mut self, costs: &HashMap<Mine, HashMap<Mine, usize>>) {
        let next = self.next_robot(costs);
        // println!("DIS STA {:?}", self);
        for key in costs.keys() {
            if *key > next {
                *self.distance.entry(*key).or_insert(0) = 999;
                continue;
            }
            let mut d = 0;
            for (m, s) in &costs[key] {
                if self.mines[m] > *s {
                    continue;
                }
                let mut d1 = *s - self.mines[m];
                let robot = self.robots[m];
                if d1 > 0 && robot > 0 {
                    if d1 % robot != 0 {
                        d1 = d1 / robot + 1;
                    } else {
                        d1 /= robot;
                    }
                }
                d = std::cmp::max(d, self.minutes + d1);
                // println!(
                //     "DIS     {:?} {:?}:{:?} {s}-{}/{}={d1}",
                //     self, key, m, self.mines[m], robot
                // );
            }
            *self.distance.entry(*key).or_insert(0) = d;
        }
        // *self.distance.entry(Mine::Clay).or_insert(0) = 100 - self.robots[&Mine::Ore];
        // *self.distance.entry(Mine::Ore).or_insert(0) = 100 - self.robots[&Mine::Clay];
    }

    fn unique_key(&self) -> String {
        let mut v: Vec<_> = self
            .robots
            .iter()
            .map(|(m, s)| format!("{:?}:{s} ", m))
            .collect();
        v.sort();
        let robot_key = v.iter().fold(String::from("robot-"), |a, b| a + b);

        let mut v2: Vec<_> = self
            .mines
            .iter()
            .map(|(m, s)| format!("{:?}:{s} ", m))
            .collect();
        v2.sort();
        let mine_key = v2.iter().fold(String::from("mines-"), |a, b| a + b);
        format!("{}{}", robot_key, mine_key)
    }

    fn havest(&mut self) {
        //println!("HAVEST ROBOT {:?}", self.robots);
        self.mines.iter_mut().for_each(|(k, v)| {
            *v += self.robots.get(k).unwrap();
        });
        self.minutes += 1;
        //println!("HAVEST MINES {:?}", self.mines);
    }

    fn max_robot(&self) -> Option<Mine> {
        self.robots
            .iter()
            .filter(|(a, b)| **b > 0)
            .map(|(a, b)| *a)
            .max()
    }

    fn next_robot(&self, costs: &HashMap<Mine, HashMap<Mine, usize>>) -> Mine {
        match self.max_robot() {
            Some(x) => match x {
                Mine::Ore => Mine::Obsidian,
                Mine::Clay => Mine::Obsidian,
                Mine::Obsidian => Mine::Geode,
                Mine::Geode => Mine::Geode,
            },
            None => Mine::Obsidian,
        }
    }

    fn prev_robots(target: Mine) -> Vec<Mine> {
        match target {
            Mine::Ore => vec![],
            Mine::Clay => vec![],
            Mine::Obsidian => vec![Mine::Ore, Mine::Clay],
            Mine::Geode => vec![Mine::Ore, Mine::Obsidian],
        }
    }

    fn build_robot(&mut self, robot: &Mine, costs: &HashMap<Mine, HashMap<Mine, usize>>) {
        *self.robots.entry(*robot).or_insert(0) += 1;
        costs[robot]
            .iter()
            .for_each(|(m, s)| *self.mines.entry(*m).or_insert(0) -= s);
    }

    fn can_build_robot(&self, robot: &Mine, costs: &HashMap<Mine, HashMap<Mine, usize>>) -> bool {
        let c = costs.get(robot).unwrap();
        c.iter().all(|(v, x)| self.mines.get(v).unwrap() >= x)
    }

    fn has_robot(&self, robot: Mine) -> bool {
        self.robots.iter().any(|(a, b)| *a == robot && *b > 0)
    }

    fn cmp_robot(&self, other: &Plan) -> Ordering {
        if self.minutes != other.minutes {
            return Ordering::Equal;
        }
        if self.robots.iter().all(|(a, b)| *b == other.robots[a]) {
            for (m, x) in self.mines.iter() {
                if *x != other.mines[m] {
                    return x.cmp(&other.mines[m]);
                }
            }
        }
        Ordering::Equal
    }

    fn gen_plans(
        &self,
        target: Mine,
        costs: &HashMap<Mine, HashMap<Mine, usize>>,
        mem: &mut Vec<String>,
    ) -> Vec<Plan> {
        let mut vec = vec![];
        if self.minutes > 24 {
            return vec;
        }
        let mut st = self.clone();
        st.havest();
        let key = st.unique_key();
        if !mem.contains(&key) {
            st.calculate_distance(costs);
            vec.push(st);
            mem.push(key);
        }

        if self.can_build_robot(&target, costs) {
            let mut p = self.clone();
            //println!("BUI BEF {:?}", p);
            p.havest();
            p.build_robot(&target, costs);

            //println!("BUI AFT {:?}", p);
            let key = p.unique_key();
            if !mem.contains(&key) {
                p.calculate_distance(costs);
                vec.push(p);
                mem.push(key);
            }
        } else {
            for prev_tar in Plan::prev_robots(target) {
                let pt = self.clone();
                let v = pt.gen_plans(prev_tar, costs, mem);
                vec.extend(v);
            }
        }
        vec
    }
}

impl BluePrint {
    fn crack(&mut self) -> usize {
        let mut queue = BinaryHeap::new();
        queue.push(self.plans[0].clone());
        let mut mem = Vec::new();
        let mut max_crack = 0;
        while let Some(p) = queue.pop() {
            println!("----- start drill -----------");
            //queue.iter().for_each(|q| println!("QUEUE   {:?}", q));
            println!("CURRENT {:?}", p);
            let crack = p.mines[&Mine::Geode];
            if p.minutes == 24 && crack > 0 {
                return crack;
            }
            let next_robot = p.next_robot(&self.costs);
            let plans = p.gen_plans(next_robot, &self.costs, &mut mem);
            println!(
                "new plan count: {} queue size: {}",
                plans.len(),
                queue.len()
            );
            plans.iter().for_each(|p| {
                println!("NEW     {:?}", p);
                queue.push(p.clone());
            });
        }
        max_crack
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Mine {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> Ordering {
        for (m, s) in self.distance.iter().rev() {
            if *m == Mine::Geode {
                let cp = self.robots[m].cmp(&other.robots[m]);
                if cp != Ordering::Equal {
                    return cp;
                }
            }
            let cp = other.distance[m].cmp(s);
            if cp != Ordering::Equal {
                return cp;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
