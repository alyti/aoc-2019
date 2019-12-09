//! Link: https://adventofcode.com/2019/day/8
//! Day 8: Space Image Format
//!
//! The Elves' spirits are lifted when they realize you have
//!  an opportunity to reboot one of their Mars rovers, 
//!  and so they are curious if you would spend a brief sojourn on Mars.
//! You land your ship near the rover.
//! 
//! When you reach the rover, you discover that it's already in the process of rebooting!
//! It's just waiting for someone to enter a BIOS password. 
//! The Elf responsible for the rover takes a picture of the password (your puzzle input)
//!  and sends it to you via the Digital Sending Network.

use crate::util::sif::{SpaceImage, draw_layer};

// Unfortunately,
//  images sent via the Digital Sending Network aren't encoded with any normal encoding;
//  instead, they're encoded in a special Space Image Format.
// None of the Elves seem to remember why this is the case. 
// They send you the instructions to decode it.
// 
// Images are sent as a series of digits that each represent the color of a single pixel. 
// The digits fill each row of the image left-to-right, 
//  then move downward to the next row,
//  filling rows top-to-bottom until every pixel of the image is filled.
// 
// Each image actually consists of a series of identically-sized layers that are filled in this way.
// So, the first digit corresponds to the top-left pixel of the first layer,
//  the second digit corresponds to the pixel to the right of that on the same layer,
//  and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.
// 
// The image you received is 25 pixels wide and 6 pixels tall.
// 
// Now you're ready to decode the image.
// The image is rendered by stacking the layers and aligning the pixels with the same positions in each layer.
// The digits indicate the color of the corresponding pixel: 0 is black, 1 is white, and 2 is transparent.
// 
// The layers are rendered with the first layer in front and the last layer in back. 
// So, if a given position has a transparent pixel in the first and second layers,
//  a black pixel in the third layer,
//  and a white pixel in the fourth layer,
//  the final image would have a black pixel at that position.
#[aoc_generator(day8)]
fn input_generator(input: &str) -> SpaceImage {
    SpaceImage::load((25, 6), input)
}

// To make sure the image wasn't corrupted during transmission, 
//  the Elves would like you to find the layer that contains the fewest 0 digits. 
// On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
// 
// Your puzzle answer was 1620. 
#[aoc(day8, part1, Map)]
fn solve_part1_map(img: &SpaceImage) -> usize {
    let layer = img.layers.iter().enumerate()
        .map(|(i, _)| (i, img.pixels_in_layer(i, 0)))
        .min_by_key(|&(_, num)| num)
        .unwrap_or_default().0;
    (img.pixels_in_layer(layer, 1) * img.pixels_in_layer(layer, 2))
}

// What message is produced after decoding your image?
// 
// Your puzzle answer was BCYEF.
#[aoc(day8, part2, Map)]
fn solve_part2_map(img: &SpaceImage) -> String {
    let mut s = draw_layer(img.flat_layer());
    s.insert(0, '\n');
    s
}