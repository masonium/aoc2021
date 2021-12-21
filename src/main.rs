use aoc2021::read_lines;
use integer_sqrt::IntegerSquareRoot;
//use ndarray as nd;
#[allow(unused)]
use std::collections::{HashMap, HashSet};
// use nd::prelude::*;

type V = [i64; 3];
type A = [isize; 3];

#[derive(Clone)]
struct Scanner {
    beacons: Vec<V>,
    origin: V,
    dist_map: HashMap<V, Vec<(usize, usize)>>,
}

fn sort_dist(x: &V, y: &V) -> V {
    let mut r: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| (a - b).abs()).collect();
    r.sort();
    [r[0], r[1], r[2]]
}

fn inverse_orient(a: &A) -> A {
    for z in -3..=3isize {
        for y in -3..=3isize {
            for x in -3..=3isize {
                if x == 0 || y == 0 || z == 0 {
                    continue;
                }
                let xa = x.abs() as usize;
                let ya = y.abs() as usize;
                let za = z.abs() as usize;

                if x.abs() == y.abs() || x.abs() == z.abs() || y.abs() == z.abs() {
                    continue;
                }
                if a[xa - 1] * x.signum() == 1
                    && a[ya - 1] * y.signum() == 2
                    && a[za - 1] * z.signum() == 3
                {
                    return [x, y, z];
                }
            }
        }
    }
    panic!();
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            beacons: vec![],
            dist_map: HashMap::new(),
            origin: [0, 0, 0],
        }
    }

    fn add_beacon(&mut self, beacon: &V) {
        let n = self.beacons.len();
        for (i, b) in self.beacons.iter().enumerate() {
            let sd = sort_dist(b, beacon);
            self.dist_map.entry(sd).or_default().push((i, n));
        }
        self.beacons.push(*beacon);
    }

    fn len_intersect(&self, other: &Self) -> usize {
        let a: HashSet<V> = self.dist_map.keys().cloned().collect();
        let b: HashSet<V> = other.dist_map.keys().cloned().collect();

        a.intersection(&b).count()
    }

    fn reorient(&self, v: V, a: A) -> Self {
	let a = inverse_orient(&a);
        let b: Vec<V> = self
            .beacons
            .iter()
            .map(|b| {
                [
                    b[a[0].abs() as usize - 1] * a[0].signum() as i64,
                    b[a[1].abs() as usize - 1] * a[1].signum() as i64,
                    b[a[2].abs() as usize - 1] * a[2].signum() as i64,
                ]
            })
            .collect();

        Scanner {
            origin: [self.origin[0] + v[0], self.origin[1] + v[1], self.origin[2] + v[2]],
            beacons: b,
            dist_map: self.dist_map.clone(),
        }
    }

    // relative position, re-orient
    fn align(&self, other: &Self) -> Self {
        assert!(self.len_intersect(other) >= 66);

        let mut b2b: HashMap<usize, HashSet<usize>> = HashMap::new();

        // for _ in 0..self.beacons.len()  {
        //     b2b.push((0..other.beacons.len()).collect());
        // }

        for (k, v) in self.dist_map.iter() {
            if other.dist_map.contains_key(k) {
                // each in the pair can only map to the other pair.
                assert!(v.len() == 1 || other.dist_map[k].len() == 1);
                let r = v[0];
                let v2 = other.dist_map[k][0];
                let s: HashSet<usize> = [v2.0, v2.1].iter().cloned().collect();
                let n = b2b
                    .entry(r.0)
                    .or_insert_with(|| s.clone())
                    .intersection(&s)
                    .cloned()
                    .collect();
                b2b.insert(r.0, n);
                let n = b2b
                    .entry(r.1)
                    .or_insert_with(|| s.clone())
                    .intersection(&s)
                    .cloned()
                    .collect();
                b2b.insert(r.1, n);
            }
        }
        //println!("{}", b2b.len());

        // 1-1 mapping across common beacons
        for z in -3..=3isize {
            for y in -3..=3isize {
                'outer: for x in -3..=3isize {
                    if x == 0 || y == 0 || z == 0 {
                        continue;
                    }
                    let xa = x.abs() as usize;
                    let ya = y.abs() as usize;
                    let za = z.abs() as usize;

                    if x.abs() == y.abs() || x.abs() == z.abs() || y.abs() == z.abs() {
                        continue;
                    }
                    //println!("found orient: {:?}", (x, y, z));

                    // make sure the first two beacons map to each other.
                    let bs: Vec<(usize, usize)> = b2b
                        .iter()
                        .take(4)
                        .map(|(a, b)| (*a, *b.iter().next().unwrap()))
                        .collect();
                    //dbg!(&bs);

                    let o1 = {
                        let b1 = self.beacons[bs[0].0];
                        let b2 = other.beacons[bs[0].1];
                        //dbg!(&(b1, b2));
                        [
                            b1[0] - b2[xa - 1] * x.signum() as i64,
                            b1[1] - b2[ya - 1] * y.signum() as i64,
                            b1[2] - b2[za - 1] * z.signum() as i64,
                        ]
                    };

                    let o2 = {
                        let b1 = self.beacons[bs[1].0];
                        let b2 = other.beacons[bs[1].1];
                        [
                            b1[0] - b2[xa - 1] * x.signum() as i64,
                            b1[1] - b2[ya - 1] * y.signum() as i64,
                            b1[2] - b2[za - 1] * z.signum() as i64,
                        ]
                    };
                    let o3 = {
                        let b1 = self.beacons[bs[2].0];
                        let b2 = other.beacons[bs[2].1];
                        [
                            b1[0] - b2[xa - 1] * x.signum() as i64,
                            b1[1] - b2[ya - 1] * y.signum() as i64,
                            b1[2] - b2[za - 1] * z.signum() as i64,
                        ]
                    };
                    let o4 = {
                        let b1 = self.beacons[bs[3].0];
                        let b2 = other.beacons[bs[3].1];
                        [
                            b1[0] - b2[xa - 1] * x.signum() as i64,
                            b1[1] - b2[ya - 1] * y.signum() as i64,
                            b1[2] - b2[za - 1] * z.signum() as i64,
                        ]
                    };
                    // dbg!(o1);
                    // dbg!(o2);
                    if o1 == o2 && o2 == o3 && o3 == o4 {
                        //println!("found orient: {:?}", (x, y, z));
                        return other.reorient(o1, [x, y, z]);
                    }

                    // make sure all are consistent under new mapping
                    // for (a, b) in b2b.iter() {
                    // 	let b1 = self.beacons[*a];
                    // 	let b2 = other.beacons[*b.iter().next().unwrap()];
                    // 	//dbg!(b1, b2);
                    // 	if b1[0] !=  as i64 {
                    // 	    continue 'outer;
                    // 	}
                    // 	if b1[1] != b2[ya-1] * y.signum() as i64 {
                    // 	    continue 'outer;
                    // 	}
                    // 	if b1[2] != b2[za-1] * z.signum() as i64 {
                    // 	    continue 'outer;
                    // 	}
                    // }}
                    // println!("found orient: {:?}", (xa, ya, za));
                    // return;
                }
            }
        }

        panic!();
        //dbg!(&b2b);
    }
}

