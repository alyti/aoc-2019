//! Link: https://adventofcode.com/2019/day/5
//! Day 5: Sunny with a Chance of Asteroids
//!
//! You're starting to sweat as the ship makes its way toward Mercury.
//! The Elves suggest that you get the air conditioner working
//!  by upgrading your ship computer to support
//!  the Thermal Environment Supervision Terminal.
//! The Thermal Environment Supervision Terminal (TEST)
//!  starts by running a diagnostic program (your puzzle input).

use failure::Error;
use std::str::FromStr;
use crate::common::intcode_old::IntcodeVM;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> IntcodeVM {
    IntcodeVM::from_str(input).unwrap()
}

// Finally, the program will output a diagnostic code and immediately halt.
// This final output isn't an error;
//  an output followed immediately by a halt means the program finished.
// If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.
// After providing 1 to the only input instruction and passing all the tests,
//  what diagnostic code does the program produce?
#[aoc(day5, part1, Loop)]
fn solve_part1_loop(vm: &IntcodeVM) -> Result<i64, Error> {
    let output = vm.clone().simple_input(vec![1]).execute_and_collect()?;
    Ok(*output.last().expect("expected output to contain at least one value"))
}

// The air conditioner comes online!
// Its cold air feels good for a while,
//  but then the TEST alarms start to go off.
// Since the air conditioner can't vent its heat anywhere but
//  back into the spacecraft,
//  it's actually making the air inside the ship warmer.
//
// Instead, you'll need to use the TEST to extend the thermal radiators.
// Fortunately, the diagnostic program (your puzzle input) is already
//  equipped for this.
// Unfortunately, your Intcode computer is not.
//
// This time, when the TEST diagnostic program runs its
//  input instruction to get the ID of the system to test,
//  provide it 5, the ID for the ship's thermal radiator controller.
// This diagnostic test suite only outputs one number,
//  the diagnostic code.
#[aoc(day5, part2, Loop)]
fn solve_part2_loop(vm: &IntcodeVM) -> Result<i64, Error> {
    let output = vm.clone().simple_input(vec![5]).execute_and_collect()?;
    Ok(*output.last().expect("expected output to contain at least one value"))
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Run {
        pub code: String,
        pub inputs: Vec<i64>,
        pub expected_success: bool,
        pub expected_output: Vec<i64>,
    }

    #[test]
    fn day5_examples() -> Result<(), Error> {
        let runs = vec![
            Run{
                code: "3,9,8,9,10,9,4,9,99,-1,8".to_owned(),
                inputs: vec![8],
                expected_success: true,
                expected_output: vec![1],
            },
            Run{
                code: "3,9,7,9,10,9,4,9,99,-1,8".to_owned(),
                inputs: vec![8],
                expected_success: true,
                expected_output: vec![0],
            },
            Run{
                code: "3,3,1108,-1,8,3,4,3,99".to_owned(),
                inputs: vec![8],
                expected_success: true,
                expected_output: vec![1],
            },
            Run{
                code: "3,3,1107,-1,8,3,4,3,99".to_owned(),
                inputs: vec![8],
                expected_success: true,
                expected_output: vec![0],
            },

            Run{
                code: "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_owned(),
                inputs: vec![0],
                expected_success: true,
                expected_output: vec![0],
            },
            Run{
                code: "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_owned(),
                inputs: vec![100],
                expected_success: true,
                expected_output: vec![1],
            },

            Run{
                code: "3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_owned(),
                inputs: vec![0],
                expected_success: true,
                expected_output: vec![0],
            },
            Run{
                code: "3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_owned(),
                inputs: vec![100],
                expected_success: true,
                expected_output: vec![1],
            },

            Run{
                code: "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_owned(),
                inputs: vec![7],
                expected_success: true,
                expected_output: vec![999],
            },
            Run{
                code: "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_owned(),
                inputs: vec![8],
                expected_success: true,
                expected_output: vec![1000],
            },
            Run{
                code: "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_owned(),
                inputs: vec![9],
                expected_success: true,
                expected_output: vec![1001],
            },

            // Expected failure since there's no input.
            Run{
                code: "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_owned(),
                inputs: vec![],
                expected_success: false,
                expected_output: vec![],
            },
        ];
        for (index, run) in runs.iter().enumerate() {
            let mut vm = IntcodeVM::from_str(run.code.as_str())?;
            vm.simple_input(run.inputs.clone());
            let output = vm.execute_and_collect();
            assert_eq!(run.expected_success, output.is_ok(), "Run #{}, success check", index);
            assert_eq!(run.expected_output, output.unwrap_or_default(), "Run #{}, output check", index);
        }
        Ok(())
    }
}