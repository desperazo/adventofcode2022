use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BinaryHeap, HashSet},
    hash::{Hash, Hasher},
};

use regex::Regex;
pub fn solve() -> u32 {
    let mut blueprints = parse();
    blueprints.iter_mut().map(|b| b.drill(24) * b.id).sum()
}

pub fn solve_2() -> u32 {
    let mut blueprints = parse();
    blueprints
        .iter_mut()
        .take(3)
        .map(|b| b.drill(32))
        .reduce(|acc, e| acc * e)
        .unwrap()
}

struct BluePrint {
    id: u32,
    max_geode: u32,
    costs: BTreeMap<Mine, BTreeMap<Mine, u32>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Plan {
    robots: [u32; 4],
    mines: [u32; 4],
    minutes: u32,
    max_minutes: u32,
}

impl Plan {
    fn build(&self, robot: &Mine, costs: &BTreeMap<Mine, BTreeMap<Mine, u32>>) -> Option<Plan> {
        let cost = costs.get(robot).unwrap();
        let take_minutes = cost
            .iter()
            .map(|(m, c)| {
                let robot_count = self.robots[*m as usize];
                let mine_count = self.mines[*m as usize];
                match robot_count {
                    0 => 99,
                    _ if mine_count >= *c => 0,
                    _ => ((c - mine_count - 1) / robot_count) + 1,
                }
            })
            .max()
            .unwrap();
        let new_minutes = self.minutes + take_minutes + 1;
        if new_minutes >= self.max_minutes {
            return None;
        }

        let mut new_mines = self.mines;
        new_mines.iter_mut().enumerate().for_each(|(m, n)| {
            if let Some(c) = cost.get(&m.try_into().unwrap()) {
                *n = *n + self.robots[m] * (take_minutes + 1) - c;
            } else {
                *n += self.robots[m] * (take_minutes + 1);
            }
        });
        let mut new_robots = self.robots;
        new_robots[*robot as usize] += 1;
        Some(Plan {
            robots: new_robots,
            mines: new_mines,
            minutes: new_minutes,
            max_minutes: self.max_minutes,
        })
    }

    fn havest(&mut self) -> u32 {
        let left = self.max_minutes - self.minutes;
        self.robots
            .iter()
            .enumerate()
            .for_each(|(i, v)| self.mines[i] += *v * left);
        self.minutes = self.max_minutes;
        self.mines[Mine::Geode as usize]
    }

    fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn max_future_geode(&self) -> u32 {
        let remain_times = self.max_minutes - self.minutes;
        let max_robot_geode = self.robots[Mine::Geode as usize] + remain_times;
        self.mines[Mine::Geode as usize] + (remain_times * max_robot_geode)
    }
}

impl Ord for Plan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_future_geode().cmp(&other.max_future_geode())
    }
}

impl PartialOrd for Plan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl BluePrint {
    fn build_plans(&mut self, plan: &mut Plan) -> Vec<Plan> {
        let mut res = vec![];
        if plan.minutes >= plan.max_minutes {
            return res;
        }
        for m in Mine::iterator() {
            if let Some(t) = plan.build(&m, &self.costs) {
                if t.max_future_geode() >= self.max_geode {
                    res.push(t);
                }
            }
        }
        self.max_geode = self.max_geode.max(plan.havest());
        res
    }

    fn drill(&mut self, max_minutes: u32) -> u32 {
        let mut queue = BinaryHeap::new();
        queue.push(Plan {
            minutes: 0,
            mines: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
            max_minutes,
        });
        let mut seen = HashSet::new();
        while let Some(mut p) = queue.pop() {
            let key = p.hash_value();
            if !seen.insert(key) || p.max_future_geode() < self.max_geode {
                continue;
            }
            let res = self.build_plans(&mut p);
            queue.extend(res);
        }
        self.max_geode
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Mine {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Mine {
    pub fn iterator() -> impl Iterator<Item = Mine> {
        [Mine::Ore, Mine::Clay, Mine::Obsidian, Mine::Geode]
            .iter()
            .copied()
    }
}

impl TryFrom<usize> for Mine {
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Mine::Ore),
            1 => Ok(Mine::Clay),
            2 => Ok(Mine::Obsidian),
            3 => Ok(Mine::Geode),
            _ => Err(()),
        }
    }

    type Error = ();
}

fn parse() -> Vec<BluePrint> {
    let input = super::utils::read("./src/input/day19.txt");
    let reg = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    let mut blueprints = vec![];
    for v in input.iter() {
        if let Some(cap) = reg.captures(v) {
            let costs = BTreeMap::from([
                (
                    Mine::Ore,
                    BTreeMap::from([(Mine::Ore, cap.get(2).unwrap().as_str().parse().unwrap())]),
                ),
                (
                    Mine::Clay,
                    BTreeMap::from([(Mine::Ore, cap.get(3).unwrap().as_str().parse().unwrap())]),
                ),
                (
                    Mine::Obsidian,
                    BTreeMap::from([
                        (Mine::Ore, cap.get(4).unwrap().as_str().parse().unwrap()),
                        (Mine::Clay, cap.get(5).unwrap().as_str().parse().unwrap()),
                    ]),
                ),
                (
                    Mine::Geode,
                    BTreeMap::from([
                        (Mine::Ore, cap.get(6).unwrap().as_str().parse().unwrap()),
                        (
                            Mine::Obsidian,
                            cap.get(7).unwrap().as_str().parse().unwrap(),
                        ),
                    ]),
                ),
            ]);
            let bp = BluePrint {
                id: cap.get(1).unwrap().as_str().parse().unwrap(),
                max_geode: 0,
                costs,
            };
            blueprints.push(bp);
        }
    }
    blueprints
}
