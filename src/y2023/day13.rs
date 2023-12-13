//! <https://adventofcode.com/2023/day/13>
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
use crate::framework::{aoc, AdventOfCode, ParseError};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: Vec<Vec<Vec<bool>>>,
}

#[aoc(2023, 13)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        self.line.push(
            block
                .trim()
                .split('\n')
                .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<_>>(),
        );
        Ok(())
    }
    fn part1(&mut self) -> Self::Output1 {
        self.line.iter().map(evaluate).sum::<usize>()
    }
    fn part2(&mut self) -> Self::Output2 {
        2
    }
}

fn evaluate(m: &Vec<Vec<bool>>) -> usize {
    let width = m[0].len();
    let height = m.len();
    // check horizontal mirror
    'next_line: for y in 0..height - 1 {
        if m[y] != m[y + 1] {
            continue 'next_line;
        }
        let mir = y;
        for offset in 1..(mir + 1).min(height - y - 1) {
            if m[mir - offset] != m[mir + 1 + offset] {
                continue 'next_line;
            }
        }
        return 100 * (mir + 1);
    }
    'next_column: for x in 0..width - 1 {
        for y in 0..height {
            if m[y][x] != m[y][x + 1] {
                continue 'next_column;
            }
        }
        let mir = x;
        for offset in 1..(mir + 1).min(width - x - 1) {
            for y in 0..height {
                if m[y][mir - offset] != m[y][mir + 1 + offset] {
                    continue 'next_column;
                }
            }
        }
        return mir + 1;
    }
    unreachable!()
}
