//! Link: https://adventofcode.com/2019/day/10
//! Day 10: Monitoring Station
//! 
//! You fly into the asteroid belt and reach the Ceres monitoring station.
//! The Elves here have an emergency: 
//!  they're having trouble tracking all of the asteroids and can't be sure they're safe.
//! 
//! The Elves would like to build a new monitoring station in a nearby area of space; 
//!  they hand you a map of all of the asteroids in that region (your puzzle input).

use std::collections::{HashSet, BTreeMap};
use std::fmt;

// The map indicates whether each position is empty (.) or contains an asteroid (#).
// The asteroids are much smaller than they appear on the map, 
//  and every asteroid is exactly in the center of its marked position.
// The asteroids can be described with X,Y coordinates where X is the
//  distance from the left edge and Y is the distance from the top edge
//  (so the top-left corner is 0,0 and the position immediately to its right is 1,0).
#[aoc_generator(day10)]
fn input_generator(s: &str) -> Vec<Asteroid> {
    s.lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' {Some(Asteroid(x, y))} else {None})
            })
        .collect()
}

#[derive(Clone, Debug)]
struct Asteroid(usize, usize);

impl Asteroid {
    pub fn angle(&self, other: &Self) -> isize {
        -((self.0 as f64 - other.0 as f64).atan2(self.1 as f64 - other.1 as f64) * 1000.0) as isize
    }

    pub fn dist(&self, other: &Self) -> usize {
        ((self.0 as isize - other.0 as isize).abs() + (self.1 as isize - other.1 as isize).abs()) as usize
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl fmt::Display for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as usize * 100 + self.1 as usize)
    }
}

fn best_position(map: &[Asteroid]) -> Option<(&Asteroid, usize)> {
    map.iter().map(|a| {
        let b: HashSet<isize> = map.iter()
            .filter(|aa| a != *aa)
            .map(|aa| a.angle(aa))
            .collect();
        (a, b.len())
    }).max_by_key(|(_, c)| *c)
}

// Your job is to figure out which asteroid would be the
//  best place to build a new monitoring station.
// A monitoring station can detect any asteroid to which 
//  it has direct line of sight - that is, 
//  there cannot be another asteroid exactly between them. 
// This line of sight can be at any angle,
//  not just lines aligned to the grid or diagonally.
// The best location is the asteroid that 
//  can detect the largest number of other asteroids.
//
// Find the best location for a new monitoring station. 
// How many other asteroids can be detected from that location?
// 
// Your puzzle answer was 280.
#[aoc(day10, part1, Math)]
fn solve_part1_math(a: &[Asteroid]) -> Option<usize> {
    match best_position(a) {
        Some((_, c)) => Some(c),
        None => None,
    }
}

#[derive(Debug)]
struct Station{
    platform: Asteroid,
    pub map: BTreeMap<isize, Vec<Asteroid>>,
}

impl From<(Vec<Asteroid>, &Asteroid)> for Station {
    fn from((map, platform): (Vec<Asteroid>, &Asteroid)) -> Self { 
        let mut m: BTreeMap<isize, Vec<Asteroid>> = map.iter()
            .filter(|aa| platform != *aa)
            .map(|a| (a, platform.angle(a)))
            .fold(BTreeMap::new(), |mut map, (a, angle)| {
                map.entry(angle).or_default().push(a.clone()); map
            });
        m.iter_mut().for_each(|(_, v)| v.sort_by_key(|a| platform.dist(a)));
        Self{
            map: m,
            platform: platform.clone(),
        }
    }
}

impl Station {
    fn resolve_bet<'a>(&'a mut self, nth: usize) -> Option<Asteroid> {
        let c = self.map.clone();
        c.keys()
            .cloned()
            .cycle()
            .skip_while(|angle| *angle < 0)
            .filter_map(|ref angle| {
                self.map.get_mut(angle).and_then(|a| a.pop())
            })
            .skip(nth -1)
            .next()
    }
}

// Once you give them the coordinates, the Elves quickly deploy an
//  Instant Monitoring Station to the location and discover the worst: 
//  there are simply too many asteroids.
//
// The only solution is complete vaporization by giant laser.
//
// Fortunately, in addition to an asteroid scanner, 
//  the new monitoring station also comes equipped with a
//  giant rotating laser perfect for vaporizing asteroids.
// The laser starts by pointing up and always rotates clockwise,
//  vaporizing any asteroid it hits.
// 
// If multiple asteroids are exactly in line with the station,
//  the laser only has enough power to vaporize one of them before continuing its rotation.
// In other words, the same asteroids that can be detected can be vaporized,
//  but if vaporizing one asteroid makes another one detectable,
//  the newly-detected asteroid won't be vaporized until the 
//  laser has returned to the same position by rotating a full 360 degrees.
// 
// The Elves are placing bets on which will be the 200th asteroid to be vaporized. 
// Win the bet by determining which asteroid that will be; 
//  what do you get if you multiply its X coordinate by 100 and then add its Y coordinate? 
//
// Your puzzle answer was 706.
#[aoc(day10, part2, Math)]
fn solve_part2_math(a: &[Asteroid]) -> Option<Asteroid> {
    let mut s = Station::from((a.to_vec(), best_position(a).unwrap().0));
    s.resolve_bet(200)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_map() {
        let a = input_generator(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");

        let best = best_position(&a);
        assert_eq!(best, Some((&Asteroid(11, 13), 210)));
        let mut s = Station::from((a.clone(), best.unwrap().0));
        assert_eq!(s.resolve_bet(200), Some(Asteroid(8, 2)));
    }
}