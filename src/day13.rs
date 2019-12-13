//! Link: https://adventofcode.com/2019/day/13
//! Day 13: Care Package
//! 
//! As you ponder the solitude of space and the ever-increasing 
//!  three-hour roundtrip for messages between you and Earth, 
//!  you notice that the Space Mail Indicator Light is blinking. 
//! To help keep you sane, the Elves have sent you a care package.
//! 
//! It's a new game for the ship's arcade cabinet! 
//! Unfortunately, the arcade is all the way on the other end of the ship. 
//! Surely, it won't be hard to build your own 
//!  - the care package even comes with schematics.

use std::cmp::Ordering;
use crate::common::intcode::{vm::Intcode, error::IntcodeError};
use failure::Error;

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Intcode {
    Intcode::from(input)
}

// The arcade cabinet runs Intcode software like the 
//  game the Elves sent (your puzzle input).
// It has a primitive screen capable of drawing square tiles on a grid. 
// The software draws tiles to the screen with output instructions: 
//  every three output instructions specify the
//  x position (distance from the left),
//  y position (distance from the top),
//  and tile id. 
// 
// The tile id is interpreted as follows: 
//  0 is an empty tile. No game object appears in this tile.
//  1 is a wall tile. Walls are indestructible barriers.
//  2 is a block tile. Blocks can be broken by the ball.
//  3 is a horizontal paddle tile. The paddle is indestructible.
//  4 is a ball tile. The ball moves diagonally and bounces off objects.
// 
// Start the game. How many block tiles are on the screen when the game exits?
// 
// Your puzzle answer was 270.
#[aoc(day13, part1, Base)]
fn solve_part1_base(vm: &Intcode) -> usize {
    let all: Vec<_> = vm.clone().collect::<Result<_, IntcodeError>>().unwrap();
    all.chunks(3)
        .map(|chunk| chunk[2])
        .filter(|&tile| tile == 2)
        .count()
}

// The game didn't run because you didn't put in any quarters. 
// Unfortunately, you did not bring any quarters. 
// Memory address 0 represents the number of quarters that have been inserted; 
//  set it to 2 to play for free.
// 
// The arcade cabinet has a joystick that can move left and right. 
// The software reads the position of the joystick with input instructions:
//   If the joystick is in the neutral position, provide 0.
//   If the joystick is tilted to the left, provide -1.
//   If the joystick is tilted to the right, provide 1.
// 
// The arcade cabinet also has a segment display capable of 
//  showing a single number that represents the player's current score. 
// When three output instructions specify X=-1, Y=0, 
//  the third output instruction is not a tile; 
//  the value instead specifies the new score to show in the segment display.
// 
// Beat the game by breaking all the blocks.
// What is your score after the last block is broken?
// 
// Your puzzle answer was 12535.
#[aoc(day13, part2, Base)]
fn solve_part2_base(vm: &Intcode) -> Result<i64, Error> {
    let mut local_vm = vm.clone();
    local_vm.memory_mut().insert(0, 2);

    let mut score = 0;
    let mut ballx = 0;
    let mut paddlex = 0;

    loop {
        let mut tiles = Vec::new();
        while let Some(Ok(v)) = local_vm.next() {
            tiles.push(v);
        }

        for chunk in tiles.chunks(3) {
            let (x, _, tile) = (chunk[0], chunk[1], chunk[2]);
            if x == -1 {
                score = tile;
            } else if tile == 3 {
                paddlex = x;
            } else if tile == 4 {
                ballx = x;
            }
        }
        
        match local_vm.next() {
            Some(Err(IntcodeError::NeedsInput)) => {
                local_vm.inputs.push_back(
                    match ballx.cmp(&paddlex) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    }
                )
            }
            None => {
                break;
            }
            Some(Err(e)) => {
                return Err(Error::from(e));
            }
            _ => unreachable!(),
        }
    }

    Ok(score)
}