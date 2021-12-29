use std::collections::{HashMap, HashSet};
use crate::read_lines;
use ndarray as nd;
use nd::prelude::*;

struct Var(usize);
enum Value {
    Vary(Var),
    Lit(isize)
}

enum Inst {
    Inp(Var),
    Add(Var, Value),
    Mul(Var, Value),
    Div(Var, Value),
    Mod(Var, Value),
    Eql(Var, Value)
}

impl Value {
    fn get(&self, var: &[isize]) -> isize {
	use Value::*;
	match self {
	    Vary(Var(a)) => var[*a],
	    Lit(y) => *y
	}
    }
}

fn run_phase(insts: &[Inst], phase: usize, input: isize, start_z: isize) -> isize {
    let mut vars = [0isize; 4];
    vars[3] = start_z;

    for (i, inst) in insts[phase*18..(phase+1)*18].iter().enumerate() {
	use Inst::*;
	match inst {
	    Inp(Var(v))  => {
		assert_eq!(i, 0);
		vars[*v] = input;
	    },
	    Add(Var(v), a)  => {
		let b = a.get(&vars);
		vars[*v] += b;
	    },
	    Mul(Var(ref v), a)  => {
		let b = a.get(&vars);
		vars[*v] *= b;
	    },
	    Div(Var(ref v), a)  => {
		let b = a.get(&vars);
		vars[*v] /= b;
	    },
	    Mod(Var(ref v), a)  => {
		let b = a.get(&vars);
		vars[*v] = (vars[*v]  % b + b) % b;
	    },
	    Eql(Var(ref v), a) => {
		let b = a.get(&vars);
		vars[*v] = if vars[*v] == b {
		    1
		} else {
		    0
		};
	    }
	}
    }
    vars[3]
}

pub fn day24() {
    let lines = read_lines("input/day24.txt", true).unwrap();

    fn string_to_var(s: &str) -> Option<Var> {
	match s {
	    "w" => Some(Var(0)),
	    "x" => Some(Var(1)),
	    "y" => Some(Var(2)),
	    "z" => Some(Var(3)),
	    _ => None
	}
    }

    fn string_to_value(s: &str) -> Value {
	if let Some(x) = string_to_var(s) {
	    Value::Vary(x)
	} else {
	    Value::Lit(s.parse::<isize>().unwrap())
	}
    }

    let insts: Vec<Inst> = lines.iter().map(|x| {
	let toks: Vec<&str> = x.split(" ").collect();
	if toks[0] == "inp" {
	    Inst::Inp(string_to_var(toks[1]).unwrap())
	} else {
	    let var = string_to_var(toks[1]).unwrap();
	    let val = string_to_value(toks[2]);
	    match toks[0] {
		"add" => Inst::Add(var, val),
		"mul" => Inst::Mul(var, val),
		"div" => Inst::Div(var, val),
		"mod" => Inst::Mod(var, val),
		"eql" => Inst::Eql(var, val),
		_ => panic!()
	    }
	}
    }).collect();

    let mut res = HashMap::new();
    println!("{:?}", find_res(&insts, 0, 0, true, &mut res)
	     .map(|x| x.iter().fold(0, |val, x| 10 * val + x)).unwrap());

    let mut res = HashMap::new();
    println!("{:?}", find_res(&insts, 0, 0, false, &mut res)
	     .map(|x| x.iter().fold(0, |val, x| 10 * val + x)).unwrap());

}

fn find_res(insts: &[Inst], start_phase: usize, z: isize, down: bool, res: &mut HashMap<(usize, isize), Option<Vec<isize>>>) -> Option<Vec<isize>> {
    if let Some(x) = res.get(&(start_phase, z)) {
	return x.clone();
    }
    let mut r = || {
	if start_phase == 14 {
	    if z == 0 {
		return Some(vec![]);
	    } else {
		return None;
	    }
	}
	for w in (1..=9) {
	    let w = if down { 10-w } else { w };
	    let next_z = run_phase(insts, start_phase, w, z);
	    if let Some(mut v) = find_res(insts, start_phase+1, next_z, down, res) {
		v.insert(0, w);
		return Some(v);
	    }
	}
	None
    };
    let x = r();
    res.insert((start_phase, z), x.clone());
    x
}
