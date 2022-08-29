//! <https://adventofcode.com/2018/day/17>
use {
    crate::{
        color,
        framework::{aoc, AdventOfCode, ParseError},
        regex,
    },
    std::collections::{HashMap, HashSet},
};

type Dim2 = (usize, usize);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Puzzle {
    line: Vec<(bool, usize, usize, usize)>,
    map: HashSet<Dim2>,
    water_map: HashSet<Dim2>,
}

#[aoc(2018, 17)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        // x=550, y=1443..1454
        let parser = regex!(r"^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)$");
        let segment = parser.captures(block).ok_or(ParseError)?;
        assert!(
            (&segment[1] == "x" && &segment[3] == "y")
                || (&segment[1] == "y" && &segment[3] == "x")
        );
        self.line.push((
            &segment[1] != "x",
            segment[2].parse::<usize>()?,
            segment[4].parse::<usize>()?,
            segment[5].parse::<usize>()?,
        ));
        Ok(())
    }
    fn after_insert(&mut self) {
        // dbg!(&self.line.len());
        for (horizontal, base, from, to) in self.line.iter() {
            for i in *from..=*to {
                let p = if *horizontal { (*base, i) } else { (i, *base) };
                self.map.insert(p);
            }
        }
        // dbg!(self.map.len());
    }
    fn part1(&mut self) -> Self::Output1 {
        let min_y = self.map.iter().map(|(y, _)| *y).min().unwrap();
        let min_x = self.map.iter().map(|(_, x)| *x).min().unwrap();
        let max_y = self.map.iter().map(|(y, _)| *y).max().unwrap();
        let max_x = self.map.iter().map(|(_, x)| *x).max().unwrap();
        let start = (0, 500);
        let mut water: HashMap<Dim2, Water> = HashMap::new();
        water.insert(start, Water::On);
        for p in self.map.iter() {
            water.insert(*p, Water::Block);
        }
        let mut to_update: Vec<Dim2> = vec![start];
        to_update.push((start.0 + 1, start.1));
        let focus = (30, 500);
        self.render(&focus, &water, false);
        let mut count = 0;
        // self.depth = 20;
        while let Some(pos) = to_update.pop() {
            if pos.0 == 0 || max_y < pos.0 {
                continue;
            }
            count += 1;
            // dbg!(pos.0);
            // if 80800 < count {
            //     break;
            // }
            let state = water.get(&pos).unwrap_or(&Water::None);
            let above = water.get(&(pos.0 - 1, pos.1)).unwrap_or(&Water::None);
            let left = water.get(&(pos.0, pos.1 - 1)).unwrap_or(&Water::None);
            let right = water.get(&(pos.0, pos.1 + 1)).unwrap_or(&Water::None);
            let below = water.get(&(pos.0 + 1, pos.1)).unwrap_or(&Water::None);
            if let Some(next) = transition(state, above, left, right, below) {
                water.insert(pos, next);
                let above = (pos.0 - 1, pos.1);
                let left = (pos.0, pos.1 - 1);
                let right = (pos.0, pos.1 + 1);
                let below = (pos.0 + 1, pos.1);
                to_update.push(above);
                to_update.push(left);
                to_update.push(right);
                to_update.push(below);
            }
            if count % 100 == 0 {
                self.render(
                    &(pos.0 / 20 * 20, (pos.1 + 30) / 20 * 20 - 30),
                    &water,
                    true,
                );
            }
            // if count == 800 {
            //     break;
            // }
        }
        self.render(&focus, &water, true);
        println!("({},{})-({},{})", min_y, min_x, max_y, max_x);
        water
            .iter()
            .filter(|(p, s)| {
                min_y <= p.0 && p.0 <= max_y && ![Water::None, Water::Block].contains(s)
            })
            .count()
    }
    fn part2(&mut self) -> Self::Output2 {
        let min_y = self.map.iter().map(|(y, _)| *y).min().unwrap();
        let min_x = self.map.iter().map(|(_, x)| *x).min().unwrap();
        let max_y = self.map.iter().map(|(y, _)| *y).max().unwrap();
        let max_x = self.map.iter().map(|(_, x)| *x).max().unwrap();
        let start = (0, 500);
        let mut water: HashMap<Dim2, Water> = HashMap::new();
        water.insert(start, Water::On);
        for p in self.map.iter() {
            water.insert(*p, Water::Block);
        }
        let mut to_update: Vec<Dim2> = vec![start];
        to_update.push((start.0 + 1, start.1));
        let focus = (30, 500);
        self.render(&focus, &water, false);
        let mut count = 0;
        while let Some(pos) = to_update.pop() {
            if pos.0 == 0 || max_y < pos.0 {
                continue;
            }
            count += 1;
            let state = water.get(&pos).unwrap_or(&Water::None);
            let above = water.get(&(pos.0 - 1, pos.1)).unwrap_or(&Water::None);
            let left = water.get(&(pos.0, pos.1 - 1)).unwrap_or(&Water::None);
            let right = water.get(&(pos.0, pos.1 + 1)).unwrap_or(&Water::None);
            let below = water.get(&(pos.0 + 1, pos.1)).unwrap_or(&Water::None);
            if let Some(next) = transition(state, above, left, right, below) {
                water.insert(pos, next);
                let above = (pos.0 - 1, pos.1);
                let left = (pos.0, pos.1 - 1);
                let right = (pos.0, pos.1 + 1);
                let below = (pos.0 + 1, pos.1);
                to_update.push(above);
                to_update.push(left);
                to_update.push(right);
                to_update.push(below);
            }
            if count % 100 == 0 {
                self.render(
                    &(pos.0 / 20 * 20, (pos.1 + 30) / 20 * 20 - 30),
                    &water,
                    true,
                );
            }
        }
        self.render(&focus, &water, true);
        println!("({},{})-({},{})", min_y, min_x, max_y, max_x);
        water
            .iter()
            .filter(|(p, s)| min_y <= p.0 && p.0 <= max_y && **s == Water::BothBound)
            .count()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Water {
    None,
    Drop,
    On,
    LeftBound,
    RightBound,
    BothBound,
    Block,
}

fn transition(
    state: &Water,
    above: &Water,
    left: &Water,
    right: &Water,
    below: &Water,
) -> Option<Water> {
    let dry = [Water::None, Water::Block];
    let solid = [Water::BothBound, Water::Block];
    let left_solid = [Water::LeftBound, Water::BothBound, Water::Block];
    let right_solid = [Water::RightBound, Water::BothBound, Water::Block];
    match (state, above, left, right, below) {
        (Water::Block, _, _, _, _) => None,
        // (Water::None, a, _, _, _) if !dry.contains(a) => Some(Water::On),
        (Water::None, _, Water::LeftBound, r, b) if solid.contains(r) && solid.contains(b) => {
            Some(Water::BothBound)
        }
        (Water::None, _, Water::On, r, b) if solid.contains(r) && solid.contains(b) => {
            Some(Water::RightBound)
        }
        (Water::None, _, Water::On, _, b) if solid.contains(b) => Some(Water::On),
        (Water::None, _, Water::On, _, Water::None) => Some(Water::Drop),
        (Water::None, _, Water::LeftBound, _, b) if solid.contains(b) => Some(Water::LeftBound),
        (Water::None, _, Water::LeftBound, _, Water::None) => Some(Water::Drop),

        (Water::None, _, l, Water::RightBound, b) if solid.contains(l) && solid.contains(b) => {
            Some(Water::BothBound)
        }
        (Water::None, _, l, Water::On, b) if solid.contains(l) && solid.contains(b) => {
            Some(Water::LeftBound)
        }
        (Water::None, _, _, Water::On, b) if solid.contains(b) => Some(Water::On),
        (Water::None, _, _, Water::On, Water::None) => Some(Water::Drop),
        (Water::None, _, _, Water::RightBound, b) if solid.contains(b) => Some(Water::RightBound),
        (Water::None, _, _, Water::RightBound, Water::None) => Some(Water::Drop),

        (Water::None, a, _, _, b) if !dry.contains(a) && solid.contains(b) => Some(Water::On),
        (Water::None, a, _, _, _) if !dry.contains(a) => Some(Water::Drop),

        (Water::Drop, _, l, r, _) if left_solid.contains(l) && right_solid.contains(r) => {
            Some(Water::BothBound)
        }
        (Water::Drop, _, _, _, b) if solid.contains(b) => Some(Water::On),

        (Water::On, _, l, r, _) if left_solid.contains(l) && right_solid.contains(r) => {
            Some(Water::BothBound)
        }
        (Water::On, _, l, _, _) if left_solid.contains(l) => Some(Water::LeftBound),
        (Water::On, _, _, r, _) if right_solid.contains(r) => Some(Water::RightBound),

        (Water::LeftBound, _, _, r, _) if right_solid.contains(r) => Some(Water::BothBound),

        (Water::RightBound, _, l, _, _) if left_solid.contains(l) => Some(Water::BothBound),
        _ => None,
    }
}

impl Puzzle {
    fn render(&self, center: &Dim2, water: &HashMap<Dim2, Water>, repaint: bool) {
        let height: isize = 50;
        if repaint {
            for _ in 0..=height {
                print!("{}", color::REVERT);
            }
        }
        println!(
            "-------------------------------------------- ({:>4}, {:>4})",
            center.0, center.1
        );
        for y in (center.0 as isize - height + 15)..(center.0 as isize + 15) {
            if y < 0 {
                for _ in -45_isize..45 {
                    print!(" ");
                }
                println!();
                continue;
            }
            for x in -45_isize..45 {
                let xx: usize = (center.1 as isize + x).max(0_isize) as usize;
                let w = water.get(&(y as usize, xx)).unwrap_or(&Water::None);
                match w {
                    Water::None => {
                        print!(" ");
                    }
                    Water::Block => {
                        print!("#");
                    }
                    Water::Drop => {
                        print!("{}|{}", color::BLUE, color::RESET);
                    }
                    Water::On => {
                        print!("{}~{}", color::BLUE, color::RESET);
                    }
                    Water::LeftBound => {
                        print!("{}>{}", color::BLUE, color::RESET);
                    }
                    Water::RightBound => {
                        print!("{}<{}", color::BLUE, color::RESET);
                    }
                    Water::BothBound => {
                        print!("{}={}", color::BLUE, color::RESET);
                    }
                }
            }
            println!();
        }
    }
}
