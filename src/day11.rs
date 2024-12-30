use std::collections::HashMap;

use crate::tools;

fn blink(line: HashMap<String, u64>) -> HashMap<String, u64> {
    let keys: Vec<_> = line.keys().cloned().collect();
    let mut target: HashMap<String, u64> = HashMap::new();
    for key in keys {
        if key == "0" {
            let zero = line.get("0").unwrap().clone();
            *target.entry("1".to_string()).or_insert(0) += zero;
        } else if key.len() % 2 == 0 {
            let values = line.get(&key).unwrap().clone();
            let (left, right) = key.split_at(key.len() / 2);
            let real_right = right.parse::<u64>().unwrap().to_string();
            *target.entry(left.to_string()).or_insert(0) += values;
            *target.entry(real_right.to_string()).or_insert(0) += values;
        } else {
            let values = line.get(&key).unwrap().clone();
            let multiplied = (key.parse::<u128>().unwrap() * 2024).to_string();
            *target.entry(multiplied).or_insert(0) += values;
        }
    }
    target
}

fn work(blinks: usize) -> u64 {
    let line = read_input();
    let mut data: HashMap<String, u64> = HashMap::new();
    for item in line {
        *data.entry(item).or_insert(0) += 1;
    }
    let result = (0..blinks).fold(data, |d, _| blink(d));
    result.values().sum()
}

pub fn part1() -> u64 {
    work(25)
}

pub fn part2() -> u64 {
    work(75)
}

fn read_input() -> Vec<String> {
    tools::read_file("11", false)
        .split_whitespace()
        .map(str::to_string)
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
