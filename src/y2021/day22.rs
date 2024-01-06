//! <https://adventofcode.com/2021/day/22>
use std::collections::HashSet;
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        line_parser, regex,
    },
    itertools::Itertools,
    std::collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Puzzle {
    line: Vec<(bool, isize, isize, isize, isize, isize, isize)>,
}

#[aoc(2021, 22)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";

    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        let parser = regex!(
            r"^(on|off) x=(-?[0-9]+)\.\.(-?[0-9]+),y=(-?[0-9]+)\.\.(-?[0-9]+),z=(-?[0-9]+)\.\.(-?[0-9]+)$"
        );
        let segment = parser.captures(block).ok_or(ParseError)?;
        self.line.push((
            &segment[1] == "on",
            line_parser::to_isize(&segment[2])?,
            line_parser::to_isize(&segment[3])?,
            line_parser::to_isize(&segment[4])?,
            line_parser::to_isize(&segment[5])?,
            line_parser::to_isize(&segment[6])?,
            line_parser::to_isize(&segment[7])?,
        ));
        Ok(())
    }
    fn part1(&mut self) -> Self::Output1 {
        let offset = |p: isize| (p + 50isize) as usize;
        let mut grid: Vec<Vec<Vec<bool>>> = Vec::new();
        for _ in 0..=101 {
            let mut v = Vec::new();
            for _ in 0..=101 {
                let mut w = Vec::new();
                w.resize(101, false);
                v.push(w);
            }
            grid.push(v);
        }
        for (to, x1, x2, y1, y2, z1, z2) in self.line.iter() {
            // print!("{}..{},{}..{},{}..{}", x1, x2, y1, y2, z1, z2);
            // println!(
            //     " - {:?},{:?},{:?}",
            //     50 < *z1 || *z2 < -50,
            //     50 < *y1 || *y2 < -50,
            //     50 < *x1 || *x2 < -50,
            // );
            debug_assert!(x1 <= x2 && y1 <= y2 && z1 <= z2);
            if 50 < *z1 || *z2 < -50 {
                continue;
            }
            for z in (*z1).max(-50)..=(*z2).min(50) {
                if 50 < *y1 || *y2 < -50 {
                    continue;
                }
                for y in (*y1).max(-50)..=(*y2).min(50) {
                    if 50 < *x1 || *x2 < -50 {
                        continue;
                    }
                    for x in (*x1).max(-50)..=(*x2).min(50) {
                        grid[offset(z)][offset(y)][offset(x)] = *to;
                    }
                }
            }
        }
        grid.iter()
            .map(|v| {
                v.iter()
                    .map(|w| w.iter().filter(|b| **b).count())
                    .sum::<usize>()
            })
            .sum()
    }
    fn part2(&mut self) -> Self::Output2 {
        let xs: Vec<isize> = self
            .line
            .iter()
            .flat_map(|(_, x1, x2, _, _, _, _)| vec![*x1, *x2 + 1])
            .collect::<HashSet<isize>>()
            .iter()
            .copied()
            .sorted()
            .collect::<Vec<isize>>();
        let ys: Vec<isize> = self
            .line
            .iter()
            .flat_map(|(_, _, _, y1, y2, _, _)| vec![*y1, *y2 + 1])
            .collect::<HashSet<isize>>()
            .iter()
            .copied()
            .sorted()
            .collect::<Vec<isize>>();
        let zs: Vec<isize> = self
            .line
            .iter()
            .flat_map(|(_, _, _, _, _, z1, z2)| vec![*z1, *z2 + 1])
            .collect::<HashSet<isize>>()
            .iter()
            .copied()
            .sorted()
            .collect::<Vec<isize>>();
        let to_index_x: HashMap<isize, usize> = xs
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<HashMap<isize, usize>>();
        let to_index_y: HashMap<isize, usize> = ys
            .iter()
            .enumerate()
            .map(|(i, y)| (*y, i))
            .collect::<HashMap<isize, usize>>();
        let to_index_z: HashMap<isize, usize> = zs
            .iter()
            .enumerate()
            .map(|(i, z)| (*z, i))
            .collect::<HashMap<isize, usize>>();
        let mut grid: Vec<Vec<Vec<bool>>> = zs
            .iter()
            .map(|_| {
                ys.iter()
                    .map(|_| xs.iter().map(|_| false).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for (to, ref x1, x2, y1, ref y2, ref z1, z2) in self.line.iter() {
            let i1 = *to_index_z.get(z1).unwrap();
            let i2 = *to_index_z.get(&(*z2 + 1)).unwrap();
            let j1 = *to_index_y.get(y1).unwrap();
            let j2 = *to_index_y.get(&(*y2 + 1)).unwrap();
            let k1 = *to_index_x.get(x1).unwrap();
            let k2 = *to_index_x.get(&(*x2 + 1)).unwrap();
            for i in i1..i2 {
                for j in j1..j2 {
                    for k in k1..k2 {
                        grid[i][j][k] = *to;
                    }
                }
            }
        }
        let mut sum = 0;
        for (i, c) in grid.iter().enumerate() {
            for (j, l) in c.iter().enumerate() {
                for (k, b) in l.iter().enumerate() {
                    if *b {
                        sum += (zs[i + 1] - zs[i]) * (ys[j + 1] - ys[j]) * (xs[k + 1] - xs[k]);
                    }
                }
            }
        }
        sum as usize
    }
}
