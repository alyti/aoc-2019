//! Link: https://adventofcode.com/2019/day/12
//! Day 12: The N-Body Problem
//! 
//! The space near Jupiter is not a very safe place; 
//!  you need to be careful of a big distracting red spot,
//!  extreme radiation, and a whole lot of moons swirling around.
//! You decide to start by tracking the four largest moons: 
//!  Io, Europa, Ganymede, and Callisto.
//! 
//! After a brief scan, you calculate the position of each moon (your puzzle input). 
//! You just need to simulate their motion so you can avoid them.

use std::str::FromStr;
use pest::{Parser, error::Error};
use num_integer::lcm;

#[derive(Parser)]
#[grammar = "parsers/moons.pest"]
struct MoonParser;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Vec3{
    x: i64, 
    y: i64,
    z: i64,
}

const MOON_NAMES: [&str; 4] = ["Io", "Europa", "Ganymede", "Callisto"];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Moon{
    name: String,
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn energy(&self) -> u64 {
        let pot = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kin = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        (pot * kin) as u64
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Moons(Vec<Moon>);

impl FromStr for Moons {
    type Err = Error<Rule>;
    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        let pairs = MoonParser::parse(Rule::moon_list, s.trim())?
            .next()
            .unwrap();

        let moons: Vec<Moon> = pairs
            .into_inner()
            .filter(|pair| pair.as_rule() == Rule::moon)
            .enumerate()
            .map(|(i, pair)| {
                let mut moon = pair.into_inner();
                let x: i64 = moon.next().unwrap().as_str().parse().unwrap();
                let y: i64 = moon.next().unwrap().as_str().parse().unwrap();
                let z: i64 = moon.next().unwrap().as_str().parse().unwrap();
                Moon {
                    name: MOON_NAMES[i % MOON_NAMES.len()].to_string(),
                    pos: Vec3{x,y,z},
                    vel: Vec3::default(),
                }
            })
            .collect();
        Ok(Moons(moons))
    }
}

impl Moons {
    fn update_velocity_for(&mut self, a: usize, b: usize) {
        if self.0[a].pos.x < self.0[b].pos.x {
            self.0[a].vel.x += 1;
            self.0[b].vel.x -= 1;
        }
        if self.0[a].pos.y < self.0[b].pos.y {
            self.0[a].vel.y += 1;
            self.0[b].vel.y -= 1;
        }
        if self.0[a].pos.z < self.0[b].pos.z {
            self.0[a].vel.z += 1;
            self.0[b].vel.z -= 1;
        }
    }
}

fn simulator(moons: Moons) -> impl Iterator<Item = Moons> {
    std::iter::successors(Some(moons), |moons| {
        let mut next_state = moons.clone();
        for a in 0..moons.0.len() {
            for b in 0..moons.0.len() {
                next_state.update_velocity_for(a, b);
            }
        }
        for m in &mut next_state.0 {
            m.update_position();
        }
        Some(next_state)
    })
}

// Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional velocity.
// The position of each moon is given in your scan; the x, y, and z velocity of each moon starts at 0.
#[aoc_generator(day12)]
fn input_generator(input: &str) -> Result<Moons, Error<Rule>> {
    Moons::from_str(input)
}

// Simulate the motion of the moons in time steps. 
// Within each time step, first update the velocity of every moon by applying gravity. 
// Then, once all moons' velocities have been updated, 
//  update the position of every moon by applying velocity.
// Time progresses by one step once all of the positions are updated.
// 
// To apply gravity, consider every pair of moons. 
// On each axis (x, y, and z), the velocity of each moon changes by 
//  exactly +1 or -1 to pull the moons together. 
// However, if the positions on a given axis are the same,
//  the velocity on that axis does not change for that pair of moons.
// 
// Once all gravity has been applied, apply velocity: 
//  simply add the velocity of each moon to its own position.
// 
// Then, it might help to calculate the total energy in the system. 
// The total energy for a single moon is its potential energy multiplied by its kinetic energy. 
// A moon's potential energy is the sum of the absolute values of its x, y, and z position coordinates. 
// A moon's kinetic energy is the sum of the absolute values of its velocity coordinates.
// 
// What is the total energy in the system after simulating the moons given in your scan for 1000 steps?
// 
// Your puzzle answer was 9958.
#[aoc(day12, part1, Base)]
fn solve_part1_base(m: &Moons) -> u64 {
    let result = simulator(m.clone()).skip(1000).next().unwrap();
    result
        .0
        .iter()
        .map(|m| m.energy())
        .sum()
}


fn r(input: &Moons, x: impl Fn(&Vec3) -> i64) -> usize {
    simulator(input.clone())
        .enumerate()
        .skip(1)
        .filter(|(_, moons)| {
            input.0.iter().zip(moons.0.iter()).all(|(a, b)| {
                x(&a.pos) == x(&b.pos) && x(&a.vel) == x(&b.vel)
            })
        })
        .next()
        .unwrap()
        .0
}

// All this drifting around in space makes you wonder about the nature of the universe.
// Does history really repeat itself? 
// You're curious whether the moons will ever return to a previous state.
// 
// Determine the number of steps that must occur before all of 
// the moons' positions and velocities exactly match a previous point in time.
// 
// How many steps does it take to reach the first state that exactly matches a previous state?
// 
// Your puzzle answer was 318382803780324.
#[aoc(day12, part2)]
fn part2(input: &Moons) -> usize {
    lcm(r(&input, |v| v.x), lcm(r(&input, |v| v.y), r(&input, |v| v.z)))
}