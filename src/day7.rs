/*
 * Link: https://adventofcode.com/2019/day/7
 * Day 7: Amplification Circuit
 *
 * Based on the navigational maps,
 *  you're going to need to send more power to your ship's thrusters to reach Santa in time. 
 * To do this, you'll need to configure a series of amplifiers already installed on the ship.
*/

use std::str::FromStr;
use crate::util::intcode::IntcodeVM;
use permutohedron::Heap;

#[aoc(day7, part1, Map)]
fn solve_part1_map(input: &str) -> Option<i32> {
    let vm = IntcodeVM::from_str(input).unwrap();
    Heap::new(&mut vec![0, 1, 2, 3, 4]).collect::<Vec<Vec<i32>>>().iter()
        .map(|stages| {
            let mut power = 0;
            for stage in stages.iter() {
                power = *vm.clone()
                    .simple_input(vec![*stage, power])
                    .execute_and_collect().unwrap().last().unwrap();
            }
            power
        })
        .max()
}

#[aoc(day7, part2, Map)]
fn solve_part2_map(input: &str) -> Option<i32> {
    let vm = IntcodeVM::from_str(input).unwrap();
    Heap::new(&mut vec![5, 6, 7, 8, 9]).collect::<Vec<Vec<i32>>>().iter()
        .map(|stages| {
            let mut rx = std::collections::VecDeque::new();
            let mut tx = std::collections::VecDeque::new();
            for _ in 0..=5 {
                let (t, r) = std::sync::mpsc::channel();
                rx.push_back(r);
                tx.push_back(t);
            }
            for (stage, t) in stages.iter().zip(tx.iter()) {
                t.send(*stage).unwrap();
            }
            let txa = tx.pop_front().unwrap();
            let rxe = rx.pop_back().unwrap();
            txa.send(0).unwrap();
            for (r, t) in rx.into_iter().zip(tx.into_iter()) {
                let mut amp = vm.clone();
                amp.rx(r).tx(t);
                std::thread::spawn(move || amp.execute());
            }
            let mut last = -1;
            for z in rxe {
                last = z;
                let _ = txa.send(z);
            }
            last
        })
        .max()
}

