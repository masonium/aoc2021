use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

const ALLOWED_POSITIONS: [u8; 7] = [0, 1, 3, 5, 7, 9, 10];

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Pos {
    Room(u8, u8),
    Hallway(u8)
}

impl Pos {
    fn dist(&self, other: &Self) -> u32 {
	use Pos::*;
	match (self, other) {
	    (Hallway(x), Hallway(y)) => (*x as isize - *y as isize).abs() as u32,
	    (Room(x, rank), Hallway(y)) | (Hallway(y), Room(x, rank)) => {
		(*x as isize - *y as isize).abs() as u32 + *rank as u32 + 1
	    }
	    (Room(x, r1), Room(y, r2)) if x == y => {
		(*r1 as isize - *r2 as isize).abs() as u32
	    }
	    (Room(x, r1), Room(y, r2)) => {
		(*x as isize - *y as isize).abs() as u32 + *r1 as u32 + *r2 as u32 + 2
	    }
	}
    }
}

type Amph = (u8, Pos);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Amphs {
    locs: Vec<Amph>,
}

pub fn ordered<T: Ord>(a: T, b: T) -> (T, T) {
    match a.cmp(&b) {
	std::cmp::Ordering::Greater => (b, a),
	_ => (a, b)
    }
}

impl PartialOrd for Amphs {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
	Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Amphs {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
	std::cmp::Ordering::Equal
    }
}

impl Amphs {
    fn is_occupied(&self, p: &Pos) -> Option<u8> {
	self.locs.iter().find(|x| x.1 == *p).map(|x| x.0)
    }

    /// Return true iff b is reachable from a.
    /// Assumes neither a nor b are occupied.
    fn reachable_hallway(&self, a: u8, b: u8) -> bool {
	let (a, b) = ordered(a, b);
	for ap in ALLOWED_POSITIONS {
	    if a < ap && ap < b {
		if self.is_occupied(&Pos::Hallway(ap)).is_some() {
		    return false;
		}
	    }
	}

	true
    }

    fn hallway_count(&self) -> usize {
	self.locs.iter().map(|x| match x.1 { Pos::Room(_, _) => 0, Pos::Hallway(_) => 1}).sum()
    }
    // if rank i can reach the hallway
    pub fn reachable_to_hallway(&self, p: Pos) -> bool {
	match p {
	    Pos::Room(x, rank) => {
		(0..rank).all(|r| !self.is_occupied(&Pos::Room(x, r)).is_some())
	    }
	    Pos::Hallway(_) => false
	}
    }

    /// Return true if the amphipod at position can reach `loc`.
    pub fn reachable(&self, i: usize, loc: Pos) -> bool {
	let p = self.locs[i].1;
	if loc == p {
	    return true;
	}

	if self.is_occupied(&loc).is_some() {
	    return false;
	}
	use Pos::*;
	match (p, loc) {
	    (Hallway(start), Hallway(end)) => {
		self.reachable_hallway(start, end)
	    },
	    (Room(_, _), Room(_, _)) => false,
	    (Room(x, rank), Hallway(end)) | (Hallway(end), Room(x, rank)) => {
		if self.reachable_hallway(x, end) {
		    (0..rank).all(|r| !self.is_occupied(&Room(x, r)).is_some())
		} else {
		    false
		}
	    },
	}
    }

    /// Assumes not already home
    pub fn reachable_home(&self, i: usize, max_rank: u8) -> Option<Pos> {
	let target = self.locs[i].0;
	//let p = self.locs[i].1;

	let target_room = {
	    let mut t = None;
	    for r in (0..max_rank).rev() {
		if let Some(a) = self.is_occupied(&Pos::Room(target, r)) {
		    if a == target {
			continue;
		    } else {
			return None;
		    }
		} else {
		    t = Some(r);
		    break;
		}
	    }
	    Pos::Room(target, t.unwrap())
	};


	if self.reachable(i, target_room) {
	    Some(target_room)
	} else {
	    None
	}
    }

    /// move the amph at i to a new location.
    /// Return the new state and the cost of moving.
    /// No error checking to determine if the move is valid.
    pub fn move_amph(&self, i: usize, new_pos: Pos) -> (Amphs, u32) {
	let cp = &self.locs[i].1;
	let mut n = self.clone();
	n.locs[i].1 = new_pos;
	(n, Self::cost(self.locs[i].0) * cp.dist(&new_pos))
    }

    pub fn display(&self, max_rank: u8) {
	println!("#############");
	print!("#");
	for i in 0..11 {
	    print!("{}", self.character(Pos::Hallway(i)));
	}
	println!("#");
	print!("###");
	for r in 0..max_rank {
	    for i in 2..10 {
		print!("{}", self.character(Pos::Room(i, r)));
	    }
	    println!("##");
	    print!("  #");
	}
	println!("  #########  ");
    }

    /// Distance heurisic.
    pub fn dist_heuristic(&self, _max_rank: u8) -> u32 {
	let mut cost = 0;
	for i in [2u8,4,6,8] {
	    let mut occ = 0;
	    for amph in &self.locs {
		// only consider where i is the target
		if amph.0 != i {
		    continue;
		}
		// skip if we're already in the target {
		if let Pos::Room(l, _rank) = amph.1 {
		    if l == i {
			continue;
		    }
		}
		// otherwise find the lowest rank, that's not occupied
		// with a good target, and move there.
		loop {
		    if let Some(x) = self.is_occupied(&Pos::Room(i, occ)) {
			if x == i {
			    occ += 1;
			    continue;
			}
		    }
		    break;
		}
		// move to that rank and increment
		cost += Self::cost(i) * amph.1.dist(&Pos::Room(i, occ));
		occ += 1;
	    }
	}

	cost
    }

