//! <https://adventofcode.com/2018/day/21>
use {
    crate::{
        framework::{aoc, AdventOfCode, ParseError},
        regex,
    },
    std::collections::{HashMap, HashSet},
};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Puzzle {
    line: Vec<Inst>,
    pc_index: usize,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Inst {
    Addr(usize, usize, usize),
    Addi(usize, usize, usize),
    Mulr(usize, usize, usize),
    Muli(usize, usize, usize),
    Banr(usize, usize, usize),
    Bani(usize, usize, usize),
    Borr(usize, usize, usize),
    Bori(usize, usize, usize),
    Setr(usize, usize, usize),
    Seti(usize, usize, usize),
    Gtir(usize, usize, usize),
    Gtri(usize, usize, usize),
    Gtrr(usize, usize, usize),
    Eqir(usize, usize, usize),
    Eqri(usize, usize, usize),
    Eqrr(usize, usize, usize),
}

impl TryFrom<&str> for Inst {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parser = regex!(r"^(\w{4}) (\d+) (\d+) (\d+)$");
        if let Some(segment) = parser.captures(value) {
            let opr1 = segment[2].parse::<usize>()?;
            let opr2 = segment[3].parse::<usize>()?;
            let opr3 = segment[4].parse::<usize>()?;
            match &segment[1] {
                "addr" => Ok(Inst::Addr(opr1, opr2, opr3)),
                "addi" => Ok(Inst::Addi(opr1, opr2, opr3)),
                "mulr" => Ok(Inst::Mulr(opr1, opr2, opr3)),
                "muli" => Ok(Inst::Muli(opr1, opr2, opr3)),
                "banr" => Ok(Inst::Banr(opr1, opr2, opr3)),
                "bani" => Ok(Inst::Bani(opr1, opr2, opr3)),
                "bori" => Ok(Inst::Bori(opr1, opr2, opr3)),
                "borr" => Ok(Inst::Borr(opr1, opr2, opr3)),
                "setr" => Ok(Inst::Setr(opr1, opr2, opr3)),
                "seti" => Ok(Inst::Seti(opr1, opr2, opr3)),
                "gtir" => Ok(Inst::Gtir(opr1, opr2, opr3)),
                "gtri" => Ok(Inst::Gtri(opr1, opr2, opr3)),
                "gtrr" => Ok(Inst::Gtrr(opr1, opr2, opr3)),
                "eqir" => Ok(Inst::Eqir(opr1, opr2, opr3)),
                "eqri" => Ok(Inst::Eqri(opr1, opr2, opr3)),
                "eqrr" => Ok(Inst::Eqrr(opr1, opr2, opr3)),
                _ => Err(ParseError),
            }
        } else {
            Err(ParseError)
        }
    }
}

