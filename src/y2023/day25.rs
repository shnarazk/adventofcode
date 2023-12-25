//! <https://adventofcode.com/2023/day/25>
use {
    crate::{
        framework::{aoc_at, AdventOfCode, ParseError},
        progress,
    },
    itertools::Itertools,
    std::collections::{HashMap, HashSet},
};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Puzzle {
    line: HashMap<String, Vec<String>>,
    link: Vec<(usize, usize)>,
    hash: HashMap<usize, Vec<(usize, usize)>>,
    names: HashSet<String>,
}

#[aoc_at(2023, 25)]
impl AdventOfCode for Puzzle {
    type Output1 = usize;
    type Output2 = String;
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        let b = block.split(": ").collect::<Vec<&str>>();
        let others = b[1].split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
        self.names.insert(b[0].to_string());
        for name in others.iter() {
            self.names.insert(name.to_string());
        }
        self.line.insert(b[0].to_string(), others);
        Ok(())
    }
    fn end_of_data(&mut self) {
        let names = self
            .names
            .iter()
            .sorted()
            .cloned()
            .enumerate()
            .map(|(n, s)| (s, n))
            .collect::<HashMap<String, usize>>();
        for (from, tos) in self.line.iter() {
            for to in tos.iter() {
                let f = *names.get(from).unwrap();
                let t = *names.get(to).unwrap();
                self.link.push((f.min(t), f.max(t)));
            }
        }
        for (i, (from, to)) in self.link.iter().enumerate() {
            self.hash.entry(*from).or_default().push((i, *to));
            self.hash.entry(*to).or_default().push((i, *from));
        }
        dbg!(self.names.len());
        dbg!(self.link.len());
    }
    fn part1(&mut self) -> Self::Output1 {
        let num_node = self.names.len();
        let num_edge = self.link.len();
        dbg!(self.hash.values().map(|v| v.len()).max().unwrap());
        dbg!(self.hash.values().filter(|v| v.len() == 3).count());
        dbg!(self.hash.values().filter(|v| v.len() == 4).count());
        dbg!(self.hash.values().filter(|v| v.len() == 5).count());
        dbg!(self.hash.values().filter(|v| v.len() == 6).count());
        dbg!(self.hash.values().filter(|v| v.len() == 7).count());
        dbg!(self.hash.values().filter(|v| v.len() == 8).count());
        dbg!(self.hash.values().filter(|v| v.len() == 9).count());
        dbg!(self.hash.values().filter(|v| v.len() == 10).count());
        let total: f64 = num_node.pow(2) as f64 / 2_f64;
        for i in 0..num_node {
            if self.hash.get(&i).map_or(true, |v| 5 > v.len()) {
                continue;
            }
            for j in i + 1..num_node {
                if self.hash.get(&j).map_or(true, |v| 5 > v.len()) {
                    continue;
                }
                progress!((i * num_node + j) as f64 / total);
                for k in j + 1..num_node {
                    if self.hash.get(&k).map_or(true, |v| 5 > v.len()) {
                        continue;
                    }
                    let v = self.node_connectivity(vec![i, j, k]);
                    if 1 < v.len() {
                        let cand_rule_set = [i, j, k]
                            .iter()
                            .flat_map(|i| self.hash.get(i).unwrap())
                            .collect::<HashSet<_>>();
                        let cand_rules = cand_rule_set
                            .iter()
                            .map(|(i, _)| *i)
                            .collect::<Vec<usize>>();
                        let n_cands = cand_rules.len();

                        for i in 0..n_cands {
                            for j in i + 1..n_cands {
                                progress!((i * num_edge + j));
                                for k in j + 1..n_cands {
                                    let v = self.edge_connectivity(&[
                                        cand_rules[i],
                                        cand_rules[j],
                                        cand_rules[k],
                                    ]);
                                    if v.len() == 2 {
                                        return v.iter().product();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unreachable!()
    }
    fn part2(&mut self) -> Self::Output2 {
        "Happy holiday!".to_string()
    }
}
impl Puzzle {
    fn node_connectivity(&self, forbidden: Vec<usize>) -> Vec<usize> {
        let len = self.names.len() - forbidden.len();
        let mut result: Vec<usize> = vec![];
        let mut used: HashSet<usize> = HashSet::new();
        let mut ng = 0;
        while result.iter().sum::<usize>() < len {
            let mut to_visit: Vec<usize> = Vec::new();
            let remain = (0..len).find(|x| !used.contains(x)).unwrap();
            to_visit.push(remain);
            while let Some(n) = to_visit.pop() {
                if used.contains(&n) {
                    continue;
                }
                used.insert(n);
                ng += 1;
                if let Some(v) = self.hash.get(&n) {
                    for (_, to) in v.iter() {
                        if forbidden.contains(to) {
                            continue;
                        }
                        if !used.contains(to) {
                            to_visit.push(*to);
                        }
                    }
                }
            }
            assert!(0 < ng);
            result.push(ng);
            if 2 < result.len() {
                return vec![];
            }
            ng = 0;
        }
        result
    }
    fn edge_connectivity(&self, forbidden: &[usize]) -> Vec<usize> {
        let len = self.names.len();
        let mut result: Vec<usize> = vec![];
        let mut used: HashSet<usize> = HashSet::new();
        let mut ng = 0;
        while result.iter().sum::<usize>() < len {
            let mut to_visit: Vec<usize> = Vec::new();
            let remain = (0..len).find(|x| !used.contains(x)).unwrap();
            to_visit.push(remain);
            while let Some(n) = to_visit.pop() {
                if used.contains(&n) {
                    continue;
                }
                used.insert(n);
                ng += 1;
                if let Some(v) = self.hash.get(&n) {
                    for (i, to) in v.iter() {
                        if forbidden.contains(i) {
                            continue;
                        }
                        if !used.contains(to) {
                            to_visit.push(*to);
                        }
                    }
                }
            }
            result.push(ng);
            if 2 < result.len() {
                return vec![];
            }
            ng = 0;
        }
        result
    }
}
