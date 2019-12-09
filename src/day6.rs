//! Link: https://adventofcode.com/2019/day/6
//! Day 6: Universal Orbit Map
//! 
//! You've landed at the Universal Orbit Map facility on Mercury.
//! Because navigation in space often involves transferring between orbits,
//!  the orbit maps here are useful for finding efficient routes between,
//!  for example, you and Santa. 
//! You download a map of the local orbits (your puzzle input).
//! 
//! Except for the universal Center of Mass (COM),
//!  every object in space is in orbit around exactly one other object.

use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Fail)]
enum NodeParseError {
    #[fail(display = "invalid side count for `{}`", input)]
    InvalidSideCount {
        input: String,
    },
}

struct Node(String, String);

impl FromStr for Node {
    type Err = NodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides: Vec<&str> = s.split(')').collect();
        if sides.len() != 2 {
            Err(NodeParseError::InvalidSideCount{input: s.to_owned()})
        } else {
            Ok(Node { 0: sides[0].to_owned(), 1: sides[1].to_owned() })
        }
    }
}

#[derive(Default, Debug)]
struct Map(HashMap<String, Vec<String>>);

impl Map { 
    fn from<'a>(&'a self, origin: &'a str) -> HashMap<&'a str, i32> {
        let mut result = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        result.insert(origin, 0);
        queue.push_back(origin);
        while let Some(u) = queue.pop_front() {
            let dist = *result.get(u).expect("ohno");
            for v in self.0.get(u).iter().flat_map(|e| e.iter()) {
                result.entry(v).or_insert_with(|| { queue.push_back(v); dist + 1 });
            }
        }
        result
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Option<Map> {
    input
        .split_whitespace()
        .map(|x| x.parse::<Node>())
        .try_fold(Map::default(), |mut map: Map, n: Result<Node, NodeParseError>| {
            match n {
                Ok(Node(a, b)) => {
                    map.0.entry(a.clone()).or_insert_with(|| vec![]).push(b.clone());
                    map.0.entry(b.clone()).or_insert_with(|| vec![]).push(a.clone());
                    Some(map)
                },
                Err(_) => None,
            }
        })
}

// What is the total number of direct and indirect orbits in your map data?
#[aoc(day6, part1, Map)]
fn solve_part1_map(input: &Map) -> i32 {
    input.from("COM").values().sum()
}

// What is the minimum number of orbital transfers required 
//  to move from the object YOU are orbiting to the object SAN is orbiting?
#[aoc(day6, part2, Map)]
fn solve_part2_map(input: &Map) -> i32 {
    (*input.from("YOU").get("SAN").unwrap() - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SubRun {
        pub origin: String,
        pub target: String,
        pub expected_success: bool,
        pub expected_distance: i32,
    }
    struct Run {
        pub input: String,
        pub origin: String,
        pub expected_sum: i32,
        pub subs: Vec<SubRun>,
    }
    #[test]
    fn day6_examples() -> Result<(), std::option::NoneError> {
        let runs = vec![
            Run{
                input: "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_owned(),
                origin: "COM".to_owned(),
                expected_sum: 42,
                subs: vec![
                    SubRun{
                        origin: "COM".to_owned(),
                        target: "D".to_owned(),
                        expected_success: true,
                        expected_distance: 1,
                    },
                ]
            },
            Run{
                input: "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_owned(),
                origin: "COM".to_owned(),
                expected_sum: 54,
                subs: vec![
                    SubRun{
                        origin: "YOU".to_owned(),
                        target: "SAN".to_owned(),
                        expected_success: true,
                        expected_distance: 4,
                    },
                ]
            }
        ];

        for (index, run) in runs.iter().enumerate() {
            let input = input_generator(run.input.as_str())?;
            let distances = input.from(run.origin.as_str());
            assert_eq!(run.expected_sum, distances.values().sum(), 
                "Run #{}, sum check", index);
            for (sub_index, sub_run) in run.subs.iter().enumerate() {
                let from_origin = input.from(sub_run.origin.as_str());
                let target = from_origin.get(sub_run.target.as_str());
                assert_eq!(sub_run.expected_success, target.is_some(),
                    "Run #{}, #{}, success check", index, sub_index);
                assert_eq!(sub_run.expected_distance, (*target.unwrap_or(&0) - 2),
                    "Run #{}, #{}, distance check", index, sub_index);
            }
        }

        Ok(())
    }
}