/*
 * Link: https://adventofcode.com/2019/day/3
 * Day 3: Crossed Wires 
 * 
 * The gravity assist was successful, and you're well on your way to the Venus refuelling station.
 * During the rush back on Earth, the fuel management system wasn't completely installed,
 *  so that's next on the priority list.
*/

#[derive(Debug)]
enum Direction {
    Up, 
    Down,
    Left,
    Right, 
    Unknown
}

#[derive(Debug)]
struct Command {
    pub direction: Direction,
    pub distance: u32,
}

#[derive(Debug)]
struct Wire{
    pub cmnds: Vec<Command>,
}

fn iter_wire<'a>(wire: &'a Wire) -> impl Iterator<Item = (i32, i32)> + 'a {
	gen_iter::GenIter(move || {
		let mut x = 0;
		let mut y = 0;
		for s in &wire.cmnds {
			for _ in 0..s.distance {
				match s.direction {
					Direction::Up => {
						y -= 1;
					}
					Direction::Down => {
						y += 1;
					}
					Direction::Left => {
						x -= 1;
					}
					Direction::Right => {
						x += 1;
                    },
                    Direction::Unknown => (),
				}
				yield (x, y);
			}
		}
	})
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Wire> {
    input.split_whitespace().map(|line| {
        Wire{
            cmnds: line.split_terminator(",").map(|command| {
            let mut chars = command.chars();
            Command{
                direction: match chars.next() {
                    Some('U') => Direction::Up,
                    Some('D') => Direction::Down,
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right, 
                    Some(_) => Direction::Unknown,
                    None => Direction::Unknown,
                },
                distance: chars.as_str().parse::<u32>().unwrap_or(0),
            }
        }).collect()}
    }).collect()
}

/*
 * Opening the front panel reveals a jumble of wires. 
 * Specifically, two wires are connected to a central port and extend outward on a grid. 
 * You trace the path each wire takes as it leaves the central port,
 *  one wire per line of text (your puzzle input).
 * 
 * The wires twist and turn, but the two wires occasionally cross paths.
 * To fix the circuit, you need to find the intersection point closest to the central port. 
 * Because the wires are on a grid, use the Manhattan distance for this measurement. 
 * While the wires do technically cross right at the central port where they both start,
 *  this point does not count, nor does a wire count as crossing with itself.
*/
#[aoc(day3, part1, Generators)]
fn solve_part1_gen(input: &[Wire]) -> Option<i32> {
    let mut grid = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
	for (x, y) in iter_wire(&input[0]) {
		grid.entry(y).or_default().insert(x);
	}
	let mut cross = Vec::new();
	for (x, y) in iter_wire(&input[1]) {
		if let Some(row) = grid.get(&y) {
			if row.contains(&x) {
				cross.push(x.abs() + y.abs());
			}
		}
	}
	cross.into_iter().filter(|d| d > &0).min()
}

/*
 * It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
 * 
 * To do this, calculate the number of steps each wire takes to reach each intersection;
 *  choose the intersection where the sum of both wires' steps is lowest. 
 * If a wire visits a position on the grid multiple times,
 *  use the steps value from the first time it visits that position when
 *  calculating the total value of a specific intersection.
*/
#[aoc(day3, part2, Generators)]
fn solve_part2_gen(input: &[Wire]) -> Option<usize> {
	let mut grid = std::collections::HashMap::<i32, std::collections::HashMap<i32, usize>>::new();
	for (dist, (x, y)) in iter_wire(&input[0]).enumerate() {
		grid.entry(y).or_default().insert(x, dist);
	}
	let mut cross = Vec::new();
	for (dist, (x, y)) in iter_wire(&input[1]).enumerate() {
		if let Some(row) = grid.get(&y) {
			if let Some(other) = row.get(&x) {
				cross.push(dist + other);
			}
		}
	}
	cross.into_iter().filter(|d| d > &0).min().map(|d| d + 2)
}
#[cfg(test)]
mod tests {
    use super::*;

    // R75,D30,R83,U83,L12,D49,R71,U7,L72
    // U62,R66,U55,R34,D71,R55,D58,R83 
    // Distance = 159, Steps = 610
    #[test]
    fn day3_example1() {
        let input = &input_generator(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(solve_part1_gen(input), Some(159));
        assert_eq!(solve_part2_gen(input), Some(610));
    }

    // R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    // U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
    // Distance = 135, Steps = 410
    #[test]
    fn day3_example2() {
        let input = &input_generator(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(solve_part1_gen(input), Some(135));
        assert_eq!(solve_part2_gen(input), Some(410));
    }
}