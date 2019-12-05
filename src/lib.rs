#![feature(generators, try_trait)]

extern crate aoc_runner;

#[macro_use] extern crate failure;
#[macro_use] extern crate aoc_runner_derive;
extern crate crypto;
extern crate gen_iter;


pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib!{ year = 2019 }