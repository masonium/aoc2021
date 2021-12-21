//! AOC 2021 solutions
pub use nd::prelude::*;
pub use ndarray as nd;
use regex::Regex;
use std::{cmp::Ord, collections::BinaryHeap};
use std::collections::{HashMap, HashSet};
use std::path::Path;
pub mod day16;
pub mod day18;
pub mod day21;

pub use day16::day16;
pub use day18::day18;
pub use day21::day21;

/// Return the set of lines in the file, optionally removing any empty lines.
pub fn read_lines<P: AsRef<Path>>(p: P, filter_empty: bool) -> std::io::Result<Vec<String>> {
    let b = std::fs::read_to_string(p)?;
    Ok(b.split("\n")
        .map(|x| x.to_string())
        .filter(|x| !filter_empty || !x.is_empty())
        .collect())
}

/// day01 solution
pub fn day01() {
    let lines = read_lines("input/day01.txt", true).unwrap();
    let nums: Vec<_> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut inc = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            inc += 1;
        }
    }
    println!("{}", inc);

    inc = 0;
    for i in 3..nums.len() {
        if nums[i] > nums[i - 3] {
            inc += 1;
        }
    }
    println!("{}", inc);
}

pub fn day02() {
    let lines = read_lines("input/day02.txt", true).unwrap();
    {
        let mut horiz = 0;
        let mut depth = 0;
        for inst in &lines {
            let toks: Vec<_> = inst.split(" ").collect();
            if toks.len() < 2 {
                continue;
            }
            let dir = toks[0];
            let dist = toks[1].parse::<i32>().unwrap();

            match dir {
                "forward" => {
                    horiz += dist;
                }
                "down" => {
                    depth += dist;
                }
                "up" => {
                    depth -= dist;
                    depth = std::cmp::max(depth, 0);
                }
                _ => {
                    continue;
                }
            }
        }

        println!("{}", horiz * depth);
    }
    {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;
        for inst in lines {
            let toks: Vec<_> = inst.split(" ").collect();
            if toks.len() < 2 {
                continue;
            }
            let dir = toks[0];
            let dist = toks[1].parse::<i32>().unwrap();

            match dir {
                "forward" => {
                    horiz += dist;
                    depth += aim * dist
                }
                "down" => {
                    aim += dist;
                }
                "up" => {
                    aim -= dist;
                }
                _ => {
                    continue;
                }
            }
        }

        println!("{}", horiz * depth);
    }
}

fn filter_by_common(c: &Vec<Vec<u8>>, bit: usize, most_common: bool) -> Vec<Vec<u8>> {
    let mut one_count = 0;
    for x in c {
        if x[bit] == b'1' {
            one_count += 1;
        }
    }
    let zero_count = c.len() - one_count;

    let mut common: Vec<Vec<u8>> = vec![];
    let target_bit = if most_common {
        if one_count >= zero_count {
            b'1'
        } else {
            b'0'
        }
    } else {
        if one_count < zero_count {
            b'1'
        } else {
            b'0'
        }
    };

    for x in c {
        if x[bit] == target_bit {
            common.push(x.clone());
        }
    }
    common
}

pub fn day03() {
    let lines = read_lines("input/day03.txt", true).unwrap();

    let mut one_count = vec![0; lines[0].len()];
    for x in &lines {
        for (i, b) in x.bytes().enumerate() {
            if b == b'1' {
                one_count[i] += 1;
            }
        }
    }
    let h = lines.len() / 2;
    let gamma_str = one_count
        .iter()
        .map(|x| if *x > h { '1' } else { '0' })
        .collect::<String>();
    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = (1 << lines[0].len()) - gamma - 1;

    println!("{}", gamma * epsilon);

    let num_bits = lines[0].len();
    let mut oxy: Vec<_> = lines.iter().map(|x| x.clone().into_bytes()).collect();
    let mut cos = oxy.clone();

    for b in 0..num_bits {
        if oxy.len() > 1 {
            oxy = filter_by_common(&oxy, b, true);
        }
        if cos.len() > 1 {
            cos = filter_by_common(&cos, b, false);
        }
    }

    println!(
        "{}",
        i32::from_str_radix(&String::from_utf8(oxy[0].clone()).unwrap(), 2).unwrap()
            * i32::from_str_radix(&String::from_utf8(cos[0].clone()).unwrap(), 2).unwrap()
    );
}

