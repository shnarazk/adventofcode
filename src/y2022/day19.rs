//! <https://adventofcode.com/2022/day/19>
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        geometric::neighbors,
        line_parser, regex,
    },
    std::collections::{BinaryHeap, HashMap},
};

trait Calculation {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    // fn div(&self, other: &Self) -> Self;
    fn scale(&self, num: usize) -> Self;
}
impl Calculation for [usize; 4] {
    fn add(&self, other: &Self) -> Self {
        [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ]
    }
    fn sub(&self, other: &Self) -> Self {
        [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ]
    }
    // fn div(&self, other: &Self) -> Self {
    //     [
    //         (self[0] as f64 / other[0] as f64).round(),
    //         (self[1] as f64 / other[1] as f64).round(),
    //         (self[2] as f64 / other[2] as f64).round(),
    //         (self[3] as f64 / other[3] as f64).round(),
    //     ]
    // }
    fn scale(&self, num: usize) -> Self {
        [self[0] * num, self[1] * num, self[2] * num, self[3] * num]
    }
}

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Blueprint {
    id: usize,
    /// resource: [ore, clay, obsidian, geode]
    /// robot:[ore_robot, clay_robot, obsidian_robot, geode_robot]
    trans: [([usize; 4], [usize; 4]); 4],
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    time: usize,
    resources: [usize; 4],
    robots: [usize; 4],
}

impl Blueprint {
    fn profit(&self) -> usize {
        let mut num_geodes = 0;
        let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
        let init = State {
            time: 1,
            robots: [1, 0, 0, 0],
            resources: [1, 0, 0, 0],
        };
        to_visit.push(init);
        while let Some(state) = to_visit.pop() {
            if num_geodes < state.resources[3] {
                num_geodes = state.resources[3];
                dbg!(num_geodes);
            }
            if 0 < state.robots[3] {
                let total = state.resources[3] + (24 - state.time) * state.robots[3];
                if num_geodes < total {
                    num_geodes = total;
                    dbg!(num_geodes);
                }
            }
            for (requires, produces) in self.trans.iter() {
                // check if all the required stuffs can be generated
                if !requires
                    .iter()
                    .zip(state.robots.iter())
                    .all(|(req, bot)| *req == 0 || 0 < *bot)
                {
                    continue;
                }
                // dbg!("buildable");
                // So generate a robot after the required time is elapsed
                let mut next = state.clone();
                let time_to_wait: usize = requires
                    .iter()
                    .zip(state.resources.iter().zip(state.robots.iter()))
                    .filter_map(|(req, (res, bot))| {
                        req.checked_sub(*res).map(|t| {
                            if *bot == 0 {
                                t
                            } else if t % bot == 0 {
                                t / bot
                            } else {
                                t / bot + 1
                            }
                        })
                    })
                    .max()
                    .unwrap_or_default();
                next.time += time_to_wait;
                if 24 <= next.time {
                    continue;
                }
                next.resources = state
                    .robots
                    .scale(time_to_wait + 1)
                    .add(&next.resources)
                    .sub(requires);
                next.time += 1;
                next.robots = produces.add(&state.robots);
                // println!(
                //     "resource: {:?}, robots: {:?} at time {}",
                //     next.resources, next.robots, next.time
                // );
                to_visit.push(next);
            }
        }
        self.id * num_geodes
    }
}

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: Vec<Blueprint>,
}

#[aoc(2022, 19)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";

    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        let parser = regex!(
            r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."
        );
        if let Some(segment) = parser.captures(block) {
            macro_rules! nth {
                ($n: expr) => {
                    segment[$n].parse::<usize>()?
                };
            }
            self.line.push(Blueprint {
                id: segment[1].parse::<_>()?,
                trans: [
                    ([nth!(2), 0, 0, 0], [1, 0, 0, 0]),
                    ([nth!(3), 0, 0, 0], [0, 1, 0, 0]),
                    ([nth!(4), nth!(5), 0, 0], [0, 0, 1, 0]),
                    ([nth!(6), 0, nth!(7), 0], [0, 0, 0, 1]),
                ],
            });
        }
        Ok(())
    }
    fn after_insert(&mut self) {
        // dbg!(&self.line);
        dbg!(&self.line.len());
    }
    fn part1(&mut self) -> Self::Output1 {
        self.line.iter().map(|bp| bp.profit()).sum()
    }
    fn part2(&mut self) -> Self::Output2 {
        2
    }
}
