use std::cmp::Ordering;
use std::collections::HashMap;
use std::{str::FromStr, string::ParseError};

use crate::tools;

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    queue: Vec<Vec<u32>>,
}
impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let rules_raw: Vec<Vec<u32>> = lines
            .by_ref()
            .take_while(|line| line.contains('|'))
            .map(|line| {
                line.split('|')
                    .take(2)
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();
        let queue: Vec<Vec<u32>> = lines
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();

        let mut rules = Vec::new();
        for rule in rules_raw {
            rules.push((rule[0], rule[1]));
        }

        Ok(Input { rules, queue })
    }
}

fn is_valid_update(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    let indices: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(index, value)| (value.clone(), index))
        .collect();

    for v in update {
        let iv = indices[v];
        let violates = rules.iter().filter(|(a, _)| *a == *v).any(|(_, b)| {
            let ibo = indices.get(b);
            if let Some(ib) = ibo {
                iv > *ib
            } else {
                false
            }
        });
        if violates {
            return false;
        }
    }

    true
}

fn work_part1(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Option<u32> {
    if is_valid_update(update, rules) {
        Some(update[update.len() / 2])
    } else {
        None
    }
}

fn work_part2(update: &mut Vec<u32>, rules: &Vec<(u32, u32)>) -> Option<u32> {
    if is_valid_update(update, rules) {
        return None;
    }

    update.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            return Ordering::Less;
        }
        if rules.contains(&(*b, *a)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });

    return Some(update[update.len() / 2]);
}

fn read_input() -> String {
    tools::read_file("05", false)
}

pub fn part1() -> u32 {
    let input = Input::from_str(&read_input()).unwrap();
    input
        .queue
        .iter()
        .filter_map(|update| work_part1(&update, &input.rules))
        .sum()
}

pub fn part2() -> u32 {
    let mut input = Input::from_str(&read_input()).unwrap();
    input
        .queue
        .iter_mut()
        .filter_map(|update| work_part2(update, &input.rules))
        .sum()
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
