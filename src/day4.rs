/*
 * Link: https://adventofcode.com/2019/day/4
 * Day 4: Secure Container
 * 
 * You arrive at the Venus fuel depot only to discover it's protected by a password.
 * The Elves had written the password on a sticky note, but someone threw it out.
*/

#[aoc_generator(day4)]
fn input_generator(input: &str) -> (u32, u32){
    let items = input
        .split('-')
        .map(str::parse::<u32>)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    (items[0], items[1])
}

fn has_adjacent(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(c1, c2)| c1 == c2)
}

fn increased_or_same(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).all(|(c1, c2)| c1 <= c2)
}

/*
 * However, they do remember a few key facts about the password:
 * 
 * It is a six-digit number.
 * The value is within the range given in your puzzle input.
 * Two adjacent digits are the same (like 22 in 122345).
 * Going from left to right, the digits never decrease; 
 *  they only ever increase or stay the same (like 111123 or 135679).
*/
#[aoc(day4, part1, Filter)]
fn day4_part1_filter(input: &(u32, u32)) -> usize {
    let (from, to) = *input;
    
    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| has_adjacent(&s))
        .filter(|s| increased_or_same(&s))
        .count()
}


fn has_adjacent_part2(s: &str) -> bool {
    let s = s.as_bytes();

    (0..s.len() - 3).any(|i| s[i + 1] == s[i + 2] && s[i] != s[i + 1] && s[i + 2] != s[i + 3])
        || (s[0] == s[1] && s[1] != s[2])
        || (s[s.len() - 1] == s[s.len() - 2] && s[s.len() - 2] != s[s.len() - 3])
}

/*
 * An Elf just remembered one more important detail:
 *  the two adjacent matching digits are not part of a larger group of matching digits.
*/
#[aoc(day4, part2, Filter)]
fn day4_part2_filter(input: &(u32, u32)) -> usize {
    let (from, to) = *input;

    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| has_adjacent_part2(&s))
        .filter(|s| increased_or_same(&s))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1(input: &str) -> bool {
        has_adjacent(input) && increased_or_same(input)
    }

    fn test_part2(input: &str) -> bool {
        has_adjacent_part2(input) && increased_or_same(input)
    }

    #[test]
    fn day4_example1() {
        let input = "111111";
        assert_eq!(true, test_part1(input));
        assert_eq!(false, test_part2(input));

        assert_eq!(input_generator("111111-111111"), (111111, 111111));
    }

    #[test]
    fn day4_input_alyti() {
        let input = input_generator("138241-674034");
        assert_eq!(input, (138241, 674034));
        assert_eq!(day4_part1_filter(&input), 1890);
        assert_eq!(day4_part2_filter(&input), 1277);
    }
}