use std::collections::HashMap;

#[derive(PartialEq, Hash, Clone, Eq)]
struct State {
    p: [usize; 2],
    score: [usize; 2],
    i: usize
}

static DIE: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

impl State {
    fn new() -> State {
	State { p: [4, 7], score: [0, 0], i: 0}
    }

    fn advance(&self) -> HashMap<State, usize> {
	let mut states = HashMap::new();
	for s in 3..=9 {
	    let mut p = self.p.clone();
	    let i = self.i;
	    let mut score = self.score.clone();

	    p[i%2] = (p[i%2] + s - 1) % 10 + 1;
	    score[i%2] += p[i%2];

	    let new_state = State {p, score, i: i + 1};
	    *states.entry(new_state).or_default() += DIE[s];
	}
	states
    }
}

pub fn day21() {

    let mut p = [4, 7];

    let mut score = [0, 0];

    let mut i = 0;
    let mut d = 1;
    loop {
	let s = d + ((d % 100) + 1) + (((d+1) % 100) + 1);
	d = ((d + 2) % 100) + 1;

	p[i%2] = (p[i%2] + s - 1) % 10 + 1;
	score[i%2] += p[i%2];
	if score[i%2] >= 1000 {
	    println!("{}", (i+1) * 3 * score[(i+1) % 2]);
	    break;
	}

	i += 1;
    }

    {
	let mut states = HashMap::new();
	states.insert(State::new(), 1);
	let mut wins = [0, 0];

	while states.len() > 0 {
	    let mut new_states = HashMap::new();
	    for (s, v) in states.iter() {
		for (x, y) in s.advance() {
		    if x.score[0] >= 21 {
			wins[0] += v*y;
		    } else if x.score[1] >= 21 {
			wins[1] += v*y;
		    } else {
			*new_states.entry(x).or_default() += v*y;
		    }
		}
	    }
	    states = new_states;
	}
	println!("{:?}", std::cmp::max(wins[0], wins[1]));
    }
}
