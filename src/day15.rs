//! Link: https://adventofcode.com/2019/day/14
//! Day 14: Oxygen System
//! 
//! Out here in deep space, many things can go wrong.
//! Fortunately, many of those things have indicator lights.
//! Unfortunately, one of those lights is lit: 
//!  the oxygen system for part of the ship has failed!
//! 
//! According to the readouts, the oxygen system must have failed
//!  days ago after a rupture in oxygen tank two; 
//!  that section of the ship was automatically 
//!  sealed once oxygen levels went dangerously low.
//! A single remotely-operated repair droid is your 
//!  only option for fixing the oxygen system.
//! 
//! The Elves' care package included an Intcode program (your puzzle input) 
//!  that you can use to remotely control the repair droid.
//! By running that program, you can direct the repair droid 
//!  to the oxygen system and fix the problem.

use crate::common::intcode::{vm::Intcode};
use std::collections::HashMap;

const NORTH: i8 = 1;
const SOUTH: i8 = 2;
const WEST: i8 = 3;
const EAST: i8 = 4;

const DIRECTIONS: [i8; 4] = [NORTH, SOUTH, EAST, WEST];

const WALL: i8 = 1;
const OXYGEN: i8 = 2;
const SPACE: i8 = 3;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Intcode {
    Intcode::from(input)
}

// The remote control program executes the following steps in a loop forever:
// 
// Accept a movement command via an input instruction.
// Send the movement command to the repair droid.
// Wait for the repair droid to finish the movement operation.
// Report on the status of the repair droid via an output instruction.
// 
// Only four movement commands are understood:
//  north (1), south (2), west (3), and east (4). 
// Any other command is invalid. 
// The movements differ in direction, but not in distance: 
//  in a long enough east-west hallway, 
//  a series of commands like 4,4,4,4,3,3,3,3 
//  would leave the repair droid back where it started.
// 
// The repair droid can reply with any of the following status codes:
//  - 0: The repair droid hit a wall. Its position has not changed.
//  - 1: The repair droid has moved one step in the requested direction.
//  - 2: The repair droid has moved one step in the requested direction; 
//       its new position is the location of the oxygen system.
// 
// You don't know anything about the area around the repair droid, 
// but you can figure it out by watching the status codes.
// 
// What is the fewest number of movement commands required to
//  move the repair droid from its starting position to the location of the oxygen system?
// 
// Your puzzle answer was 308.
#[aoc(day15, part1, Base)]
fn solve_part1_base(vm: &Intcode) -> usize {
    let mut all_vms = vec![(vm.clone(), (0, 0))];
    let mut map: HashMap<(i64, i64), i8> = HashMap::new();
    let mut steps = None;
    'a: for i in 1.. {
        for (prog, pos) in all_vms.drain(..).collect::<Vec<_>>().into_iter() {
            for &dir in DIRECTIONS.iter() {
                let new_pos = translate(&pos, dir);
                if map.contains_key(&new_pos) {
                    continue;
                }
                let mut new = prog.clone();
                new.inputs.push_back(dir.into());
                match new.next().unwrap().unwrap() {
                    0 => {
                        map.insert(new_pos, WALL);
                    }
                    1 => {
                        map.insert(new_pos, SPACE);
                        all_vms.push((new, new_pos));
                    }
                    2 => {
                        map.insert(new_pos, OXYGEN);
                        steps = Some(i);
                        break 'a;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    steps.unwrap()
}

// You quickly repair the oxygen system; oxygen gradually fills the area.
// 
// Oxygen starts in the location containing the repaired oxygen system.
// It takes one minute for oxygen to spread to all open 
//  locations that are adjacent to a location that already contains oxygen.
// Diagonal locations are not adjacent.
// 
// Use the repair droid to get a complete map of the area. 
// How many minutes will it take to fill with oxygen.
// 
// Your puzzle answer was 328.
#[aoc(day15, part2, Base)]
fn solve_part2_base(vm: &Intcode) -> usize {
    let mut all_vms = vec![(vm.clone(), (0, 0))];
    let mut map: HashMap<(i64, i64), i8> = HashMap::new();
    let mut goal_prog = None;
    'a: for _ in 1.. {
        for (prog, pos) in all_vms.drain(..).collect::<Vec<_>>().into_iter() {
            for &dir in DIRECTIONS.iter() {
                let new_pos = translate(&pos, dir);
                if map.contains_key(&new_pos) {
                    continue;
                }
                let mut new = prog.clone();
                new.inputs.push_back(dir.into());
                match new.next().unwrap().unwrap() {
                    0 => {
                        map.insert(new_pos, WALL);
                    }
                    1 => {
                        map.insert(new_pos, SPACE);
                        all_vms.push((new, new_pos));
                    }
                    2 => {
                        map.insert(new_pos, OXYGEN);
                        goal_prog = Some((new, new_pos));
                        break 'a;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    all_vms = vec![(goal_prog.unwrap())];

    map = map.into_iter().filter(|(_k, v)| *v != SPACE.into()).collect();

    let mut minutes = None;
    for i in 0.. {
        if all_vms.is_empty() {
            minutes = Some(i - 1);
            break;
        }
        for (prog, pos) in all_vms.drain(..).collect::<Vec<_>>().into_iter() {
            for &dir in DIRECTIONS.iter() {
                let new_pos = translate(&pos, dir);
                if map.contains_key(&new_pos) {
                    continue;
                }
                let mut new = prog.clone();
                new.inputs.push_back(dir.into());
                match new.next().unwrap().unwrap() {
                    0 => {
                        map.insert(new_pos, WALL);
                    }
                    1 => {
                        map.insert(new_pos, SPACE);
                        all_vms.push((new, new_pos));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    minutes.unwrap()
}

fn translate(pos: &(i64, i64), direction: i8) -> (i64, i64) {
    match direction {
        NORTH => (pos.0 - 1, pos.1),
        SOUTH => (pos.0 + 1, pos.1),
        WEST => (pos.0, pos.1 - 1),
        EAST => (pos.0, pos.1 + 1),
        _ => unreachable!(),
    }
}