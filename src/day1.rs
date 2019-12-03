/*
 * Link: https://adventofcode.com/2019/day/1
 * Day 1: The Tyranny of the Rocket Equation
 * 
 * The Elves quickly load you into a spacecraft and prepare to launch.
 * At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper.
 * They haven't determined the amount of fuel required yet.
*/

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    return input.lines().map(|x| x.parse::<u32>().unwrap_or(0)).collect();
}

fn fuel_for_mass(mass: u32) -> u32 {
    let result = (mass / 3).checked_sub(2);
    match result {
        Some(x) => return x,
        None => return 0,
    }
}

fn fuel_for_fuel(mut fuel: u32) -> u32 {
    let mut final_fuel: u32 = 0;
    while fuel > 0 {  
        final_fuel += fuel;
        fuel = fuel_for_mass(fuel as u32);
    }
    return final_fuel;
}

/*
 * Fuel required to launch a given module is based on its mass. 
 * Specifically, to find the fuel required for a module,
 *  take its mass, divide by three, round down, and subtract 2.
*/
#[aoc(day1, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    return input.iter().map(|&f| fuel_for_mass(f)).sum();
}

/*
 * Fuel itself requires fuel just like a module - take its mass,
 *  divide by three, round down, and subtract 2.
 * However, that fuel also requires fuel, and that fuel requires fuel, and so on.
 * Any mass that would require negative fuel should instead be treated as if it requires zero fuel;
 *  the remaining mass, if any, is instead handled by wishing really hard,
 *  which has no mass and is outside the scope of this calculation.
*/
#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    return input.iter().map(|&f| fuel_for_fuel(fuel_for_mass(f))).sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
     * For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
     * For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
     * For a mass of 1969, the fuel required is 654.
     * For a mass of 100756, the fuel required is 33583.
    */
    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("1969\n100756")), 34237);
    }
    
    /*
     * At first, a module of mass 1969 requires 654 fuel. 
     * Then, this fuel requires 216 more fuel (654 / 3 - 2). 
     * 216 then requires 70 more fuel, which requires 21 fuel, 
     *  which requires 5 fuel, which requires no further fuel. 
     * So, the total fuel required for a module of mass 1969 is:
     *  654 + 216 + 70 + 21 + 5 = 966.
     * The fuel required by a module of mass 100756 and its fuel is: 
     *  33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    */
    #[test]
    fn example2() {
        assert_eq!(solve_part2(&input_generator("1969\n100756")), 51312);
    }
}