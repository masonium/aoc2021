use crate::read_lines;

enum Data {
    Literal(usize),  Sum, Product, Min, Max, Greater, Less, Equal
}

struct Packet {
    version: usize,
    data: Data,
    sub_packets: Vec<Packet>
}

impl Packet {
    fn literal(v: usize, data: usize) -> Packet {
	Packet { version: v, data: Data::Literal(data), sub_packets: vec![] }
    }
    fn op(v: usize, op_num: usize) -> Packet {
	use Data::*;
	let o = match op_num {
	    0 => Sum, 1 => Product, 2 => Min, 3 => Max, 5 => Greater, 6 => Less, 7 => Equal, _ => { panic!(); }
	};
	Packet { version: v, data: o, sub_packets: vec![] }
    }
    fn sum_versions(&self) -> usize {
	self.version + self.sub_packets.iter().map(|x| x.sum_versions()).sum::<usize>()
    }

    fn interp(&self) -> usize {
	use Data::*;
	let vals: Vec<usize> = self.sub_packets.iter().map(|x| x.interp()).collect();
	match self.data {
	    Literal(x) => x,
	    Sum => {
		vals.iter().sum()
	    },
	    Product => {
		vals.iter().product()
	    },
	    Min => {
		*vals.iter().min().unwrap()
	    },
	    Max => {
		*vals.iter().max().unwrap()
	    },
	    Greater => {
		if vals[0] > vals[1] { 1 } else { 0 }
	    },
	    Less => {
		if vals[0] < vals[1] { 1 } else { 0 }
	    },
	    Equal => {
		if vals[0] == vals[1] { 1 } else { 0 }
	    },
	}
    }
}

fn bin(x: &[u8]) -> usize {
    usize::from_str_radix(&String::from_utf8_lossy(x), 2).unwrap()
}
fn hex(x: &[u8]) -> usize {
    usize::from_str_radix(&String::from_utf8_lossy(x), 16).unwrap()
}

fn parse_packet(x: &[u8]) -> Option<(Packet, usize, &[u8])> {
    let version = bin(&x[..3]);
    let id = bin(&x[3..6]);
    println!("{}, {}", version, id);
    if id == 4 {
	let mut r = 6;
	let mut lit = 0;
	while x[r] == b'1' {
	    lit = lit * 16 + bin(&x[r+1..=r+4]);
	    r += 5;
	}
	lit = lit * 16 + bin(&x[r+1..=r+4]);
	r += 5;
	return Some((Packet::literal(version, lit), r, &x[r..]));
    }

    if id != 4 {
	let type_id = x[6] - b'0';

	println!("op {}", type_id);
	if type_id == 0 {
	    let total_len = bin(&x[7..22]);
	    println!("bit len {}", total_len);
	    // length of subpackets
	    let mut l: usize = 0;
	    let mut next = &x[22..];
	    let mut packet = Packet::op(version, id);
	    while l < total_len {
		let (p, len, n) = parse_packet(next).unwrap();
		packet.sub_packets.push(p);
		next = n;
		l += len;
	    }
	    assert_eq!(l, total_len);
	    return Some((packet, 22 + total_len, next));
	} else {
	    let total_packets = bin(&x[7..18]);
	    println!("packet len {}", total_packets);
	    // length of subpackets
	    let mut l: usize = 0;
	    let mut next = &x[18..];
	    let mut packet = Packet::op(version, id);
	    for _ in 0..total_packets {
		let (p, len, n) = parse_packet(next).unwrap();
		packet.sub_packets.push(p);
		next = n;
		l += len;
	    }

	    return Some((packet, 18+l, next));
	}
    }

    return None;
}

pub fn day16() {
    let lines = read_lines("input/day16.txt", true).unwrap();
    let packet = &lines[0];
    println!("{:04b}", 2);

    let mut  binary = vec![];
    for p in packet.as_bytes() {
	binary.extend(format!("{:04b}", hex(&[*p])).as_bytes());
    }
    println!("{}", String::from_utf8_lossy(&binary));
    let (packet, _, _) = parse_packet(&binary).unwrap();

    println!("{}", packet.sum_versions());
    println!("{}", packet.interp());


}