fn compose_alignment(a: &A, b: &A) -> A {
    [
        a[b[0].abs() as usize - 1] * b[0].signum(),
        a[b[1].abs() as usize - 1] * b[1].signum(),
        a[b[2].abs() as usize - 1] * b[2].signum(),
    ]
}

fn orient_location(v: &V, b: &A) -> V {
    [
        v[b[0].abs() as usize - 1] * b[0].signum() as i64,
        v[b[1].abs() as usize - 1] * b[1].signum() as i64,
        v[b[2].abs() as usize - 1] * b[2].signum() as i64,
    ]
}

pub fn day19() {
    let lines = read_lines("input/day19.txt", false).unwrap();
    let mut curr = Scanner::new();
    let mut scanners = vec![];
    for line in lines {
        if line.is_empty() {
            scanners.push(curr);
            curr = Scanner::new()
        } else if line.contains("scanner") {
        } else {
            let toks: Vec<_> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            let v = [toks[0], toks[1], toks[2]];
            curr.add_beacon(&v);
        }
    }
    //scanners.push(curr);

    let mut total_beacons: usize = scanners.iter().map(|x| x.beacons.len()).sum();

    let min_beacons = 12;
    let min_rel = min_beacons * (min_beacons - 1) / 2;
    let mut orients: Vec<_> = vec![];
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            let li = scanners[i].len_intersect(&scanners[j]);
            if li >= min_rel {
                println!("intersection of {} and {}: {}", i, j, li);
                total_beacons -= (li * 2).integer_sqrt() + 1;
                orients.push((i, j)); //, scanners[i].align(&scanners[j])));
                orients.push((j, i)); //, scanners[j].align(&scanners[i])));

                // orients.push((i, j, scanners[i].align(&scanners[j])));
                // orients.push((j, i, scanners[j].align(&scanners[i])));
            }
        }
    }
    println!("{}", total_beacons);

    // find the positions and orient of all relative to 0
    let mut found: Vec<Option<Scanner>> = vec![None; scanners.len()];
    found[0] = Some(scanners[0].clone());
    let mut active = vec![0];

    while let Some(a) = active.pop() {
        // println!("{}", a);
        // go through every alignment of a with something.
        for (i, j) in &orients {
            if *i == a && found[*j].is_none() {
		dbg!((i, j));
                found[*j] = Some(found[*i].as_ref().unwrap().align(&scanners[*j]));
                active.push(*j);
            }
        }
    }

    for x in 0..scanners.len() {
        if found[x].is_none() {
            println!("bad: {}", x);
        }
    }

    // 23:57
    println!(
        "{}",
        found
            .iter()
            .map(|x| {
                let p = x.as_ref().unwrap().origin;
                p[0].abs() + p[1].abs() + p[2].abs()
            })
            .max()
            .unwrap()
    );
}


fn main() -> std::io::Result<()> {
    // aoc2021::day01();
    // aoc2021::day02();
    // aoc2021::day03();
    // aoc2021::day04();
    // aoc2021::day05();
    // aoc2021::day06();
    // aoc2021::day07();
    // aoc2021::day08();
    // aoc2021::day09();
    // aoc2021::day10();
    // aoc2021::day11();
    // aoc2021::day12();
    // aoc2021::day13();
    // aoc2021::day14();
    // aoc2021::day15();
    // aoc2021::day16();
    // aoc2021::day17();
    // aoc2021::day18();
    //day19();
    // aoc2021::day21();
    Ok(())
}
