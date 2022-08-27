//! <https://adventofcode.com/2018/day/15>
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
use {
    crate::framework::{aoc, AdventOfCode, ParseError},
    core::cmp::Reverse,
    std::collections::{BinaryHeap, HashMap, HashSet},
};

const HIT_POINT: usize = 200;
const ATTACK_POWER: usize = 3;

type Dim2 = (isize, isize);

/// direction vectors in reading order
const DIRS: [Dim2; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn mdist(a: &Dim2, b: &Dim2) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Creature {
    Elf(Dim2, usize),
    Goblin(Dim2, usize),
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position().partial_cmp(other.position())
    }
}

impl Ord for Creature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position().cmp(other.position())
    }
}

impl Creature {
    fn is_elf(&self) -> bool {
        matches!(self, Creature::Elf(_, _))
    }
    fn hit_point(&self) -> usize {
        match self {
            Creature::Elf(_, hp) => *hp,
            Creature::Goblin(_, hp) => *hp,
        }
    }
    fn decrement_hit_point(&mut self) -> bool {
        let p = match self {
            Creature::Elf(_, hp) => hp,
            Creature::Goblin(_, hp) => hp,
        };
        if let Some(np) = (*p).checked_sub(ATTACK_POWER) {
            *p = np;
        } else {
            *p = 0;
        }
        *p == 0
    }
    fn position(&self) -> &Dim2 {
        match self {
            Creature::Elf(p, _) => p,
            Creature::Goblin(p, _) => p,
        }
    }
    fn set_position(&mut self, to: &Dim2) {
        match self {
            Creature::Elf(p, _) => {
                p.0 = to.0;
                p.1 = to.1;
            }
            Creature::Goblin(p, _) => {
                p.0 = to.0;
                p.1 = to.1;
            }
        }
    }
    //
    // Eeometry
    //
    //
    // Moving
    //
    fn target_creatures<'a, 'b>(&'a self, world: &'b Puzzle) -> Vec<&'b Creature> {
        world
            .creatures
            .iter()
            .filter(|c| self.is_elf() != c.is_elf())
            .collect::<Vec<_>>()
    }
    fn all_ranges(&self, world: &Puzzle) -> Vec<Dim2> {
        let pos = self.position();
        let v = self.target_creatures(world);
        let mut valids: HashSet<Dim2> = HashSet::new();
        for c in v.iter() {
            let p = c.position();
            for adj in DIRS {
                let q = (p.0 + adj.0, p.1 + adj.1);
                if world.map.contains(&q) && world.creatures.iter().all(|c| *c.position() != q) {
                    valids.insert(q);
                }
            }
        }
        let mut p: Vec<Dim2> = valids.iter().copied().collect::<Vec<_>>();
        p.sort_by_key(|t| mdist(t, pos));
        // if self.is_elf() {
        //     dbg!(&p);
        // }
        p
    }
    fn best_moving_position(&self, world: &Puzzle) -> Option<Dim2> {
        let pos = self.position();
        let v = self.all_ranges(world);
        // {
        //     let mut h: HashMap<Dim2, char> = HashMap::new();
        //     for p in v.iter() {
        //         h.insert(*p, '?');
        //     }
        //     world.render(Some(h));
        // }
        let table = world.build_distance_table(*pos);
        let mut targets = Vec::new();
        let mut dist_min = usize::MAX;
        for p in v.iter() {
            if let Some(d) = table.get(p) {
                match d.cmp(&dist_min) {
                    std::cmp::Ordering::Equal => targets.push(p),
                    std::cmp::Ordering::Less => {
                        targets.clear();
                        targets.push(p);
                        dist_min = *d;
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        }
        (!targets.is_empty()).then(|| {
            targets.sort();
            *targets[0]
        })
    }
    fn is_in_a_range<'a, 'b>(&'a self, world: &'b Puzzle) -> Option<Vec<&'b Creature>> {
        let p = self.position();
        let enemies = self.target_creatures(world);
        let mut targets: HashSet<&Creature> = HashSet::new();
        for adj in DIRS {
            let q = (p.0 + adj.0, p.1 + adj.1);
            for e in enemies.iter() {
                if *e.position() == q {
                    targets.insert(*e);
                }
            }
        }
        (!targets.is_empty()).then(|| targets.iter().copied().collect::<Vec<_>>())
    }
    // return `true `if I attacked.
    fn attack(&mut self, world: &mut Puzzle) -> bool {
        let mut targets = Vec::new();
        if let Some(v) = self.is_in_a_range(world) {
            let mut hp = usize::MAX;
            for enemy in v.iter() {
                let h = enemy.hit_point();
                match h.cmp(&hp) {
                    std::cmp::Ordering::Less => {
                        targets.clear();
                        targets.push(enemy.position());
                        hp = h;
                    }
                    std::cmp::Ordering::Equal => {
                        targets.push(enemy.position());
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        }
        if targets.is_empty() {
            return false;
        }
        targets.sort();
        let target = *targets[0];
        if world.reduce_hp(&target) {
            world.kill_creature(&target);
        }
        true
    }
    fn turn(&mut self, world: &mut Puzzle) -> bool {
        if !world.exists(self.position()) {
            return false;
        }
        if self.attack(world) {
            return true;
        }
        let pos = self.position();
        let mut updating = false;
        if let Some(target) = self.best_moving_position(world) {
            let table = world.build_distance_table(target);
            // let h = table
            //     .iter()
            //     .filter(|(_, v)| **v < 10)
            //     .map(|(k, v)| (*k, (*v as u8 + b'0') as char))
            //     .collect::<HashMap<Dim2, _>>();
            // world.render(Some(h));
            let mut dist_so_far = usize::MAX;
            let mut r = *pos;
            for ad in DIRS.iter() {
                let q = (pos.0 + ad.0, pos.1 + ad.1);
                if let Some(d) = table.get(&q) {
                    if *d < dist_so_far {
                        dist_so_far = *d;
                        r = q;
                    }
                }
            }
            assert_ne!(r, *pos);
            // println!(" - creatue at {:?} moves to {:?}", pos, r);
            world.move_creature(pos, &r);
            self.set_position(&r);
            updating = true;
        }
        updating && self.attack(world)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Puzzle {
    line: Vec<Vec<u8>>,
    map: HashSet<Dim2>,
    creatures: Vec<Creature>,
    height: usize,
    width: usize,
}

impl Puzzle {
    fn render(&self, overlay: Option<HashMap<Dim2, char>>) {
        let mut map: HashMap<Dim2, char> = HashMap::new();
        let mut hps: HashMap<Dim2, usize> = HashMap::new();
        for c in self.creatures.iter() {
            map.insert(*c.position(), if c.is_elf() { 'E' } else { 'G' });
            hps.insert(*c.position(), c.hit_point());
        }
        if let Some(h) = overlay {
            for (k, v) in h.iter() {
                map.insert(*k, *v);
            }
        }
        for (j, l) in self.line.iter().enumerate() {
            let mut v: Vec<usize> = Vec::new();
            for (i, c) in l.iter().enumerate() {
                if let Some(hp) = hps.get(&(j as isize, i as isize)) {
                    v.push(*hp);
                }
                print!(
                    "{}",
                    map.get(&(j as isize, i as isize)).unwrap_or(&(*c as char))
                );
            }
            if v.is_empty() {
                println!();
            } else {
                println!(" {:?}", v);
            }
        }
    }
    fn build_distance_table(&self, from: Dim2) -> HashMap<Dim2, usize> {
        let mut to_visit: BinaryHeap<Reverse<(usize, Dim2)>> = BinaryHeap::new();
        let mut visited: HashMap<Dim2, usize> = HashMap::new();
        let creatures: HashSet<Dim2> = self
            .creatures
            .iter()
            .map(|c| *c.position())
            .collect::<HashSet<_>>();
        to_visit.push(Reverse((0, from)));
        while let Some(Reverse((dist, pos))) = to_visit.pop() {
            if visited.contains_key(&pos) {
                continue;
            }
            visited.insert(pos, dist);
            for d in DIRS.iter() {
                let x = (pos.0 + d.0, pos.1 + d.1);
                if self.map.contains(&x) && !visited.contains_key(&x) && creatures.get(&x).is_none()
                {
                    // dbg!(&x);
                    to_visit.push(Reverse((dist + 1, x)));
                }
            }
        }
        visited
    }
    fn exists(&mut self, at: &Dim2) -> bool {
        self.creatures.iter().any(|c| c.position() == at)
    }
    fn move_creature(&mut self, from: &Dim2, to: &Dim2) {
        for c in self.creatures.iter_mut() {
            if c.position() == from {
                c.set_position(to);
            }
        }
    }
    fn reduce_hp(&mut self, at: &Dim2) -> bool {
        for c in self.creatures.iter_mut() {
            if c.position() == at {
                return c.decrement_hit_point();
            }
        }
        unreachable!()
    }
    fn kill_creature(&mut self, at: &Dim2) {
        self.creatures.retain(|c| c.position() != at)
    }
}

#[aoc(2018, 15)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        self.line
            .push(block.chars().map(|c| c as u8).collect::<Vec<u8>>());
        Ok(())
    }
    fn after_insert(&mut self) {
        for (j, l) in self.line.iter_mut().enumerate() {
            for (i, c) in l.iter_mut().enumerate() {
                let pos = (j as isize, i as isize);
                if *c != b'#' {
                    self.map.insert(pos);
                }
                match *c {
                    b'E' => {
                        self.creatures.push(Creature::Elf(pos, HIT_POINT));
                        *c = b'.';
                    }
                    b'G' => {
                        self.creatures.push(Creature::Goblin(pos, HIT_POINT));
                        *c = b'.';
                    }
                    _ => (),
                }
            }
        }
        dbg!(self.creatures.len());
        self.height = self.line.len();
        self.width = self.line[0].len();
    }
    fn part1(&mut self) -> Self::Output1 {
        self.render(None);
        for turn in 0.. {
            self.creatures.sort();
            let mut creatures = self.creatures.clone();
            for c in creatures.iter_mut() {
                if !self.exists(c.position()) {
                    continue;
                }
                if c.target_creatures(self).is_empty() {
                    println!("On turn {}, ", turn + 1);
                    self.render(None);
                    assert!(self.creatures.iter().all(|c| 0 < c.hit_point()));
                    let hit_points = self.creatures.iter().map(|c| c.hit_point()).sum::<usize>();
                    dbg!(hit_points);
                    dbg!((turn - 1) * hit_points);
                    dbg!(turn * hit_points);
                    dbg!((turn + 1) * hit_points);
                    return turn * hit_points;
                }
                c.turn(self);
            }
            println!("turn {} completed.", turn + 1);
            self.render(None);
        }
        unreachable!()
    }
    fn part2(&mut self) -> Self::Output2 {
        0
    }
}
