//! <https://adventofcode.com/2024/day/21>
#![allow(dead_code)]
#![allow(unused_variables)]
use {
    crate::framework::{aoc, AdventOfCode, ParseError},
    serde::Serialize,
    std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    },
    winnow::{
        ascii::newline,
        combinator::{repeat, separated},
        token::one_of,
        PResult, Parser,
    },
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
enum Kind1 {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
}

impl std::fmt::Display for Kind1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind1::K0 => '0',
                Kind1::K1 => '1',
                Kind1::K2 => '2',
                Kind1::K3 => '3',
                Kind1::K4 => '4',
                Kind1::K5 => '5',
                Kind1::K6 => '6',
                Kind1::K7 => '7',
                Kind1::K8 => '8',
                Kind1::K9 => '9',
                Kind1::KA => 'A',
            }
        )
    }
}

impl Kind1 {
    fn iterator() -> impl Iterator<Item = Kind1> {
        [
            Kind1::K0,
            Kind1::K1,
            Kind1::K2,
            Kind1::K3,
            Kind1::K4,
            Kind1::K5,
            Kind1::K6,
            Kind1::K7,
            Kind1::K8,
            Kind1::K6,
            Kind1::K7,
            Kind1::K9,
            Kind1::KA,
        ]
        .into_iter()
    }
}

const LV1LINK: [(Kind1, Kind1); 15] = [
    (Kind1::K0, Kind1::K2),
    (Kind1::K0, Kind1::KA),
    (Kind1::K1, Kind1::K2),
    (Kind1::K1, Kind1::K4),
    (Kind1::K2, Kind1::K3),
    (Kind1::K2, Kind1::K5),
    (Kind1::K3, Kind1::K6),
    (Kind1::K3, Kind1::KA),
    (Kind1::K4, Kind1::K5),
    (Kind1::K4, Kind1::K7),
    (Kind1::K5, Kind1::K6),
    (Kind1::K5, Kind1::K8),
    (Kind1::K6, Kind1::K9),
    (Kind1::K7, Kind1::K8),
    (Kind1::K8, Kind1::K9),
];

fn lv1_build_pathes(from: Kind1, to: Kind1) -> Vec<Vec<Kind1>> {
    if from == to {
        return vec![vec![to]];
    }
    let mut to_visit: Vec<Vec<Kind1>> = Vec::new();
    let mut pathes: Vec<Vec<Kind1>> = Vec::new();
    to_visit.push(vec![from]);
    while let Some(path) = to_visit.pop() {
        if path.last().unwrap() == &to {
            // path.remove(0);
            pathes.push(path);
            continue;
        }
        for (p, q) in LV1LINK.iter() {
            let r = if path.last().unwrap() == p && !path.contains(q) {
                q
            } else if path.last().unwrap() == q && !path.contains(p) {
                p
            } else {
                continue;
            };
            let mut path1 = path.clone();
            path1.push(*r);
            to_visit.push(path1);
        }
    }
    pathes.sort_unstable_by_key(|l| l.len());
    pathes
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
enum Kind2 {
    U,
    D,
    L,
    R,
    A,
}

impl std::fmt::Display for Kind2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind2::U => '^',
                Kind2::D => 'v',
                Kind2::L => '<',
                Kind2::R => '>',
                Kind2::A => 'A',
            }
        )
    }
}

fn to_string(v: &[Kind2]) -> String {
    v.iter().map(|k| format!("{k}")).collect::<String>()
}

impl Kind2 {
    fn iterator() -> impl Iterator<Item = Kind2> {
        [Kind2::U, Kind2::D, Kind2::L, Kind2::R, Kind2::A].into_iter()
    }
}

const LV2LINK: [(Kind2, Kind2); 5] = [
    (Kind2::U, Kind2::D),
    (Kind2::U, Kind2::A),
    (Kind2::D, Kind2::L),
    (Kind2::D, Kind2::R),
    (Kind2::R, Kind2::A),
];

