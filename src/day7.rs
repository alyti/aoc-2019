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

#[aoc_generator(day7)]
fn input_generator(input: &str) -> IntcodeVM {
    IntcodeVM::from_str(input).unwrap()
}

/* 
 * There are five amplifiers connected in series;
 *  each one receives an input signal and produces an output signal.
 * They are connected such that the first amplifier's output leads to the second amplifier's input,
 *  the second amplifier's output leads to the third amplifier's input, and so on.
 * The first amplifier's input value is 0,
 *  and the last amplifier's output leads to your ship's thrusters.
 * 
 * The Elves have sent you some Amplifier Controller Software (your puzzle input), 
 *  a program that should run on your existing Intcode computer. 
 * Each amplifier will need to run a copy of the program.
 * 
 * When a copy of the program starts running on an amplifier,
 *  it will first use an input instruction to ask the amplifier for 
 *  its current phase setting (an integer from 0 to 4).
 * Each phase setting is used exactly once,
 *  but the Elves can't remember which amplifier needs which phase setting.
 * 
 * The program will then call another input instruction to get the amplifier's input signal,
 *  compute the correct output signal, and supply it back to the amplifier with an output instruction.
 *  (If the amplifier has not yet received an input signal, it waits until one arrives.)
 * 
 * Your job is to find the largest output signal that can be sent to
 *  the thrusters by trying every possible combination of phase settings on the amplifiers.
 * Make sure that memory is not shared or reused between copies of the program.
*/
#[aoc(day7, part1, Map)]
fn solve_part1_map(vm: &IntcodeVM) -> Option<i64> {
    Heap::new(&mut vec![0, 1, 2, 3, 4]).collect::<Vec<Vec<i64>>>().iter()
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

/*
 * It's no good - in this configuration,
 * the amplifiers can't generate a large enough output signal to produce the thrust you'll need.
 * The Elves quickly talk you through rewiring the amplifiers into a feedback loop.
 * 
 * Most of the amplifiers are connected as they were before;
 *  amplifier A's output is connected to amplifier B's input, and so on.
 * However, the output from amplifier E is now connected into amplifier A's input.
 * This creates the feedback loop: the signal will be sent through the amplifiers many times.
 * 
 * In feedback loop mode, the amplifiers need totally different phase settings: 
 *  integers from 5 to 9, again each used exactly once.
 * These settings will cause the Amplifier Controller Software to repeatedly take input
 *  and produce output many times before halting.
 * Provide each amplifier its phase setting at its first input instruction; 
 *  all further input/output instructions are for signals.
 * 
 * Don't restart the Amplifier Controller Software on any amplifier during this process.
 * Each one should continue receiving and sending signals until it halts.
 * 
 * All signals sent or received in this process will be between pairs of
 *  amplifiers except the very first signal and the very last signal.
 * To start the process, a 0 signal is sent to amplifier A's input exactly once.
 * 
 * Eventually, the software on the amplifiers will halt after they have processed the final loop.
 * When this happens, the last output signal from amplifier E is sent to the thrusters.
 * Your job is to find the largest output signal that can be sent to
 *  the thrusters using the new phase settings and feedback loop arrangement.
*/
#[aoc(day7, part2, Map)]
fn solve_part2_map(vm: &IntcodeVM) -> Option<i64> {
    Heap::new(&mut vec![5, 6, 7, 8, 9]).collect::<Vec<Vec<i64>>>().iter()
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
