use std::collections::{HashMap, VecDeque};

use regex::Regex;
const MAX_MINUTES: u32 = 24;

pub fn solve() -> u32 {
    let blueprints = parse();
    blueprints.iter().map(|b| b.drill()).sum()
}

struct BluePrint {
    id: u32,
    costs: HashMap<Mine, HashMap<Mine, u32>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Plan {
    robots: [u32; 4],
    mines: [u32; 4],
    minutes: u32,
}

impl Plan {
    fn build(&self, robot: &Mine, costs: &HashMap<Mine, HashMap<Mine, u32>>) -> Option<Plan> {
        let cost = costs.get(robot).unwrap();
        let wait_time = cost
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
        let new_minutes = self.minutes + wait_time + 1;
        if new_minutes > MAX_MINUTES {
            return None;
        }

        let mut new_mines = self.mines;
        new_mines.iter_mut().enumerate().for_each(|(m, n)| {
            if let Some(c) = cost.get(&m.try_into().unwrap()) {
                *n = *n + self.robots[m] * (wait_time + 1) - c;
            } else {
                *n += self.robots[m] * (wait_time + 1);
            }
        });
        let mut new_robots = self.robots;
        new_robots[*robot as usize] += 1;
        Some(Plan {
            robots: new_robots,
            mines: new_mines,
            minutes: new_minutes,
        })
    }

    fn havest(&mut self) {
        let minutes = MAX_MINUTES - self.minutes;
        self.robots
            .iter()
            .enumerate()
            .for_each(|(i, v)| self.mines[i] += *v * minutes);
        self.minutes = MAX_MINUTES;
    }
}

impl BluePrint {
    fn drill(&self) -> u32 {
        let mut plans = VecDeque::new();
        plans.push_back(Plan {
            minutes: 0,
            mines: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        });
        let mut max_geode = 0;

        while let Some(mut p) = plans.pop_front() {
            for m in Mine::iterator() {
                if let Some(t) = p.build(&m, &self.costs) {
                    max_geode = max_geode.max(t.mines[Mine::Geode as usize]);
                    if t.minutes < MAX_MINUTES {
                        plans.push_back(t);
                    }
                }
            }
            p.havest();
            max_geode = max_geode.max(p.mines[Mine::Geode as usize]);
        }
        max_geode * self.id
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
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
            let costs = HashMap::from([
                (
                    Mine::Ore,
                    HashMap::from([(Mine::Ore, cap.get(2).unwrap().as_str().parse().unwrap())]),
                ),
                (
                    Mine::Clay,
                    HashMap::from([(Mine::Ore, cap.get(3).unwrap().as_str().parse().unwrap())]),
                ),
                (
                    Mine::Obsidian,
                    HashMap::from([
                        (Mine::Ore, cap.get(4).unwrap().as_str().parse().unwrap()),
                        (Mine::Clay, cap.get(5).unwrap().as_str().parse().unwrap()),
                    ]),
                ),
                (
                    Mine::Geode,
                    HashMap::from([
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
                costs,
            };
            blueprints.push(bp);
        }
    }
    blueprints
}
