//! AOC 2021 solutions
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

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
	let toks: Vec<_> = r.captures(&l)
	    .unwrap().iter().collect();
	let v: Vec<_> = toks[1..].iter()
	    .map(|x| { 
		 x.unwrap().as_str().parse::<i32>().unwrap()
	    })
	    .collect();

	let x_delta = v[2] - v[0];
	let y_delta = v[3] - v[1];

	if v[0] == v[2] {
	    let m1 = std::cmp::min(v[1], v[3]);
	    let m2 = std::cmp::max(v[1], v[3]);
	    for i in m1..=m2 {
		*map.entry((v[0],i)).or_insert(0) += 1;
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
		*map.entry((v[0]+x_step*i,v[1]+y_step*i)).or_insert(0) += 1;
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
