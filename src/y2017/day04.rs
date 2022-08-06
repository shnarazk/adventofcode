//! <https://adventofcode.com/2017/day/4>
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        geometric::neighbors,
        line_parser, regex,
    },
    std::collections::{HashMap, HashSet},
};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: Vec<Vec<u8>>,
}

#[aoc(2017, 4)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        self.line
            .push(block.chars().map(|c| c as u8).collect::<Vec<u8>>());
        Ok(())
    }
    fn part1(&mut self) -> Self::Output1 {
        self.line.iter().filter(|p| is_valid(p)).count()
    }
    fn part2(&mut self) -> Self::Output2 {
        self.line.iter().filter(|p| is_valid2(p)).count()
    }
}

fn is_valid(phrase: &[u8]) -> bool {
    let mut words: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut buffer: Vec<u8> = Vec::new();
    for c in phrase.iter() {
        if *c == b' ' {
            *words.entry(buffer).or_insert(0) += 1;
            buffer = Vec::new();
        } else {
            buffer.push(*c);
        }
    }
    if !buffer.is_empty() {
        *words.entry(buffer).or_insert(0) += 1;
    }
    *words.values().max().unwrap_or(&0) < 2
}

fn is_valid2(phrase: &[u8]) -> bool {
    let mut words: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut fingerprint: HashSet<[usize; 26]> = HashSet::new();
    let mut buffer: Vec<u8> = Vec::new();
    macro_rules! update {
        () => {{
            // make a fingerprint
            let mut vec: [usize; 26] = [0; 26];
            for c in buffer.iter() {
                vec[(*c - b'a') as usize] += 1;
            }
            if fingerprint.contains(&vec) {
                return false;
            }
            fingerprint.insert(vec);
            *words.entry(buffer).or_insert(0) += 1;
        }};
    }
    for c in phrase.iter() {
        if *c == b' ' {
            update!();
            buffer = Vec::new();
        } else {
            buffer.push(*c);
        }
    }
    if !buffer.is_empty() {
        update!();
    }
    *words.values().max().unwrap_or(&0) < 2
}
