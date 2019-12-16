#![feature(generators, try_trait)]

extern crate aoc_runner;

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate aoc_runner_derive;
extern crate gen_iter;
extern crate num_integer;

pub mod common;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

aoc_lib!{ year = 2019 }