struct Bingo {
    numbers: Vec<i64>,
    is_called: Vec<bool>,
}

impl Bingo {
    fn new(nums: Vec<i64>) -> Bingo {
        assert!(nums.len() == 25);
        Bingo {
            numbers: nums,
            is_called: vec![false; 25],
        }
    }

    fn call(&mut self, num: i64) {
        let pos = self.numbers.iter().position(|x| *x == num);
        if let Some(p) = pos {
            self.is_called[p] = true;
        }
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.is_called[i * 5..(i + 1) * 5].iter().all(|x| *x) {
                return true;
            }
            let mut c = true;
            for j in 0..5 {
                if !self.is_called[i + j * 5] {
                    c = false;
                }
            }
            if c {
                return true;
            }
        }
        false
    }

    fn sum_uncalled(&self) -> i64 {
        self.numbers
            .iter()
            .zip(self.is_called.iter())
            .filter_map(|(n, c)| if !*c { Some(n) } else { None })
            .sum()
    }
}

fn num_list(s: &str) -> Vec<i64> {
    s.split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn day04() {
    let lines = read_lines("input/day04.txt", true).unwrap();
    let nums: Vec<i64> = lines[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut i = 1;
    let mut cards = vec![];
    while i < lines.len() {
        let mut nums = vec![];
        for j in 0..5 {
            nums.append(&mut num_list(&lines[i + j]));
        }
        cards.push(Bingo::new(nums));

        i += 5;
    }

    let cl = cards.len();
    let mut num_wins = 0;
    let mut has_won = vec![false; cl];
    for n in nums.iter() {
        for (ic, c) in cards.iter_mut().enumerate() {
            c.call(*n);
            if !has_won[ic] && c.has_bingo() {
                num_wins += 1;
                has_won[ic] = true;
                if num_wins == 1 || num_wins == cl {
                    println!("{}", c.sum_uncalled() * *n);
                }
            }
        }
    }
}

fn build_map_day05(lines: &[String], include_diag: bool) -> HashMap<(i32, i32), usize> {
    let mut map = HashMap::new();

    let r = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    for l in lines {
        let toks: Vec<_> = r.captures(&l).unwrap().iter().collect();
        let v: Vec<_> = toks[1..]
            .iter()
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        let x_delta = v[2] - v[0];
        let y_delta = v[3] - v[1];

        if v[0] == v[2] {
            let m1 = std::cmp::min(v[1], v[3]);
            let m2 = std::cmp::max(v[1], v[3]);
            for i in m1..=m2 {
                *map.entry((v[0], i)).or_insert(0) += 1;
            }
        } else if v[1] == v[3] {
            let m1 = std::cmp::min(v[0], v[2]);
            let m2 = std::cmp::max(v[0], v[2]);
            for i in m1..=m2 {
                *map.entry((i, v[1])).or_insert(0) += 1;
            }
        } else if include_diag && x_delta.abs() == y_delta.abs() {
            let x_step = x_delta.signum();
            let y_step = y_delta.signum();
            for i in 0..=x_delta.abs() {
                *map.entry((v[0] + x_step * i, v[1] + y_step * i))
                    .or_insert(0) += 1;
            }
        }
    }

    map
}

pub fn day05() {
    let lines = read_lines("input/day05.txt", true).unwrap();

    let m1 = build_map_day05(&lines, false);
    let m2 = build_map_day05(&lines, true);

    println!("{}", m1.values().filter(|x| **x > 1).count());
    println!("{}", m2.values().filter(|x| **x > 1).count());
}

pub fn num_fish(start: i64, days: i64) -> i64 {
    if days <= start {
        return 1;
    }

    let days_left = days - start - 1;
    num_fish(6, days_left) + num_fish(8, days_left)
}

pub fn day06() {
    let lines = read_lines("input/day06.txt", true).unwrap();

    let mut num_fish_iter: Array2<Option<i64>> = nd::Array2::from_shape_fn((9, 257), |_| None);
    for s in 0..=8 {
        num_fish_iter[(s, 0)] = Some(1);
    }
    for d in 1..=256 {
        for s in 0..=8 {
            if s == 0 {
                num_fish_iter[(s, d)] =
                    Some(num_fish_iter[(6, d - 1)].unwrap() + num_fish_iter[(8, d - 1)].unwrap());
            } else {
                num_fish_iter[(s, d)] = Some(num_fish_iter[(s - 1, d - 1)].unwrap());
            }
        }
    }

    let fish = lines[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut count_fish = std::collections::HashMap::new();
    for f in fish {
        *count_fish.entry(f).or_insert(0) += 1;
    }

    println!(
        "{}",
        count_fish
            .iter()
            .map(|(k, v)| v * num_fish_iter[(*k as usize, 80 as usize)].unwrap())
            .sum::<i64>()
    );
    println!(
        "{}",
        count_fish
            .iter()
            .map(|(k, v)| v * num_fish_iter[(*k as usize, 256 as usize)].unwrap())
            .sum::<i64>()
    );
}

pub fn day07() {
    let lines = read_lines("input/day07.txt", true).unwrap();
    let mut crabs: Vec<_> = lines[0]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    crabs.sort();
    let mid1 = crabs[(crabs.len() - 1) / 2];
    let mid2 = crabs[crabs.len() / 2];
    assert!(mid1 == mid2);
    println!("{}", crabs.iter().map(|x| (*x - mid1).abs()).sum::<i64>());

    let dist2 = |a: i64, b: i64| (a - b).abs() * ((a - b).abs() + 1) / 2;

    let m = (crabs.iter().sum::<i64>() as f64 / crabs.len() as f64).floor() as i64;
    println!(
        "{}",
        std::cmp::min(
            crabs.iter().map(|x| dist2(*x, m)).sum::<i64>(),
            crabs.iter().map(|x| dist2(*x, m + 1)).sum::<i64>()
        )
    );
}

/// Return a decoding map.
pub fn day08_decipher(inputs: &[String]) -> HashMap<char, char> {
    let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<HashMap<_, _>>();
    let alpha_set: HashSet<char> = alpha.keys().cloned().collect();

    let mut seg2 = "";
    let mut seg3 = "";
    let mut seg4 = "";
    let mut seg6 = vec![];
    let mut seg5 = vec![];

    for x in inputs {
        match x.len() {
            2 => {
                seg2 = x;
            }
            3 => {
                seg3 = x;
            }
            4 => {
                seg4 = x;
            }
            5 => {
                seg5.push(x.chars().collect::<HashSet<char>>());
            }
            6 => {
                seg6.push(x.chars().collect::<HashSet<char>>());
            }
            _ => {}
        }
    }
    assert!(seg6.len() == 3);
    assert!(seg5.len() == 3);

    // c and f must be in seg2
    let seg2_set = seg2.chars().collect::<HashSet<char>>();
    let cf = seg2_set.clone();

    // a is the differnce between seg3 and seg2
    let seg3_set = seg3.chars().collect::<HashSet<char>>();

    let a_seg = *seg3_set.difference(&seg2_set).next().unwrap();

    // b and d are in the differece between seg4 and seg2
    let seg4_set = seg4.chars().collect::<HashSet<char>>();
    let bd: HashSet<char> = seg4_set.difference(&seg2_set).cloned().collect();
    assert!(bd.len() == 2);

    // d is in every 5 segment and missing in one 6 segment.
    let i5: HashSet<char> = seg5.iter().fold(alpha_set.clone(), |x, y| {
        x.intersection(&y).cloned().collect()
    });
    let i6: HashSet<char> = seg6.iter().fold(alpha_set.clone(), |x, y| {
        x.intersection(&y).cloned().collect()
    });
    let i5_no_6: HashSet<_> = i5.difference(&i6).cloned().collect();
    assert!(i5_no_6.len() == 1);
    let d_seg = *i5_no_6.iter().next().unwrap();

    let g_seg = {
        let mut adg = i5.clone();
        adg.remove(&a_seg);
        adg.remove(&d_seg);
        *adg.iter().next().unwrap()
    };

    // b is the 'other' one.
    let bd_no_d: HashSet<char> = bd
        .difference(&[d_seg].iter().cloned().collect())
        .cloned()
        .collect();
    assert_eq!(bd_no_d.len(), 1);
    let b_seg: char = *bd_no_d.iter().next().unwrap();

    // e is the one left;
    let e_seg: char = {
        let mut all = cf.clone();
        all = all.union(&bd).cloned().collect();
        all.insert(a_seg);
        all.insert(g_seg);
        let last: Vec<char> = alpha_set.difference(&all).cloned().collect();
        assert_eq!(last.len(), 1);
        last[0]
    };

    // c is appears with e in a 5-segment
    let c_seg: char = (|| {
        for s5 in &seg5 {
            if s5.contains(&e_seg) {
                return *s5
                    .difference(
                        &[e_seg, a_seg, d_seg, g_seg]
                            .iter()
                            .cloned()
                            .collect::<HashSet<char>>(),
                    )
                    .next()
                    .unwrap();
            }
        }
        panic!("");
    })();

    // f is appears with b in a 5-segment
    let f_seg: char = (|| {
        for s5 in seg5 {
            if s5.contains(&b_seg) {
                return *s5
                    .difference(
                        &[b_seg, a_seg, d_seg, g_seg]
                            .iter()
                            .cloned()
                            .collect::<HashSet<char>>(),
                    )
                    .next()
                    .unwrap();
            }
        }
        panic!("");
    })();

    let tups = [
        (a_seg, 'a'),
        (b_seg, 'b'),
        (c_seg, 'c'),
        (d_seg, 'd'),
        (e_seg, 'e'),
        (f_seg, 'f'),
        (g_seg, 'g'),
    ];

    tups.iter().cloned().collect()
}

fn encode(decode_map: &HashMap<char, char>, outputs: &[String]) -> usize {
    let m: HashMap<String, usize> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .enumerate()
    .map(|(i, c)| (c.to_string(), i))
    .collect();

    outputs
        .iter()
        .map(|x| {
            let mut c: Vec<char> = x.chars().map(|x| decode_map[&x]).collect();
            c.sort();
            let sorted: String = String::from_iter(c.iter());
            m[&sorted]
        })
        .fold(0, |a, b| a * 10 + b)
}

pub fn day08() {
    let lines = read_lines("input/day08.txt", true).unwrap();
    let s: usize = lines
        .iter()
        .map(|line| {
            line.split(" ")
                .skip(11)
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum();
    println!("{}", s);

    let mut sum = 0;
    for line in &lines {
        let toks: Vec<String> = line.split(" ").map(|x| x.to_owned()).collect();
        let inputs = &toks[..10];
        let outputs = &toks[11..15];
        let decode_map = day08_decipher(&inputs);
        let digits = encode(&decode_map, &outputs);
        sum += digits;
    }

    println!("{}", sum);
}

pub fn read_grid<P: AsRef<Path>>(path: P) -> Array2<usize> {
    let lines = read_lines(path, true).unwrap();
    let mut h: Array2<usize> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            h[(r, j)] = (c - b'0') as usize;
        }
    }

    h
}


pub fn day09() {
    let lines = read_lines("input/day09.txt", true).unwrap();

    let mut h: Array2<usize> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            h[(r, j)] = (c - b'0') as usize;
        }
    }

    let d = h.dim();
    let mut risk = 0;
    let mut basins: Vec<(usize, usize)> = vec![];
    for i in 0..d.0 {
        for j in 0..d.1 {
            let c = h[(i, j)];
            if i > 0 && c >= h[(i - 1, j)] {
                continue;
            }
            if j > 0 && c >= h[(i, j - 1)] {
                continue;
            }
            if i < d.0 - 1 && c >= h[(i + 1, j)] {
                continue;
            }
            if j < d.1 - 1 && c >= h[(i, j + 1)] {
                continue;
            }
            basins.push((i, j));
            risk += c + 1;
        }
    }
    println!("{}", risk);

    let mut rem: Vec<((usize, usize), usize)> =
        basins.iter().enumerate().map(|(i, x)| (*x, i)).collect();
    let mut basin_map: HashMap<(usize, usize), usize> = HashMap::new();

    while let Some((r @ (i, j), b)) = rem.pop() {
        if h[(i, j)] == 9 {
            continue;
        }
        if basin_map.contains_key(&r) {
            continue;
        }
        basin_map.insert(r, b);

        if i > 0 {
            rem.push(((i - 1, j), b));
        }
        if j > 0 {
            rem.push(((i, j - 1), b));
        }
        if i < d.0 - 1 {
            rem.push(((i + 1, j), b));
        }
        if j < d.1 - 1 {
            rem.push(((i, j + 1), b));
        }
    }

    // compute the basin size
    let mut sizes: HashMap<usize, usize> = HashMap::new();
    basin_map
        .values()
        .for_each(|x| *sizes.entry(*x).or_insert(0) += 1);
    let mut r: Vec<usize> = sizes.values().cloned().collect();
    r.sort();
    r.reverse();
    println!("{}", r[0] * r[1] * r[2]);
}

#[derive(Debug, Clone, Copy)]
pub enum P10 {
    Corrupt(usize),
    Incomplete(usize),
}

/// Return the first corrupted char, if there is one.
pub fn fix(s: &str) -> P10 {
    let fails: HashMap<char, usize> = ")]}>".chars().zip([3, 57, 1197, 25137]).collect();
    let finishes: HashMap<char, usize> = ")]}>".chars().zip([1, 2, 3, 4]).collect();

    let mut cstack = vec![];
    let opens: HashSet<char> = "[{(<".chars().collect();
    let closes: HashMap<char, char> = "[{(<".chars().zip("]})>".chars()).collect();
    for c in s.chars() {
        if opens.contains(&c) {
            cstack.push(c)
        } else {
            if let Some(x) = cstack.last() {
                if closes[x] == c {
                    cstack.pop();
                } else {
                    return P10::Corrupt(fails[&c]);
                }
            } else {
                return P10::Corrupt(fails[&c]);
            }
        }
    }

    P10::Incomplete(
        cstack
            .iter()
            .rev()
            .fold(0, |a, b| a * 5 + finishes[&closes[b]]),
    )
}

pub fn day10() {
    let lines = read_lines("input/day10.txt", true).unwrap();
    let errors: Vec<P10> = lines.iter().map(|s| fix(&s)).collect();
    println!(
        "{}",
        errors
            .iter()
            .filter_map(|c| match *c {
                P10::Corrupt(x) => Some(x),
                _ => None,
            })
            .sum::<usize>()
    );

    let mut incs: Vec<usize> = errors
        .iter()
        .filter_map(|c| match *c {
            P10::Incomplete(x) => Some(x),
            _ => None,
        })
        .collect();
    incs.sort();
    println!("{}", incs[incs.len() / 2]);
}

fn day11_step(e: &mut Array2<usize>) -> usize {
    let d = e.dim();
    e.mapv_inplace(|x| x + 1);

    let mut flashed = Array2::from_elem((d.0, d.1), false);
    let mut num_flashed = 0;

    loop {
        let mut nf = 0;
        for i in 0..d.0 {
            for j in 0..d.1 {
                // skip already flashed
                if flashed[(i, j)] {
                    continue;
                }
                if e[(i, j)] > 9 {
                    // flash this one
                    flashed[(i, j)] = true;
                    nf += 1;

                    // increase energy of neightbos {
                    let i_s = i as isize;
                    let j_s = j as isize;
                    for ni in std::cmp::max(i_s - 1, 0)..=std::cmp::min(i_s + 1, d.0 as isize - 1) {
                        for nj in
                            std::cmp::max(j_s - 1, 0)..=std::cmp::min(j_s + 1, d.1 as isize - 1)
                        {
                            e[(ni as usize, nj as usize)] += 1;
                        }
                    }
                }
            }
        }
        if nf > 0 {
            num_flashed += nf;
        } else {
            break;
        }
    }
    for i in 0..d.0 {
        for j in 0..d.1 {
            if flashed[(i, j)] {
                e[(i, j)] = 0;
            }
        }
    }

    num_flashed
}

pub fn day11() {
    let lines = read_lines("input/day11.txt", true).unwrap();
    let mut e: Array2<usize> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            e[(r, j)] = (c - b'0') as usize;
        }
    }

    let mut first_all = None;
    let mut num_flashed = 0;
    for i in 1..=100 {
        let nf = day11_step(&mut e);
        if nf == 100 && first_all == None {
            first_all = Some(i)
        }
        num_flashed += nf;
    }
    println!("{}", num_flashed);
    let mut i = 101;
    while let None = first_all {
        let nf = day11_step(&mut e);
        if nf == 100 {
            first_all = Some(i)
        } else {
            i += 1;
        }
    }

    println!("{}", i);
}
#[derive(Clone, Debug)]
struct Path12 {
    /// small caves that have been visited
    pub visited: HashMap<String, usize>,

