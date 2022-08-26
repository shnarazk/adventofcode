//! <https://adventofcode.com/2018/day/14>
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        geometric::neighbors,
        line_parser, regex,
    },
    std::collections::HashMap,
};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: usize,
}

#[aoc(2018, 14)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, _block: &str) -> Result<(), ParseError> {
        Ok(())
    }
    fn after_insert(&mut self) {
        self.line = 635041;
    }
    fn part1(&mut self) -> Self::Output1 {
        // self.line = 2018;
        let mut v: Vec<usize> = vec![3, 7];
        let mut elf1: usize = 0;
        let mut elf2: usize = 1;
        let mut remain: Option<usize> = None;
        for i in 0.. {
            let sum = v[elf1] + v[elf2];
            if 9 < sum {
                let a1 = sum / 10;
                let a2 = sum % 10;
                v.push(a1);
                v.push(a2);
            } else {
                v.push(sum);
            }
            elf1 = (elf1 + v[elf1] + 1) % v.len();
            elf2 = (elf2 + v[elf2] + 1) % v.len();
            // println!("{:?} elf1 = {}, elf2 = {}", v, elf1, elf2);
            if let Some(n) = remain {
                if n == 1 {
                    println!("{:?}", &v[self.line..]);
                    return v[self.line..].iter().fold(0, |acc, n| acc * 10 + n);
                }
                remain = Some(n - 1);
            } else if self.line == v.len() {
                dbg!();
                remain = Some(10);
            } else if self.line == v.len() + 1 {
                dbg!();
                remain = Some(9);
            }
        }
        0
    }
    fn part2(&mut self) -> Self::Output2 {
        0
    }
}
