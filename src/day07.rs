use rayon::prelude::*;

use crate::tools;

type Equation = (u128, Vec<u32>);
type Input = Vec<Equation>;

enum Operation {
    Add,
    Multiply,
}

fn check(expected: u128, so_far: u128, data_left: &[u32]) -> u128 {
    if data_left.len() == 0 {
        return if so_far == expected { 1 } else { 0 };
    }
    if so_far > expected {
        return 0;
    }
    let mut result: u128 = 0;
    result += check(expected, so_far + data_left[0] as u128, &data_left[1..]);
    result += check(expected, so_far * data_left[0] as u128, &data_left[1..]);
    result
}

fn calculate_calibration_result((expected, data): &Equation) -> u128 {
    let result = check(*expected, 0, &data[..]);
    // println!("result for {} and {:?} = {}", expected, data, result);
    result
}

pub fn part1() -> u128 {
    read_input()
        .par_iter()
        .map(|data| {
            let result = calculate_calibration_result(data);
            if result == 0 {
                0
            } else {
                data.0
            }
        })
        .sum()
}

pub fn part2() -> usize {
    0
}

fn read_input() -> Input {
    tools::read_file("07", false)
        .lines()
        .map(|line| {
            let raw = line.splitn(2, ':').collect::<Vec<&str>>();
            let a = raw[0];
            let b = raw[1];
            let result: u128 = a.parse().unwrap();
            let parts: Vec<u32> = b.split_whitespace().map(|v| v.parse().unwrap()).collect();
            (result, parts)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        println!("{}", super::part1());
    }

    #[test]
    fn part2() {
        println!("{}", super::part2());
    }
}