    /// order list of visited structures
    pub path: Vec<String>,
}

impl Path12 {
    fn new() -> Path12 {
        Path12 {
            visited: [("start".to_string(), 1)].into_iter().collect(),
            path: vec!["start".to_string()],
        }
    }

    /// Return true if the small path hasn't been visited yet
    fn can_add1(&self, n: &str) -> bool {
        char::is_uppercase(n.as_bytes()[0] as char) || !self.visited.contains_key(n)
    }

    /// Return true if the small path has been visited at most once yet
    fn can_add2(&self, n: &str) -> bool {
        if char::is_uppercase(n.as_bytes()[0] as char) {
	    return true;
	}
	if n == "start" {
	    return false;
	}
	if !self.visited.contains_key(n) {
	    return true;
	}
	if self.visited.values().all(|x| *x < 2) {
	    return true;
	}
	return false;
    }

    fn last(&self) -> &str {
        self.path.last().unwrap()
    }

    fn add(&self, n: &str) -> Path12 {
        let mut v = self.visited.clone();
	if char::is_lowercase(n.as_bytes()[0] as char) {
            *v.entry(n.to_string()).or_default() += 1;
	}

        let mut path = self.path.clone();
        path.push(n.to_string());
        Path12 { visited: v, path }
    }
}

// (setq lsp-log-io t)
pub fn day12() {
    let lines = read_lines("input/day12.txt", true).unwrap();

    let mut edges: Vec<(String, String)> = vec![];
    let mut edge_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let mut toks: Vec<_> = line.split("-").map(|x| x.to_string()).collect();
        assert_eq!(toks.len(), 2);

        let t1 = toks.pop().unwrap();
        let t0 = toks.pop().unwrap();
        edges.push((t0.clone(), t1.clone()));
        edge_map.entry(t0.clone()).or_default().push(t1.clone());
        edge_map.entry(t1).or_default().push(t0);
    }

    // enumere all of the paths
    let mut active_paths = vec![Path12::new()];

    let mut paths = vec![];

    while let Some(p) = active_paths.pop() {
        let last = p.last();
        for next in edge_map.get(last).unwrap_or(&vec![]) {
            if p.can_add1(&next) {
                let extend = p.add(next);
                if next == "end" {
                    paths.push(extend);
                } else {
                    active_paths.push(extend);
                }
            }
        }
    }

    println!("{:?}", paths.len());

    active_paths = vec![Path12::new()];
    paths = vec![];

    while let Some(p) = active_paths.pop() {
        let last = p.last();
        for next in edge_map.get(last).unwrap_or(&vec![]) {
            if p.can_add2(&next) {
                let extend = p.add(next);
                if next == "end" {
                    paths.push(extend);
                } else {
                    active_paths.push(extend);
                }
            }
        }
    }
    
    // for p in paths.iter().map(|x| x.path.join(",")) {
    // 	println!("{}", p);
    // }
    println!("{}", paths.len());
}

