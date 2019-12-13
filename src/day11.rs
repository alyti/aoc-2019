//! Link: https://adventofcode.com/2019/day/11
//! Day 11: Space Police
//! 
//! On the way to Jupiter, you're pulled over by the Space Police.
//! 
//! Not wanting to be sent to Space Jail, 
//!  you radio back to the Elves on Earth for help.
//! Although it takes almost three hours for their reply signal to reach you,
//!  they send instructions for how to power up the emergency 
//!  hull painting robot and even provide a small
//!  Intcode program (your puzzle input) that will 
//!  cause it to paint your ship appropriately.
//! 
//! There's just one problem: you don't have an emergency hull painting robot.

use std::collections::HashMap;
use crate::common::intcode_old::IntcodeVM;
use std::str::FromStr;
use failure::Error;

#[derive(Debug)]
enum Direction {
    Up, 
    Down,
    Left,
    Right, 
}

impl Direction {
    fn turn_left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply_direction(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Painter {
    canvas: HashMap<Point, bool>,
    vm: IntcodeVM,
    dir: Direction,
    cur: Point,
}

impl From<IntcodeVM> for Painter {
    fn from(vm: IntcodeVM) -> Self { 
        Self{
            canvas: HashMap::new(),
            vm: vm,
            dir: Direction::Up,
            cur: Point{x: 0, y: 0},
        }
    }
}

impl Painter {
    pub fn canvas(&self) -> HashMap<Point, bool> {
        self.canvas.clone()
    }

    pub fn canvas_mut(&mut self) -> &mut HashMap<Point, bool> {
        &mut self.canvas
    }

    pub fn bot_position(&self) -> Point {
        self.cur
    }

    fn execute(&mut self) -> Result<(), Error> {
        let mut local_vm = self.vm.clone();
        let (input, output) = local_vm.io();
        let bot_thread = std::thread::spawn(move || {
            local_vm.execute()
        });

        loop {
            let color = self.canvas.get(&self.cur).copied().unwrap_or(false);
            input.send(Some(color as i64))?;
            if let Ok(Some(new_color)) = output.recv() {
                self.canvas.insert(self.cur, new_color != 0);
                if let Ok(Some(new_dir)) = output.recv() {
                    self.dir = match new_dir {
                        0 => self.dir.turn_left(),
                        1 => self.dir.turn_right(),
                        _ => panic!("shouldn't happen..."),
                    };
                    self.cur.apply_direction(&self.dir);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        bot_thread.join().expect("couldn't join vm thread")?;
        Ok(())
    }

    pub fn canvas_bounds(&self) -> (Point, Point) {
        let min_x = self.canvas.keys().map(|&p| p.x).min().unwrap_or(0);
        let max_x = self.canvas.keys().map(|&p| p.x).max().unwrap_or(0);
        let min_y = self.canvas.keys().map(|&p| p.y).min().unwrap_or(0);
        let max_y = self.canvas.keys().map(|&p| p.y).max().unwrap_or(0);
        (Point{x: min_x, y: min_y}, Point{x: max_x, y: max_y})
    }

    pub fn dump_canvas(&self) -> String {
        let bounds = self.canvas_bounds();
        let mut dump = String::new();
        for y in bounds.0.y..=bounds.1.y {
            dump.push('\n');
            for x in bounds.0.x..=bounds.1.x {
                let s = match self.canvas.get(&Point{x,y}).copied().unwrap_or(false) {
                    false => '■',
                    true  => '□',
                };
                dump.push(s);
            }
        }
        dump
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> IntcodeVM {
    IntcodeVM::from_str(input).unwrap()
}

// You'll need to build a new emergency hull painting robot. 
// The robot needs to be able to move around on the grid 
//  of square panels on the side of your ship,
//  detect the color of its current panel,
//  and paint its current panel black or white. 
// (All of the panels are currently black.)
// 
// The Intcode program will serve as the brain of the robot.
// The program uses input instructions to access the robot's camera: 
//  provide 0 if the robot is over a black panel 
//  or 1 if the robot is over a white panel. 
// Then, the program will output two values:
// - First, it will output a value indicating the color to paint the panel 
//    the robot is over: 0 means to paint the panel black,
//    and 1 means to paint the panel white.
// - Second, it will output a value indicating the direction the robot
//    should turn: 0 means it should turn left 90 degrees,
//    and 1 means it should turn right 90 degrees.
// 
// After the robot turns, it should always move forward exactly one panel. 
// The robot starts facing up.
// 
// The robot will continue running for a while like this and halt when it 
// is finished drawing. 
// Do not restart the Intcode computer inside the robot during this process.
// 
// Build a new emergency hull painting robot and run the Intcode program on it.
// How many panels does it paint at least once?
// 
// Your puzzle answer was 1894.
#[aoc(day11, part1, IntcodeVM)]
fn solve_part1_intcode(vm: &IntcodeVM) -> Result<usize, Error> {
    let mut p = Painter::from(vm.clone());
    p.execute()?;
    Ok(p.canvas().len())
}

// You're not sure what it's trying to paint,
//  but it's definitely not a registration identifier. 
// The Space Police are getting impatient.
// 
// Checking your external ship cameras again,
//  you notice a white panel marked "emergency hull painting robot starting panel".
// The rest of the panels are still black,
//  but it looks like the robot was expecting to start on a white panel,
//  not a black one.
// 
// Based on the Space Law Space Brochure that the
//  Space Police attached to one of your windows,
//  a valid registration identifier is always eight capital letters.
// After starting the robot on a single white panel instead, 
//  what registration identifier does it paint on your hull?
// 
// Your puzzle answer was JKZLZJBH.
#[aoc(day11, part2, IntcodeVM)]
fn solve_part2_intcode(vm: &IntcodeVM) -> Result<String, Error> {
    let mut p = Painter::from(vm.clone());
    let bot = p.bot_position();
    p.canvas_mut().entry(bot).or_insert(true);
    p.execute()?;
    Ok(p.dump_canvas())
}
