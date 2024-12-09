//! <https://adventofcode.com/2024/day/9>
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        geometric::neighbors,
        parser,
    },
    serde::Serialize,
    std::collections::HashMap,
    winnow::{
        ascii::newline,
        combinator::{repeat, repeat_till, separated, seq, terminated},
        token::one_of,
        PResult, Parser,
    },
};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Puzzle {
    line: Vec<usize>,
    file: Vec<usize>,
    free: Vec<usize>,
}

#[aoc(2024, 9)]
impl AdventOfCode for Puzzle {
    fn parse(&mut self, input: String) -> Result<String, ParseError> {
        let s = &mut input.as_str();
        self.line = parser::to_digits(input.as_str())?;
        Self::parsed()
    }
    fn end_of_data(&mut self) {
        for (i, x) in self.line.iter().enumerate() {
            if i % 2 == 0 {
                self.file.push(*x);
            } else {
                self.free.push(*x);
            }
        }
        // dbg!(&self.line);
        // dbg!(&self.free.len());
        // dbg!(&self.file.len());
    }
    fn part1(&mut self) -> Self::Output1 {
        let len = self.line.iter().cloned().sum::<usize>();
        let mut tmp: Vec<Option<u32>> = vec![None; len];
        let mut p: usize = 0;
        for (i, n) in self.line.iter().enumerate() {
            let v = (i % 2 == 0).then_some(i as u32 / 2);
            for q in p..p + n {
                tmp[q] = v;
            }
            p += n;
        }
        let mut left: usize = 0;
        let mut right: usize = tmp.len() - 1;
        'stop: while left < right {
            if tmp[left].is_none() {
                while tmp[right].is_none() {
                    if right <= left {
                        break 'stop;
                    }
                    right -= 1;
                }
                assert!(tmp[left].is_none());
                assert!(tmp[right].is_some());
                tmp.swap(left, right);
                assert!(tmp[left].is_some());
                assert!(tmp[right].is_none());
            }
            left += 1;
        }
        tmp.iter()
            .enumerate()
            .map(|(i, x)| x.map_or(0, |v| i * (v as usize)))
            .sum()
    }
    fn part2(&mut self) -> Self::Output2 {
        let len = self.line.iter().cloned().sum::<usize>();
        let mut tmp: Vec<Option<u32>> = vec![None; len];
        let mut end: Vec<usize> = vec![0; self.file.len()];
        let mut p: usize = 0;
        for (i, n) in self.line.iter().enumerate() {
            if i % 2 == 0 {
                end[i / 2] = p;
            }
            let v = (i % 2 == 0).then_some(i as u32 / 2);
            for q in p..p + n {
                tmp[q] = v;
            }
            p += n;
        }
        let after = tmp
            .iter()
            .map(|p| p.map_or('.', |x| (b'0' + (x as u8)) as char))
            .collect::<String>();
        // println!("{after}");
        'next_file: for (id, &ln) in self.file.iter().enumerate().rev() {
            let mut left: Option<usize> = None;
            let mut right = 0;
            while right < end[id] {
                if tmp[right].is_none() {
                    if left.is_none() {
                        left = Some(right);
                    }
                    if let Some(l) = left {
                        if ln == right - l + 1 {
                            for i in l..l + ln {
                                tmp[i] = Some(id as u32);
                            }
                            for i in l + ln..len {
                                if tmp[i] == Some(id as u32) {
                                    tmp[i] = None;
                                }
                            }
                            let after = tmp
                                .iter()
                                .map(|p| p.map_or('.', |x| (b'0' + (x as u8)) as char))
                                .collect::<String>();
                            // println!("{after}");
                            continue 'next_file;
                        }
                    }
                } else {
                    left = None;
                }
                right += 1;
            }
        }
        let after = tmp
            .iter()
            .map(|p| p.map_or('.', |x| (b'0' + (x as u8)) as char))
            .collect::<String>();
        // println!("{after}");
        tmp.iter()
            .enumerate()
            .map(|(i, x)| x.map_or(0, |v| i * (v as usize)))
            .sum()
    }
}