pub fn day13() {
    let lines = read_lines("input/day13.txt", true).unwrap();

    let mut points = vec![];
    let mut folds = vec![];
    for line in lines {
	if line.starts_with("fold") {
	    let l = line.to_string();
	    let axis: Vec<_> = l.split(" ").collect();
	    let parts: Vec<_> = axis[2].split("=").collect();
	    folds.push((parts[0].to_string(), parts[1].parse::<isize>().unwrap()));
	} else {
	    let toks: Vec<_> = line.split(",").collect();
	    points.push((toks[0].parse::<isize>().unwrap(), toks[1].parse::<isize>().unwrap()));
	}
    }
    let mut pset: HashSet<(isize, isize)> = HashSet::from_iter(points.into_iter());
    for (i, fold) in folds.iter().enumerate() {
	let mut new_pset = HashSet::new();
	for p in &pset {
	    if fold.0 == "x" {
		if p.0 < fold.1 {
		    new_pset.insert((p.0, p.1));
		} else {
		    new_pset.insert((2 * fold.1 - p.0, p.1));
		}
	    } else {
		if p.1 < fold.1 {
		    new_pset.insert((p.0, p.1));
		} else {
		    new_pset.insert((p.0, 2 * fold.1 - p.1));
		}
	    }
	}
	pset = new_pset;
	if i == 0 {
	    println!("{}", pset.len());
	}
    }
    let x_min = pset.iter().map(|p| p.0).min().unwrap();
    let x_max = pset.iter().map(|p| p.0).max().unwrap();
    let y_min = pset.iter().map(|p| p.1).min().unwrap();
    let y_max = pset.iter().map(|p| p.1).max().unwrap();
    let mut arr = Array2::from_elem(((x_max-x_min+1) as usize, (y_max-y_min+1) as usize), ' ');
    for p in pset.iter() {
	arr[((p.0 - x_min) as usize, (p.1 - y_min) as usize)] = '#';
    }
    for y in y_min..=y_max {
	for x in x_min..=x_max {
	    print!("{}", arr[((x-x_min) as usize, (y-y_min) as usize)]);
	}
	println!("");
    }
}

