//! <https://adventofcode.com/2015/day/>
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        geometric::neighbors,
        parser, regex,
    },
    regex::Regex,
    std::collections::HashMap,
};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: Vec<()>,
}

#[aoc(2015, 0)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    // fn header(&mut self, input: String) -> Result<Option<String>, ParseError> {
    //     let parser: Regex = Regex::new(r"^(.+)\n\n((.|\n)+)$").expect("wrong");
    //     let segment = parser.captures(input).ok_or(ParseError)?;
    //     for num in segment[1].split(',') {
    //         let _value = num.parse::<usize>()?;
    //     }
    //     Ok(Some(segment[2].to_string()))
    // }
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        let parser = regex!(r"^([0-9]+)$");
        let segment = parser.captures(block).ok_or(ParseError)?;
        // self.line.push(segment[1].parse::<_>());
        Ok(())
    }
    fn end_of_data(&mut self) {
        dbg!(&self.line);
    }
    fn part1(&mut self) -> Self::Output1 {
        0
    }
    fn part2(&mut self) -> Self::Output2 {
        0
    }
}