fn lv2_build_pathes(from: Kind2, to: Kind2) -> Vec<Vec<Kind2>> {
    if from == to {
        return vec![vec![]];
    }
    let mut to_visit: Vec<Vec<Kind2>> = Vec::new();
    let mut pathes: Vec<Vec<Kind2>> = Vec::new();
    to_visit.push(vec![from]);
    while let Some(path) = to_visit.pop() {
        if path.last().unwrap() == &to {
            // path.remove(0);
            pathes.push(path);
            continue;
        }
        for (p, q) in LV2LINK.iter() {
            let r = if path.last().unwrap() == p && !path.contains(q) {
                q
            } else if path.last().unwrap() == q && !path.contains(p) {
                p
            } else {
                continue;
            };
            let mut path1 = path.clone();
            path1.push(*r);
            to_visit.push(path1);
        }
    }
    let len = pathes.iter().map(|l| l.len()).min().unwrap();
    pathes.retain(|l| l.len() == len);
    pathes
}

fn build_lv3_actions_to_push(
    lv3: Kind2,
    from: Kind2,
    to: Kind2,
) -> Vec<(Vec<Kind2>, Kind2, Kind2)> {
    if from == to {
        return vec![(vec![Kind2::A], Kind2::A, to)];
    }
    let mut pathes = lv2_build_pathes(from, to);
    let len = pathes.iter().map(|l| l.len()).min().unwrap();
    pathes.retain(|l| l.len() == len);
    pathes
        .iter()
        .map(|p| {
            let (a, b) = lv2_path_to_lv2_actions(p);
            (a, lv3, to)
        })
        .collect::<Vec<_>>()
    // let (a, b) = lv2_path_to_lv2_actions(&pathes[0]);
    // // dbg!(lv3, from, to, b);
    // (a, lv3, to)
}

const LV1TOLV2: [((Kind1, Kind1), Kind2); 30] = [
    ((Kind1::K0, Kind1::K2), Kind2::U),
    ((Kind1::K0, Kind1::KA), Kind2::R),
    ((Kind1::K1, Kind1::K2), Kind2::R),
    ((Kind1::K1, Kind1::K4), Kind2::U),
    ((Kind1::K2, Kind1::K3), Kind2::R),
    ((Kind1::K2, Kind1::K5), Kind2::U),
    ((Kind1::K3, Kind1::K6), Kind2::U),
    ((Kind1::K3, Kind1::KA), Kind2::D),
    ((Kind1::K4, Kind1::K5), Kind2::R),
    ((Kind1::K4, Kind1::K7), Kind2::U),
    ((Kind1::K5, Kind1::K6), Kind2::R),
    ((Kind1::K5, Kind1::K8), Kind2::U),
    ((Kind1::K6, Kind1::K9), Kind2::U),
    ((Kind1::K7, Kind1::K8), Kind2::R),
    ((Kind1::K8, Kind1::K9), Kind2::R),
    ((Kind1::K2, Kind1::K0), Kind2::D),
    ((Kind1::KA, Kind1::K0), Kind2::L),
    ((Kind1::K2, Kind1::K1), Kind2::L),
    ((Kind1::K4, Kind1::K1), Kind2::D),
    ((Kind1::K3, Kind1::K2), Kind2::L),
    ((Kind1::K5, Kind1::K2), Kind2::D),
    ((Kind1::K6, Kind1::K3), Kind2::D),
    ((Kind1::KA, Kind1::K3), Kind2::D),
    ((Kind1::K5, Kind1::K4), Kind2::L),
    ((Kind1::K7, Kind1::K4), Kind2::D),
    ((Kind1::K6, Kind1::K5), Kind2::L),
    ((Kind1::K8, Kind1::K5), Kind2::D),
    ((Kind1::K9, Kind1::K6), Kind2::D),
    ((Kind1::K8, Kind1::K7), Kind2::L),
    ((Kind1::K9, Kind1::K8), Kind2::L),
];