pub fn day14() {
    let lines = read_lines("input/day14.txt", true).unwrap();

    let val = Vec::from(lines[0].as_bytes());
    let mut rules: HashMap<String, (String, String)> = HashMap::new();
    for x in &lines[1..] {
	let toks: Vec<String> = x.split(" -> ").map(|x| x.to_string()).collect();
	let k = toks[0].as_bytes();
	let v = toks[1].as_bytes()[0];
	rules.insert(toks[0].clone(), (String::from_utf8_lossy(&[k[0], v]).to_string(),
				       String::from_utf8_lossy(&[v, k[1]]).to_string()));
    }

    let mut bigram_counts: HashMap<String, usize> = HashMap::new();
    for x in val.windows(2) {
	let s = String::from_utf8_lossy(&x).to_string();
	*bigram_counts.entry(s).or_default() += 1;
    }

    // after 10 iterations
    for _ in 0..10 {
	let mut new_bc: HashMap<String, _> = HashMap::new();
	for (k ,v) in bigram_counts.iter() {
	    let (a, b) = &rules[k];
	    *new_bc.entry(a.to_string()).or_default() += v;
	    *new_bc.entry(b.to_string()).or_default() += v;
	}
	bigram_counts = new_bc;
    }

    let mut m: HashMap<u8, usize> = HashMap::new();
    bigram_counts.iter().for_each(|(b, v)| {
	let s = b.as_bytes();
	*m.entry(s[0]).or_default() += v;
	*m.entry(s[1]).or_default() += v;
    });

    println!("{}", m.values().max().unwrap() /2 - m.values().min().unwrap() /2);

    // 30 more iterations
    for _ in 0..30 {
	let mut new_bc: HashMap<String, _> = HashMap::new();
	for (k ,v) in bigram_counts.iter() {
	    let (a, b) = &rules[k];
	    *new_bc.entry(a.to_string()).or_default() += v;
	    *new_bc.entry(b.to_string()).or_default() += v;
	}
	bigram_counts = new_bc;
    }

    let mut m: HashMap<u8, usize> = HashMap::new();
    bigram_counts.iter().for_each(|(b, v)| {
	let s = b.as_bytes();
	*m.entry(s[0]).or_default() += v;
	*m.entry(s[1]).or_default() += v;
    });
    
    println!("{}", m.values().max().unwrap() /2 - m.values().min().unwrap() /2);
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    path: Vec<(usize, usize)>,
    dist: usize
}

