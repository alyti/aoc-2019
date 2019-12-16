//! Link: https://adventofcode.com/2019/day/14
//! Day 14: Space Stoichiometry
//! 
//! As you approach the rings of Saturn, your ship's low fuel indicator turns on. 
//! There isn't any fuel here, but the rings have plenty of raw material. 
//! Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory 
//!  can turn these raw materials into fuel.
//! 
//! You ask the nanofactory to produce a list of the reactions
//!  it can perform that are relevant to this process (your puzzle input). 
//! Every reaction turns some quantities of specific input chemicals into 
//!  some quantity of an output chemical. Almost every chemical is
//!  produced by exactly one reaction; the only exception,
//!  ORE, is the raw material input to the entire process and is not produced by a reaction.
//! 
//! You just need to know how much ORE you'll need to collect
//!  before you can produce one unit of FUEL.

use std::collections::HashMap;

struct Formulas(HashMap<String, (usize, Vec<(usize, String)>)>);

// Each reaction gives specific quantities for its inputs and output; 
//  reactions cannot be partially run, so only whole integer 
//  multiples of these quantities can be used. 
// (It's okay to have leftover chemicals when you're done, though.)
#[aoc_generator(day14)]
fn input_generator(input: &str) -> Formulas {
    Formulas(input
        .trim()
        .lines()
        .map(|l| {
            let sides: Vec<_> = l.split(" => ").collect();
            let ingr = sides[0].split(',').map(|pair| {
                let pair: Vec<_> = pair.split_whitespace().collect();
                let num = pair[0].parse().unwrap();
                let ident = pair[1];
                (num, ident.to_owned())
            }).collect();
            let result: Vec<_> = sides[1].split_whitespace().collect();
            let num = result[0].parse().unwrap();
            let ident = result[1];
            (ident.to_owned(), (num, ingr))
        }).collect())
}

impl Formulas {
    pub fn resolve(&self, item: String, count: usize) -> usize {
        self.resolve_recursive(&item, count, &mut HashMap::new())
    }

    fn resolve_recursive(&self, item: &String, mut count: usize, extras: &mut HashMap<String, usize>) -> usize {
        if item == "ORE" {
            return count;
        }
        if let Some(n) = extras.remove(item) {
            use std::cmp::Ordering::{Greater, Equal, Less};
            match count.cmp(&n) {
                Greater => count -= n,
                Equal => return 0,
                Less => { extras.insert(item.clone(), n - count); return 0},
            }
        }
        let (reaction_count, reactors) = &self.0[item];
        let needed_reactions = match count % reaction_count {
            0 => count / reaction_count,
            x => {
                *extras.entry(item.clone()).or_default() += reaction_count - x;
                count / reaction_count + 1
            }
        };
        reactors.iter().map(|(reactor_count, reactor)| {
            self.resolve_recursive(reactor, needed_reactions * reactor_count, extras)
        }).sum::<usize>()
    }
}

// Given the list of reactions in your puzzle input, 
//  what is the minimum amount of ORE required to produce exactly 1 FUEL?
// 
// Your puzzle answer was 857266.
#[aoc(day14, part1, Recursion)]
fn solve_part1_recursion(f: &Formulas) -> usize {
    f.resolve("FUEL".to_owned(), 1)
}

// After collecting ORE for a while, you check your cargo hold: 1 trillion (1000000000000) units of ORE.
// Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
// 
// Your puzzle answer was 2144702.
#[aoc(day14, part2, Recursion)]
fn solve_part2_recursion(f: &Formulas) -> usize {
    let max_ore = f.resolve("FUEL".to_owned(), 1_000_000_000);
    let ratio = max_ore as f64 / 1_000_000_000.0;
    let mut guess = (1e12 / ratio) as usize + 1;

    while f.resolve("FUEL".to_owned(), guess) < 10_usize.pow(12) {
        guess += 2;
    }
    while f.resolve("FUEL".to_owned(), guess) > 10_usize.pow(12) {
        guess -= 1;
    }

    guess
}