fn lv1_path_to_lv2_actions(path: &[Kind1]) -> Vec<Kind2> {
    let mut ret = path
        .windows(2)
        .map(|v| {
            let p = v[0];
            let q = v[1];
            LV1TOLV2.iter().find(|(key, _)| key == &(p, q)).unwrap().1
        })
        .collect::<Vec<_>>();
    ret.push(Kind2::A);
    ret
}

const LV2TOLV2: [((Kind2, Kind2), Kind2); 10] = [
    ((Kind2::U, Kind2::D), Kind2::D),
    ((Kind2::U, Kind2::A), Kind2::R),
    ((Kind2::D, Kind2::L), Kind2::L),
    ((Kind2::D, Kind2::R), Kind2::R),
    ((Kind2::R, Kind2::A), Kind2::U),
    ((Kind2::D, Kind2::U), Kind2::U),
    ((Kind2::A, Kind2::U), Kind2::L),
    ((Kind2::L, Kind2::D), Kind2::R),
    ((Kind2::R, Kind2::D), Kind2::L),
    ((Kind2::A, Kind2::R), Kind2::D),
];

fn lv2_path_to_lv2_actions(path: &[Kind2]) -> (Vec<Kind2>, Kind2) {
    let mut p = vec![Kind2::A];
    p.append(&mut path.to_vec());
    let mut ret = path
        .windows(2)
        .map(|v| {
            let p = v[0];
            let q = v[1];
            LV2TOLV2.iter().find(|(key, _)| key == &(p, q)).unwrap().1
        })
        .collect::<Vec<_>>();
    let last = *ret.last().unwrap();
    ret.push(Kind2::A);
    (ret, last)
}