impl Node {
    fn next(&self, n: (usize, usize), h: usize) -> Node {
	let mut p = self.path.clone();
	p.push(n);
	Node { path: p, dist: self.dist + h }
    }

    fn pos(&self) -> (usize, usize) {
	*self.path.last().unwrap()
    }
    fn len(&self) -> usize {
	self.path.len()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	(other.dist, other.len()).cmp(&(self.dist, self.len()))
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day15_solve(h: &Array2<usize>) {
    let d = h.dim();
    let mut active_paths = BinaryHeap::new();
    active_paths.push(Node { path: vec![(0, 0)], dist: 0 });
    let mut visited = HashSet::new();
    
    while let Some(ap) = active_paths.pop() {
	if ap.pos() == (d.0-1, d.1-1) {
	    println!("{}", ap.dist);
	    break;
	}
	if visited.contains(&ap.pos()) {
	    continue;
	}
	visited.insert(ap.pos());

	if ap.pos().0 > 0 {
	    let new_pos = (ap.pos().0 - 1, ap.pos().1);
	    if !ap.path.contains(&new_pos) {
		active_paths.push(ap.next(new_pos, h[new_pos]));
	    }
	}
	if ap.pos().1 > 0 {
	    let new_pos = (ap.pos().0, ap.pos().1 - 1);
	    if !ap.path.contains(&new_pos) {
		active_paths.push(ap.next(new_pos, h[new_pos]));
	    }
	}
	if ap.pos().0 < d.0-1 {
	    let new_pos = (ap.pos().0 + 1, ap.pos().1);
	    if !ap.path.contains(&new_pos) {
		active_paths.push(ap.next(new_pos, h[new_pos]));
	    }
	}
	if ap.pos().1 < d.1-1 {
	    let new_pos = (ap.pos().0, ap.pos().1 + 1);
	    if !ap.path.contains(&new_pos) {
		active_paths.push(ap.next(new_pos, h[new_pos]));
	    }
	}
	//	println!("{:?}", active_paths);
    }

}

pub fn day15() {
    let lines = read_lines("input/day15.txt", true).unwrap();

    let mut h: Array2<usize> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            h[(r, j)] = (c - b'0') as usize;
        }
    }

