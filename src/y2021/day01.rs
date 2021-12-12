use crate::framework::{aoc, AdventOfCode, Maybe};

#[derive(Debug, Default)]
pub struct Puzzle {
    line: Vec<usize>,
}

#[aoc(2021, 1)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Maybe<()> {
        self.line.push(block.parse::<usize>()?);
        Ok(())
    }
    fn part1(&mut self) -> usize {
        let mut last = self.line[0];
        let mut count = 0;
        for l in self.line.iter() {
            if last < *l {
                count += 1;
            }
            last = *l;
        }
        count
    }
    fn part2(&mut self) -> usize {
        fn average3(vec: &[usize]) -> usize {
            assert!(2 < vec.len());
            vec[0] + vec[1] + vec[2]
        }
        let mut last = average3(&self.line);
        let mut count = 0;
        for i in 0..self.line.len() - 2 {
            let new = average3(&self.line[i..]);
            if last < new {
                count += 1;
            }
            last = new;
        }
        count
    }
}