impl Inst {
    #[allow(dead_code)]
    fn disassemble(&self, addr: usize, pc_index: usize) -> String {
        let l = |n: &usize| match *n {
            _ if *n == pc_index => "pc",
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            _ => unreachable!(),
        };
        let r = |n: &usize| match *n {
            _ if *n == pc_index => format!("{addr}"),
            0 => "a".to_string(),
            1 => "b".to_string(),
            2 => "c".to_string(),
            3 => "d".to_string(),
            4 => "e".to_string(),
            5 => "f".to_string(),
            _ => unreachable!(),
        };
        match self {
            Inst::Addr(o1, o2, o3) => format!("{} = {} + {};", l(o3), r(o1), r(o2)),
            Inst::Addi(o1, o2, o3) => format!("{} = {} + {};", l(o3), r(o1), o2),
            Inst::Mulr(o1, o2, o3) => format!("{} = {} * {};", l(o3), r(o1), r(o2)),
            Inst::Muli(o1, o2, o3) => format!("{} = {} * {};", l(o3), r(o1), o2),
            Inst::Banr(o1, o2, o3) => format!("{} = {} & {};", l(o3), r(o1), r(o2)),
            Inst::Bani(o1, o2, o3) => format!("{} = {} & {};", l(o3), r(o1), o2),
            Inst::Borr(o1, o2, o3) => format!("{} = {} | {};", l(o3), r(o1), r(o2)),
            Inst::Bori(o1, o2, o3) => format!("{} = {} | {};", l(o3), r(o1), o2),
            Inst::Setr(o1, _, o3) => format!("{} = {};", l(o3), r(o1)),
            Inst::Seti(o1, _, o3) => format!("{} = {};", l(o3), o1),
            Inst::Gtir(o1, o2, o3) => format!("{} = ({} > {}) as usize;", l(o3), o1, r(o2)),
            Inst::Gtri(o1, o2, o3) => format!("{} = ({} > {}) as usize;", l(o3), r(o1), o2),
            Inst::Gtrr(o1, o2, o3) => format!("{} = ({} > {}) as usize;", l(o3), r(o1), r(o2)),
            Inst::Eqir(o1, o2, o3) => format!("{} = ({} == {}) as usize;", l(o3), o1, r(o2)),
            Inst::Eqri(o1, o2, o3) => format!("{} = ({} == {}) as usize;", l(o3), r(o1), o2),
            Inst::Eqrr(o1, o2, o3) => format!("{} = ({} == {}) as usize;", l(o3), r(o1), r(o2)),
        }
    }
}

#[aoc(2018, 21)]
impl AdventOfCode for Puzzle {
    const DELIMITER: &'static str = "\n";
    fn insert(&mut self, block: &str) -> Result<(), ParseError> {
        let parser = regex!(r"^#ip (\d)");
        if let Some(segment) = parser.captures(block) {
            self.pc_index = segment[1].parse::<usize>()?;
        } else {
            self.line.push(Inst::try_from(block)?);
        }
        Ok(())
    }
    fn part1(&mut self) -> Self::Output1 {
        // for (i, c) in self.line.iter().enumerate() {
        //     println!("{:>3}: {}", i, c.disassemble(i, self.pc_index));
        // }
        part1()
    }
    fn part2(&mut self) -> Self::Output2 {
        part2()
    }
}

/*
  0: b = 123;
  1: b = b & 456;
  2: b = (b == 72) as usize;
  3: pc = b + 3;
  4: pc = 0;
  5: b = 0;
  6: c = b | 65536;
  7: b = 6663054;
  8: e = c & 255;
  9: b = b + e;
 10: b = b & 16777215;
 11: b = b * 65899;
 12: b = b & 16777215;
 13: e = (256 > c) as usize;
 14: pc = e + 14;
 15: pc = 15 + 1;
 16: pc = 27;
 17: e = 0;
 18: d = e + 1;
 19: d = d * 256;
 20: d = (d > c) as usize;
 21: pc = d + 21;
 22: pc = 22 + 1;
 23: pc = 25;
 24: e = e + 1;
 25: pc = 17;
 26: c = e;
 27: pc = 7;
 28: e = (b == a) as usize;
 29: pc = e + 29;
 30: pc = 5;
*/

/*
    0: b = 123;
=>  1: b = 123 & 456;
    2: b = (b == 72) as usize;
    3: pc = b + 3;
=>  4: pc = 0;
=>  5: b = 0;
=>  6: c = b | 65536;
    7: b = 6663054;
=>  8: e = c & 255;
    9: b = b + e;
   10: b = b & 16777215;
   11: b = b * 65899;
   12: b = b & 16777215;
   13: e = (256 > c) as usize;
   14: pc = e + 14;
=> 15: pc = 16;
=> 16: pc = 27;
=> 17: e = 0;
=> 18: d = e + 1;
   19: d = d * 256;
   20: d = (d > c) as usize;
   21: pc = d + 21;
=> 22: pc = 23;
=> 23: pc = 25;
=> 24: e += 1;
=> 25: pc = 17;
   26: c = e;
   27: pc = 7;
   28: e = (b == a) as usize;
   29: pc = e + 29;
=> 30: pc = 5;
=> 31: halt;
*/

