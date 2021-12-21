use crate::read_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Left,
    Right,
    Num(usize)
}

impl Token {
    fn num(&self) -> Option<usize> {
	match self {
	    Token::Num(x) => Some(*x),
	    _ => None
	}
    }
}

#[allow(dead_code)]
fn dbg_tokens(x: &[Token]) {
    for a in x {
	match a {
	    Token::Left => print!("["),
	    Token::Right => print!("],"),
	    Token::Num(a) => print!("{},", a)
	}
    }
    println!();
}

pub fn explode(x: &mut Vec<Token>) -> bool {
    let mut depth = 0;
    let mut last_num = None;
    let mut found_num = None;
    use Token::*;
    for i in 0..x.len() {
	match x[i] {
	    Left => {
		depth += 1;
	    },
	    Right => {
		depth -= 1;
	    },
	    Num(_) if depth > 4 => {
		if let Num(_) = x[i+1] {
		    found_num = Some(i);
		    break;
		} else {
		    last_num = Some(i);
		}
	    },
	    Num(_) => {
		last_num = Some(i);
	    }
	}
    }
    if let Some(f) = found_num {
	// dbg_tokens(&x);
	// add the left number to prev
	if let Some(last) = last_num {
	    x[last] = Token::Num(x[last].num().unwrap() + x[f].num().unwrap());
	}
	// add the right number to next, if possible.
	for i in f+2..x.len() {
	    if let Token::Num(a) = x[i] {
		x[i] = Token::Num(a + x[f+1].num().unwrap());
		break;
	    }
	}
	//dbg_tokens(&x);
	// reduce
	let (a, b) = x.split_at(f-1);
	let (_, c) = b.split_at(4);
	let mut r: Vec<Token> = vec![];
	r.extend_from_slice(a);
	r.push(Token::Num(0));
	r.extend_from_slice(c);

	*x = r;
	//dbg_tokens(&x);

	return true;
    }

    false
}

pub fn split(x: &mut Vec<Token>) -> bool {
    let mut split_index = None;
    for i in 0..x.len() {
	if let Token::Num(x) = x[i] {
	    if x >= 10 {
		split_index = Some(i);
		break;
	    }
	}
    }

    if let Some(i) = split_index {
	let (a, b) = x.split_at(i);
	let (_, c) = b.split_at(1);
	let n = x[i].num().unwrap();
	let mut r = vec![];
	r.extend_from_slice(a);
	r.extend_from_slice(&[Token::Left, Token::Num(n/2), Token::Num((n+1)/2), Token::Right]);
	r.extend_from_slice(c);

	*x = r;
	return true;
    }
    false
}

pub fn reduce(p: &mut Vec<Token>) {
    loop {
	if explode(p) {
	    assert_eq!(p.iter().filter(|x| **x == Token::Left).count(),
		       p.iter().filter(|x| **x == Token::Right).count());
	    continue;
	}
	if split(p) {
	assert_eq!(p.iter().filter(|x| **x == Token::Left).count(),
		   p.iter().filter(|x| **x == Token::Right).count());
	    continue;
	}

	break;
    }
}

pub fn parse(p: &str) -> Vec<Token> {
    p.bytes().filter_map(|b| {
	match b {
	    b'[' => Some(Token::Left),
	    b']' => Some(Token::Right),
	    b',' => None,
	    b'0'..=b'9' => Some(Token::Num((b - b'0') as usize)),
	    _ => { panic!(); }
	}
    }).collect()
}

pub fn add(x: Vec<Token>, y: Vec<Token>) -> Vec<Token> {
    let mut v = vec![Token::Left];
    v.extend(x);
    v.extend(y);
    v.push(Token::Right);
    reduce(&mut v);
    assert_eq!(v.iter().filter(|x| **x == Token::Left).count(),
	       v.iter().filter(|x| **x == Token::Right).count());
    v
}

pub fn magnitude(x: &[Token]) -> usize {
    let mut mag_stack = vec![];

    for a in x {
	match a {
	    Token::Num(n) => {
		mag_stack.push(*n);
	    },
	    Token::Right => {
		let right = mag_stack.pop().unwrap();
		let left = mag_stack.pop().unwrap();
		mag_stack.push(3*left + 2 * right);
	    },
	    _ => {
	    }
	}
    }
    assert_eq!(mag_stack.len(), 1);
    mag_stack[0]
}

pub fn day18() {
    let lines = read_lines("input/day10.txt", true).unwrap();

    let a = parse(&lines[0]);
    let r = lines[1..].iter().map(|x| parse(&x)).fold(a, add);

    // let a = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    // let b = parse("[1,1]");
    // let c = add(a, b);
    // dbg_tokens(&c);

    println!("{}", magnitude(&r));

    let sf: Vec<_> = lines.iter().map(|x| parse(&x)).collect();
    let mut max = 0;
    for i in 0..sf.len() {
	for j in 0..sf.len() {
	    if i == j {
		continue;
	    }
	    max = std::cmp::max(max, magnitude(&add(sf[i].clone(), sf[j].clone())));
	}
    }
    println!("{}", max);
}
