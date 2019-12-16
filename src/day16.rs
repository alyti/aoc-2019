//! Link: https://adventofcode.com/2019/day/16
//! Day 16: Flawed Frequency Transmission
//! 
//! You're 3/4ths of the way through the gas giants. 
//! Not only do roundtrip signals to Earth take five hours,
//!  but the signal quality is quite bad as well. 
//! You can clean up the signal with the Flawed Frequency Transmission algorithm, or FFT.
//! 
//! As input, FFT takes a list of numbers. 
//! In the signal you received (your puzzle input), 
//!  each number is a single digit: 
//!  data like 15243 represents the sequence 1, 5, 2, 4, 3.
//! 
//! FFT operates in repeated phases. 
//! In each phase, a new list is constructed with the same length as the input list. 
//! This new list is also used as the input for the next phase.

#[aoc_generator(day16)]
fn input_generator(s: &str) -> Vec<isize> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

// Each element in the new list is built by multiplying every value in 
// the input list by a value in a repeating pattern and then adding up the results. 
// 
// While each element in the output array uses all of the same input array elements, 
//  the actual repeating pattern to use depends on which output element is being calculated. 
// The base pattern is 0, 1, 0, -1.
// Then, repeat each value in the pattern a number of times equal to the position
//  in the output list being considered.
// Repeat once for the first element, twice for the second element, 
//  three times for the third element, and so on.
// 
// When applying the pattern, skip the very first value exactly once.
// 
// After using this process to calculate each element of the output list,
//  the phase is complete, and the output list of this phase is used as
//  the new input list for the next phase, if any.
// 
// After 100 phases of FFT, what are the first eight digits in the final output list?
// 
// Your puzzle answer was 94935919.
#[aoc(day16, part1, Base)]
fn solve_part1_base(v: &[isize]) -> String {
    let mut v = v.to_vec();
    for _ in 0..100 {
        let mut nv = Vec::new();
        for n in 0..v.len() {
            let sum = v.iter()
                .zip([0, 1, 0, -1].iter()
                    .flat_map(|x| std::iter::once(x).cycle().take(n + 1))
                    .cycle()
                    .skip(1))
                .map(|(a, b)| a * b)
                .sum::<isize>();
            nv.push((sum % 10).abs());
        }
        v = nv;
    }

    v[0..8].iter().flat_map(|i| vec![(*i+48) as u8 as char]).collect::<String>()
}

// Now that your FFT is working, you can decode the real signal.
// 
// The real signal is your puzzle input repeated 10000 times. 
// Treat this new signal as a single input list. 
// Patterns are still calculated as before, and 100 phases of FFT are still applied.
// 
// The first seven digits of your initial input signal also represent the message offset. 
// The message offset is the location of the eight-digit message in the final output list. 
// Specifically, the message offset indicates the number of digits to skip
//  before reading the eight-digit message.
// 
// After repeating your input signal 10000 times and running 100 phases of FFT,
//  what is the eight-digit message embedded in the final output list?
// 
// Your puzzle answer was 24158285.
#[aoc(day16, part2, Base)]
fn solve_part2_base(v: &[isize]) -> String {
    let mut v = v.to_vec();
    let index = v.iter()
        .take(7)
        .fold(0, |acc, &x| acc * 10 + x as usize);

    let len = v.len();
    v = v.into_iter()
        .cycle()
        .take(len * 10_000)
        .skip(index)
        .collect();

    for _ in 0..100 {
        let mut nv = Vec::new();
        let mut sum = v.iter().sum::<isize>();
        for n in v.iter() {
            nv.push(sum % 10);
            sum -= n;
        }
        v = nv;
    }

    v[0..8].iter().flat_map(|i| vec![(*i+48) as u8 as char]).collect::<String>()
}