/*
    0: b = 123;
=>  1: b = 123 & 456;
    2: b = (b == 72) as usize;
    3: pc = b + 3;
t>  4: pc = 0;
e>  5: b = 0;
    loop {
        =>  6: c = b | 65536;
            7: b = 6663054;
        =>  8: e = c & 255;
            9: b = b + e;
           10: b = b & 16777215;
           11: b = b * 65899;
           12: b = b & 16777215;
           13: e = (256 > c) as usize;
           14: pc = e + 14;
        t> 15: pc = 16;
        e> 16: pc = 27;
        => 17: e = 0;
        => 18: d = e + 1;
           19: d = d * 256;
           20: d = (d > c) as usize;
           21: pc = d + 21;
        t> 22: pc = 23;
        e> 23: pc = 25;
        => 24: e += 1;
           25: pc = 17;
        => 26: c = e;
           27: pc = 7;
        => if b == a {
               halt;
           }
           e = 0;
    }
*/

/*
    0: b = 123;
=>  1: b = 123 & 456;
    if b != 72 {
        goto 1;
    }
    b = 0;
    loop {
            6: c = b | 65536;
            7: b = 6663054;
        =>  8: e = c & 255;
            9: b = b + e;
           10: b = b & 16777215;
           11: b = b * 65899;
           12: b = b & 16777215;
           13: e = (256 > c) as usize;
           14: pc = e + 14;
        t> 15: pc = 16;
        e> 16: pc = 27;
        => 17: e = 0;
        => 18: d = e + 1;
           19: d = d * 256;
            if d <= c {
                d = 0;
                e += 1;
                goto 18;
            }
            d = 1;
           26: c = e;
           27: pc = 7;
        => if b == a {
               halt;
           }
           e = 0;
    }
*/

/*
    b = 0;
    'outer: loop {
        c = b | 65536;
        b = 6663054;
        loop {
               e = c & 255;
               b = b + e;
               b = b & 16777215;
               b = b * 65899;
               b = b & 16777215;
               if 256 > c {
                   if b == a {
                       halt;
                   }
                   e = 0;
                   continue 'outer;
               }
               e = 0;
               loop {
                   d = e + 1;
                   d *= 256;
                   if d > c {
                       break;
                   }
                   e += 1;
               }
               d = 1;
               c = e;
        }
    }
*/

fn part1() -> usize {
    let mut b: usize = 0;
    let mut c = b | 65536;
    b = 6663054;
    loop {
        let mut e = c & 255;
        b += e;
        b &= 16777215;
        b *= 65899;
        b &= 16777215;
        if 256 > c {
            return b;
        }
        e = 0;
        loop {
            if 256 * (e + 1) > c {
                break;
            }
            e += 1;
        }
        c = e;
    }
}

fn part2() -> usize {
    let mut found = 0;
    let mut record_status: HashSet<(usize, usize)> = HashSet::new();
    let mut record: HashMap<usize, usize> = HashMap::new();
    let mut b = 0;
    'outer: loop {
        let mut c = b | 65536;
        b = 6663054;
        loop {
            let mut e = c & 255;
            b += e;
            b &= 16777215;
            b *= 65899;
            b &= 16777215;
            if 256 > c {
                if record_status.contains(&(b, c)) {
                    let mut m = 0;
                    let mut best = 0;
                    for (k, v) in record.iter() {
                        if m < *v {
                            m = *v;
                            best = *k;
                        }
                    }
                    return best;
                }
                record_status.insert((b, c));
                if let std::collections::hash_map::Entry::Vacant(ent) = record.entry(b) {
                    ent.insert(found);
                    found += 1;
                }
                continue 'outer;
            }
            e = 0;
            loop {
                if 256 * (e + 1) > c {
                    break;
                }
                e += 1;
            }
            c = e;
        }
    }
}
