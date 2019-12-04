/*
 * Link: https://adventofcode.com/2019/day/2
 * Day 2: 1202 Program Alarm
 * 
 * On the way to your gravity assist around the Moon,
 *  your ship computer beeps angrily about a "1202 program alarm". 
 * On the radio, an Elf is already explaining how to handle the situation:
 *  - "Don't worry, that's perfectly norma--" 
 * The ship computer bursts into flames.
 * 
 * You notify the Elves that the computer's magic smoke seems to have escaped. 
 *  - "That computer ran Intcode programs like the gravity assist program it was working on; 
 *     surely there are enough spare parts up there to build a new Intcode computer!"
*/

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<usize> {
    return input.split_terminator(",").map(|x| x.parse::<usize>().unwrap_or(0)).collect();
}

// Magic smoke
fn run_intcode(mut inputs: Vec<usize>, a: usize, b: usize) -> usize {
    let mut ptr: usize = 0;
    let inputs = inputs.as_mut_slice();

    inputs[1] = a;
    inputs[2] = b;

    loop {
        let opcode = inputs[ptr];

        match opcode {
            // handle add opcode
            1 => {
                let arg1 = inputs[inputs[ptr + 1] as usize];
                let arg2 = inputs[inputs[ptr + 2] as usize];
                inputs[inputs[ptr + 3] as usize] = arg1 + arg2;
            }
            // handle multiply opcode
            2 => {
                let arg1 = inputs[inputs[ptr + 1] as usize];
                let arg2 = inputs[inputs[ptr + 2] as usize];
                inputs[inputs[ptr + 3] as usize] = arg1 * arg2;
            }
            // handle stopcode
            99 => {
                break;
            }
            // behave, user.
            _ => {
                panic!("Unknown opcode at pos {}: {}", ptr, opcode);
            }
        }

        ptr = ptr + 4;
    }

    return inputs[0];
}

/*
 * Once you have a working computer,
 *  the first step is to restore the gravity assist program
 *  (your puzzle input) to the "1202 program alarm" state 
 *  it had just before the last computer caught fire. 
 * To do this, before running the program,
 *  replace position 1 with the value 12 and replace position 2 with the value 2. 
 * What value is left at position 0 after the program halts?
*/
#[aoc(day2, part1, Loop)]
fn solve_part1_loop(input: &[usize]) -> usize {
    return run_intcode(input.to_vec(), 12, 2);
}

/*
 * The inputs should still be provided to the program by replacing the values at addresses 1 and 2,
 *  just like before. 
 * In this program, the value placed in address 1 is called the noun,
 *  and the value placed in address 2 is called the verb. 
 * Each of the two input values will be between 0 and 99, inclusive.
 * 
 * Once the program has halted, its output is available at address 0, also just like before. 
 * Each time you try a pair of inputs, make sure you first reset
 *  the computer's memory to the values in the program (your puzzle input) - in other words,
 *  don't reuse memory from a previous attempt.
 *
 * Find the input noun and verb that cause the program to produce the output 19690720.
*/
#[aoc(day2, part2, Loop)]
fn solve_part2_loop(input: &[usize]) -> Result<usize, &str> {
    // Pretty crappy way of doing it but I can't think of a better solution at the moment...
    // Takes whopping 1.7ms, so slow :c 
    for a in 0..=99 {
        for b in 0..=99 { 
            if run_intcode(input.to_vec(), a, b) == 19_690_720 {
                return Ok((100 * a + b) as usize)
            }
        }
    }
    Err("failed to find result")
}


#[cfg(test)]
mod tests {
    use super::*;

    // 1,0,0,0,99 becomes 2,0,0,0,99
    #[test]
    fn day1_example1() {
        assert_eq!(solve_part1_loop(&input_generator("1,0,0,0,99")), 2);
    }

    // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
    #[test]
    fn day1_example2() {
        assert_eq!(run_intcode(input_generator("1,1,1,4,99,5,6,0,99"), 1, 1), 30);
    }
}