    day15_solve(&h);
    let d = h.dim();
    let mut h2: Array2<usize> = Array2::zeros((d.0 * 5, d.1 * 5));
    for zy in 0..5 {
	for zx in 0..5 {
	    for y in 0..d.0 {
		for x in 0..d.1 {
		    h2[(zy*d.0 + y, zx*d.1 + x)] = (h[(y, x)] + zy + zx - 1) % 9 + 1;
		}
	    }
	}
    }

    day15_solve(&h2);
}

pub fn day17() {
    let x_target = (156, 202);
    let y_target = (-110, -69);

    let mut max_height: Option<isize> = None;
    let mut num_vel = 0;
    for x_vel in -1000..2000 {
	for y_vel in -3000..3000 {
	    let mut mh = 0;
	    let mut vel: [isize; 2] = [x_vel, y_vel];
	    let mut pos = [0, 0];
	    for _ in 0..500 {
		pos[0] += vel[0];
		pos[1] += vel[1];
		vel[0] = vel[0].signum() * std::cmp::max(vel[0].abs() - 1, 0);
		vel[1] -= 1;
		mh = std::cmp::max(pos[1], mh);
		if x_target.0 <= pos[0] && pos[0] <= x_target.1 &&
		    y_target.0 <= pos[1] && pos[1] <= y_target.1 {
			num_vel += 1;
			max_height = Some(max_height.map(|z| std::cmp::max(z, mh)).unwrap_or(mh));
			break;
		    }
	    }
	}
    };
    println!("{}", max_height.unwrap());
    println!("{}", num_vel);
}
