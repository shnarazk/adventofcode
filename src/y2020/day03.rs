//! <https://adventofcode.com/2020/day/3>
use crate::{
    framework::{aoc, AdventOfCode, ParseError},
    line_parser,
};

#[derive(Debug, Default, PartialEq)]
struct Chars {
    char: Vec<char>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Puzzle {
    line: Vec<Chars>,
}

#[aoc(2020, 3)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        self.line.push(Chars {
            char: line_parser::to_chars(block)?,
        });
        Ok(())
    }
    fn part1(&mut self) -> usize {
        self.count_for_slope(1, 3)
    }
    fn part2(&mut self) -> usize {
        // println!("{}", self.count_for_slope(1, 1));
        // println!("{}", self.count_for_slope(1, 3));
        // println!("{}", self.count_for_slope(1, 5));
        // println!("{}", self.count_for_slope(1, 7));
        // println!("{}", self.count_for_slope(2, 1));
        self.count_for_slope(1, 1)
            * self.count_for_slope(1, 3)
            * self.count_for_slope(1, 5)
            * self.count_for_slope(1, 7)
            * self.count_for_slope(2, 1)
    }
}

impl Puzzle {
    fn count_for_slope(&self, row: usize, col: usize) -> usize {
        let mut r = row;
        let mut c = col;
        let mut occ = 0;
        while r < self.line.len() {
            occ += self.access(r, c);
            r += row;
            c += col;
        }
        occ
    }
    fn access(&self, row: usize, col: usize) -> usize {
        let line = &self.line[row].char;
        let c = col % line.len();
        (line[c] == '#') as usize
    }
}
