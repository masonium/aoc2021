use crate::read_lines;
use std::collections::{HashMap, HashSet};
use integer_sqrt::IntegerSquareRoot;
use nalgebra as na;
use once_cell::sync::Lazy;

type V = na::Vector3<isize>;
type A = na::Point3<isize>;

type R = na::Matrix3<isize>;

#[derive(Clone)]
struct Scanner {
    beacons: Vec<V>,
    origin: A,
    dist_map: HashMap<V, Vec<(usize, usize)>>,
}

fn sort_dist(x: &V, y: &V) -> V {
    let mut r: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| (a - b).abs()).collect();
    r.sort();
    V::new(r[0], r[1], r[2])
}

static ORIENTS: Lazy<Vec<na::Matrix3<isize>>> = Lazy::new(|| {
    let mut orients = vec![];
    for x in -3..=3isize {
	for y in -3..=3isize {
	    for z in -3..=3isize {
		if x == 0 || y == 0 || z == 0 {
		    continue;
		}
		let xa = x.abs() as usize;
		let ya = y.abs() as usize;
		let za = z.abs() as usize;

		if xa == ya || ya == za || za == xa {
		    continue;
		}
		let mut m = na::Matrix3::zeros();
		m[(0, (xa-1) as usize)] = x.signum();
		m[(1, (ya-1) as usize)] = y.signum();
		m[(2, (za-1) as usize)] = z.signum();
		orients.push(m);
	    }
	}
    }
    orients
});

impl Scanner {
    fn new() -> Scanner {
	Scanner {
	    beacons: vec![],
	    dist_map: HashMap::new(),
	    origin: A::new(0, 0, 0),
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

    fn reorient(&self, v: V, r: &R) -> Self {
	//let a = inverse_orient(&a);
	let b: Vec<V> = self
	    .beacons
	    .iter()
	    .map(|b| {
		(self.origin + v + r * b).coords
	    })
	    .collect();

	Scanner {
	    origin: self.origin + v,
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
	for ori in &*ORIENTS {
	    // make sure the first two beacons map to each other.
	    let bs: Vec<(usize, usize)> = b2b
		.iter()
		.take(4)
		.map(|(a, b)| (*a, *b.iter().next().unwrap()))
		.collect();

	    let o1 = {
		let b1 = self.beacons[bs[0].0];
		let b2 = other.beacons[bs[0].1];
		b1 - ori * b2
	    };

	    let o2 = {
		let b1 = self.beacons[bs[1].0];
		let b2 = other.beacons[bs[1].1];
		b1 - ori * b2
	    };
	    let o3 = {
		let b1 = self.beacons[bs[2].0];
		let b2 = other.beacons[bs[2].1];
		b1 - ori * b2
	    };
	    let o4 = {
		let b1 = self.beacons[bs[3].0];
		let b2 = other.beacons[bs[3].1];
		b1 - ori * b2
	    };
	    if o1 == o2 && o2 == o3 && o3 == o4 {
		return other.reorient(o1, ori);
	    }
	}

	panic!();
    }
}

#[allow(unused)]
fn compose_alignment(r1: &R, r2: &R) -> R {
    r2 * r1
}

#[allow(unused)]
fn orient_location(v: &V, r: &R) -> V {
    r * v
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
	    let toks: Vec<_> = line.split(",").map(|x| x.parse::<isize>().unwrap()).collect();
	    let v = V::new(toks[0], toks[1], toks[2]);
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
    let mut max_dist = 0;
    for i in 0..found.len() {
	for j in i+1..found.len() {
	    let d = found[i].as_ref().unwrap().origin - found[j].as_ref().unwrap().origin;
	    max_dist = std::cmp::max(max_dist, d[0].abs() + d[1].abs() + d[2].abs());
	}
    }
    println!("{}", max_dist);
}