    pub fn character(&self, pos: Pos) -> String {
	if let Some(x) = self.is_occupied(&pos) {
	    match x {
		2 => 'A',
		4 => 'B',
		6 => 'C',
		8 => 'D',
		_ => unreachable!("bad target")
	    }.to_string()
	} else {
	    match pos {
		Pos::Room(x, _) => {
		    if x % 2 == 0 { '.' } else { '#' }
		},
		Pos::Hallway(_) => '.'
	    }.to_string()
	}
    }

    pub fn cost(target: u8) -> u32 {
	match target {
	    2 => 1,
	    4 => 10,
	    6 => 100,
	    8 => 1000,
	    _ => unreachable!("bad target")
	}
    }
    pub fn done(&self) -> bool {
	self.locs.iter().all(|(target, pos)| {
	    match pos {
		Pos::Room(x, _) if x == target => true,
		_ => false
	    }
	})
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    cost_and_heur: u32,
    cost: u32,
    amphs: Amphs
}

impl Node {
    fn next(&self, i: usize, next_pos: Pos, max_rank: u8) -> Self {
	// let pos = self.amphs.locs[i].1;
	// let target = self.amphs.locs[i].0;
	let (m, c) = self.amphs.move_amph(i, next_pos);
	//let next_move = (target, pos, next_pos);
	//let mut new_moves = self.moves.clone();
	//new_moves.push(next_move);
	Node { cost_and_heur: c + self.cost + m.dist_heuristic(max_rank),
	       cost: c + self.cost,
	       //moves: new_moves,
	       amphs: m }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	(self.cost_and_heur, self.cost).partial_cmp(&(other.cost_and_heur, other.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	self.partial_cmp(&other).unwrap()
    }
}

fn day23_solve(a: &Amphs, max_rank: u8) {
    use Pos::*;
    let amphipods = Node {cost_and_heur: 0,
			  cost: 0,
			  //moves: vec![],
			  amphs: a.clone()};


    let mut bh = std::collections::binary_heap::BinaryHeap::new();
    let mut visited = HashMap::new();
    bh.push(Reverse(amphipods));

    let mut max = 0;
    while let Some(Reverse(node)) = bh.pop() {
	let mut added = vec![];

	if visited.contains_key(&node.amphs) {
	    continue;
	}

	if node.amphs.done() {
	    println!("{}", node.cost);
	    break;
	}

	visited.insert(node.amphs.clone(), node.cost);
	max = std::cmp::max(node.cost, max);

	// for each amphipod in a hallway find where it could go.
	let a = &node.amphs;
	//let allow_room_hallway_move = a.hallway_count() < ALLOWED_POSITIONS.len();
	'outer: for i in 0..a.locs.len() {
	    let pos = &a.locs[i].1;
	    let target = a.locs[i].0;
	    match pos {
		Room(x, rank) => {
		    // see if we allow moves
		    if *x == target {
			// if we are in our target room, and everything below is in the target room,
			// continue.
			if ((*rank+1)..max_rank).all(|r| {
			    if let Some(below) = a.is_occupied(&Room(*x, r)) {
				below == *x
			    } else {
				false
			    }
			}) {
			    continue;
			}
		    }
		    // if we can move to our room, that's the only option

		    // otherwise move to a hallway
		    for h in ALLOWED_POSITIONS {
			if a.reachable(i, Hallway(h)) {
			    let nn = node.next(i, Hallway(h), max_rank);
			    if nn.cost_and_heur > 50000 {
				continue;
			    }
			    if let Some(c) = visited.get(&nn.amphs) {
				if *c < nn.cost {
				    continue;
				}
			    }
			    bh.push(Reverse(nn.clone()));
			    added.push(nn);
			}
		    }
		},
		Hallway(_) => {
		    if let Some(x) = a.reachable_home(i, max_rank) {
			let nn = node.next(i, x, max_rank);
			if nn.cost_and_heur > 50000 {
			    continue;
			}
			if let Some(c) = visited.get(&nn.amphs) {
			    if *c < nn.cost {
				continue;
			    }
			}
			bh.push(Reverse(nn.clone()));
			added.push(nn);

		    }
		},
	    }
	}
    }
}

pub fn day23() {
    use Pos::*;
    let amphs = Amphs {locs: vec![(2, Room(6, 1)),
				  (2, Room(8, 1)),
				  (8, Room(6, 0)),
				  (8, Room(8, 0)),
				  (4, Room(2, 1)),
				  (4, Room(4, 0)),
				  (6, Room(2, 0)),
				  (6, Room(4, 1))]};
    day23_solve(&amphs, 2);

    let amphs = Amphs {locs: vec![(2, Room(6, 3)),
				  (2, Room(8, 3)),
				  (8, Room(6, 0)),
				  (8, Room(8, 0)),
				  (4, Room(2, 3)),
				  (4, Room(4, 0)),
				  (6, Room(2, 0)),
				  (6, Room(4, 3)),

				  (8, Room(2, 1)),
				  (8, Room(2, 2)),
				  (6, Room(4, 1)),
				  (4, Room(4, 2)),
				  (4, Room(6, 1)),
				  (2, Room(6, 2)),
				  (2, Room(8, 1)),
				  (6, Room(8, 2))]};
    //amphs.display(4);
    day23_solve(&amphs, 4);
}
