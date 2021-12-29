use aoc2021::read_grid;
use ndarray as nd;
use nd::prelude::*;

const RIGHT: u8 = b'>';
const DOWN: u8 = b'v';
const EMPTY: u8 = b'.';

fn day25_process_right(grid: &mut Array2<u8>) -> usize {
    let mut num_moved = 0;
    let d = grid.dim();
    let mut moved = Array2::from_elem(grid.dim(), false);
    
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if grid[(i, j)] == RIGHT && grid[(i, (j+1) % d.1)] == EMPTY {
		moved[(i, j)] = true;
		num_moved += 1
	    }
	}
    }
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if moved[(i, j)] {
		grid[(i, (j+1)%d.1)] = RIGHT;
		grid[(i, j)] = EMPTY;
	    }
	}
    }
    num_moved
}

fn day25_process_down(grid: &mut Array2<u8>) -> usize {
    let mut num_moved = 0;
    let d = grid.dim();
    let mut moved = Array2::from_elem(grid.dim(), false);
    
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if grid[(i, j)] == DOWN && grid[((i+1) % d.0, j)] == EMPTY {
		moved[(i, j)] = true;
		num_moved += 1
	    }
	}
    }
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if moved[(i, j)] {
		grid[((i+1)%d.0, j)] = DOWN;
		grid[(i, j)] = EMPTY;
	    }
	}
    }
    num_moved
}


fn day25() {
    let mut grid = read_grid("input/day25.txt");

    let mut num_steps = 1;
    loop {
	let mut s = day25_process_right(&mut grid);
	s += day25_process_down(&mut grid);
	if s == 0 {
	    break;
	}
	num_steps += 1;
    }
    println!("{}", num_steps);
}


fn main() -> std::io::Result<()> {
    aoc2021::day01();
    aoc2021::day02();
    aoc2021::day03();
    aoc2021::day04();
    aoc2021::day05();
    aoc2021::day06();
    aoc2021::day07();
    aoc2021::day08();
    aoc2021::day09();
    aoc2021::day10();
    aoc2021::day11();
    aoc2021::day12();
    aoc2021::day13();
    aoc2021::day14();
    aoc2021::day15();
    aoc2021::day16();
    aoc2021::day17();
    aoc2021::day18();
    aoc2021::day19();
    aoc2021::day20();
    aoc2021::day21();
    aoc2021::day22();
    aoc2021::day23();
    aoc2021::day24();
    day25();
    Ok(())
}