#[allow(dead_code)]
fn lv1_to_lv2_pathes(from: Kind1, to: Kind1) -> Vec<Vec<Kind2>> {
    lv1_build_pathes(from, to)
        .iter()
        .map(|path| {
            let mut path1 = vec![from];
            path1.append(&mut path.clone());
            let mut ret = path1
                .windows(2)
                .map(|pair| {
                    let p = pair[0];
                    let q = pair[1];
                    LV1TOLV2.iter().find(|(key, _)| key == &(p, q)).unwrap().1
                })
                .collect::<Vec<_>>();
            ret.push(Kind2::A);
            ret
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Puzzle {
    line: Vec<Vec<Kind1>>,
    level1path: HashMap<(Kind1, Kind1), Vec<Vec<Kind1>>>,
    lv1_to_lv2: HashMap<(Kind1, Kind1), Vec<Vec<Kind2>>>,
    level2path: HashMap<(Kind2, Kind2), Vec<Vec<Kind2>>>,
    lv1: HashMap<(Kind1, Kind1), Kind2>,
    lv2: HashMap<(Kind2, Kind2), Kind2>,
}

fn parse_kind1(s: &mut &str) -> PResult<Kind1> {
    one_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'])
        .map(|c| match c {
            '0' => Kind1::K0,
            '1' => Kind1::K1,
            '2' => Kind1::K2,
            '3' => Kind1::K3,
            '4' => Kind1::K4,
            '5' => Kind1::K5,
            '6' => Kind1::K6,
            '7' => Kind1::K7,
            '8' => Kind1::K8,
            '9' => Kind1::K9,
            'A' => Kind1::KA,
            _ => unreachable!(),
        })
        .parse_next(s)
}

fn parse_line(s: &mut &str) -> PResult<Vec<Kind1>> {
    repeat(1.., parse_kind1).parse_next(s)
}

fn parse(s: &mut &str) -> PResult<Vec<Vec<Kind1>>> {
    separated(1.., parse_line, newline).parse_next(s)
}

#[aoc(2024, 21)]
impl AdventOfCode for Puzzle {
    fn parse(&mut self, input: String) -> Result<String, ParseError> {
        self.line = parse(&mut input.as_str())?;
        Self::parsed()
    }
    fn end_of_data(&mut self) {
        // dbg!(&self.line);
        /* self.level2path = Kind2::iterator()
            .flat_map(|from| {
                Kind2::iterator()
                    .map(|to| ((from, to), lv2_build_pathes(from, to)))
                    .collect::<HashMap<(Kind2, Kind2), Vec<Vec<Kind2>>>>()
            })
            .collect::<HashMap<_, _>>();
        self.level1path = Kind1::iterator()
            .flat_map(|from| {
                Kind1::iterator()
                    .filter(|to| &from != to)
                    .map(|to| ((from, to), lv1_build_pathes(from, to)))
                    .collect::<HashMap<(Kind1, Kind1), Vec<Vec<Kind1>>>>()
            })
            .collect::<HashMap<_, _>>(); */
        // dbg!(lv1_build_pathes(Kind1::K5, Kind1::K3));
        // dbg!(lv1_to_lv2_pathes(Kind1::K5, Kind1::K3));
        self.lv1 = [
            ((Kind1::K0, Kind1::K2), Kind2::U),
            ((Kind1::K0, Kind1::KA), Kind2::R),
            ((Kind1::K1, Kind1::K2), Kind2::R),
            ((Kind1::K1, Kind1::K4), Kind2::U),
            ((Kind1::K2, Kind1::K3), Kind2::R),
            ((Kind1::K2, Kind1::K5), Kind2::U),
            ((Kind1::K3, Kind1::K6), Kind2::U),
            ((Kind1::K3, Kind1::KA), Kind2::D),
            ((Kind1::K4, Kind1::K5), Kind2::R),
            ((Kind1::K4, Kind1::K7), Kind2::U),
            ((Kind1::K5, Kind1::K6), Kind2::R),
            ((Kind1::K5, Kind1::K8), Kind2::U),
            ((Kind1::K6, Kind1::K9), Kind2::U),
            ((Kind1::K7, Kind1::K8), Kind2::R),
            ((Kind1::K8, Kind1::K9), Kind2::R),
            // reverse
            ((Kind1::K2, Kind1::K0), Kind2::D),
            ((Kind1::KA, Kind1::K0), Kind2::L),
            ((Kind1::K2, Kind1::K1), Kind2::L),
            ((Kind1::K4, Kind1::K1), Kind2::D),
            ((Kind1::K3, Kind1::K2), Kind2::L),
            ((Kind1::K5, Kind1::K2), Kind2::D),
            ((Kind1::K6, Kind1::K3), Kind2::D),
            ((Kind1::KA, Kind1::K3), Kind2::U),
            ((Kind1::K5, Kind1::K4), Kind2::L),
            ((Kind1::K7, Kind1::K4), Kind2::D),
            ((Kind1::K6, Kind1::K5), Kind2::L),
            ((Kind1::K8, Kind1::K5), Kind2::D),
            ((Kind1::K9, Kind1::K6), Kind2::D),
            ((Kind1::K8, Kind1::K7), Kind2::L),
            ((Kind1::K9, Kind1::K8), Kind2::L),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        self.lv2 = [
            ((Kind2::U, Kind2::D), Kind2::D),
            ((Kind2::U, Kind2::A), Kind2::R),
            ((Kind2::D, Kind2::L), Kind2::L),
            ((Kind2::D, Kind2::R), Kind2::R),
            ((Kind2::R, Kind2::A), Kind2::U),
            // reverse
            ((Kind2::D, Kind2::U), Kind2::U),
            ((Kind2::A, Kind2::U), Kind2::L),
            ((Kind2::L, Kind2::D), Kind2::R),
            ((Kind2::R, Kind2::D), Kind2::L),
            ((Kind2::A, Kind2::R), Kind2::D),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
    }
    fn part1(&mut self) -> Self::Output1 {
        self.line
            .iter()
            .map(|p| {
                let v2 = lift::<Kind1, Kind2>(&p, &self.lv1, Kind1::KA, Kind2::A, 0);
                // for v in v2.iter() {
                //     println!("l2: {} ({})", to_string(v), v.len());
                // }
                let v3 = v2
                    .iter()
                    .flat_map(|p| lift::<Kind2, Kind2>(p, &self.lv2, Kind2::A, Kind2::A, 4))
                    .collect::<Vec<_>>();
                // for v in v3.iter() {
                //     println!("l3: {} ({})", to_string(v), v.len());
                // }
                let v4 = v3
                    .iter()
                    .flat_map(|p| lift::<Kind2, Kind2>(p, &self.lv2, Kind2::A, Kind2::A, 0))
                    .collect::<Vec<_>>();
                // for v in v4.iter() {
                //     println!("{}", to_string(v));
                // }
                let l = v4.iter().map(|l| l.len()).collect::<HashSet<_>>();
                println!("{:?}", l);
                v4.iter()
                    .map(|l| {
                        let a = l.len();
                        let b = p
                            .iter()
                            .filter(|k| **k != Kind1::KA)
                            .fold(0, |acc, k| acc * 10 + (*k as usize));
                        (a * b, a, b)
                    })
                    .min()
                    .unwrap_or((0, 0, 0))
            })
            .map(|s| dbg!(s).0)
            .sum::<usize>()
    }
    fn part2(&mut self) -> Self::Output2 {
        2
    }
}

fn lift_aux<T1, T2>(dict: &HashMap<(T1, T1), T2>, from: T1, to: T1, margin: usize) -> Vec<Vec<T2>>
where
    T1: Copy + Eq,
    T2: Copy + Eq,
{
    let mut to_visit: Vec<(Vec<T1>, Vec<T2>)> = Vec::new();
    to_visit.push((vec![from], Vec::new()));
    let mut _visited: HashSet<T1> = HashSet::new();
    let mut ret: Vec<Vec<T2>> = Vec::new();
    let mut len: usize = usize::MAX;
    while let Some((path, lifted_path)) = to_visit.pop() {
        if path.last() == Some(&to) {
            match lifted_path.len().cmp(&len) {
                Ordering::Less => {
                    len = lifted_path.len();
                    ret = vec![lifted_path];
                }
                Ordering::Equal => {
                    ret.push(lifted_path);
                }
                Ordering::Greater => {
                    if lifted_path.len() <= len + 1 {
                        ret.push(lifted_path);
                    }
                }
            }
            continue;
        }
        for (pair, by) in dict.iter() {
            if path.last() == Some(&pair.0) && !path.contains(&pair.1) {
                let mut path1 = path.clone();
                path1.push(pair.1);
                let mut lifted_path1 = lifted_path.clone();
                lifted_path1.push(*by);
                to_visit.push((path1, lifted_path1));
            }
        }
    }
    ret
}

fn lift<T1, T2>(
    path: &[T1],
    dict: &HashMap<(T1, T1), T2>,
    init: T1,
    post: T2,
    margin: usize,
) -> Vec<Vec<T2>>
where
    T1: Copy + std::fmt::Debug + Eq,
    T2: Copy + std::fmt::Debug + Eq,
{
    let mut p = vec![init];
    p.append(&mut path.to_vec());
    p.windows(2).fold(vec![Vec::new()], |acc, segment| {
        let cands = lift_aux(&dict, segment[0], segment[1], margin);
        acc.iter()
            .flat_map(|pre| {
                cands.iter().map(|c| {
                    let mut path = pre.clone();
                    let mut cc = c.clone();
                    path.append(&mut cc);
                    path.push(post);
                    path
                })
            })
            .collect::<Vec<_>>()
    })
}
