use std::{str::FromStr, string::ParseError};

use crate::enums::Direction;
use crate::tools;

#[derive(Debug)]
struct Solution {
    data: Vec<Vec<char>>,
}
impl FromStr for Solution {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Ok(Solution { data })
    }
}
impl Solution {
    fn part1(&self) -> u32 {
        let mut result: u32 = 0;

        for (y, row) in self.data.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != 'X' {
                    continue;
                }
                for direction in Direction::iter() {
                    if self.part1_check(x as i32, y as i32, direction, 0) {
                        result += 1;
                    }
                }
            }
        }

        result
    }

    fn part1_check(&self, x: i32, y: i32, direction: &Direction, matches: usize) -> bool {
        if matches == 4 {
            return true;
        }
        if x < 0 || y < 0 || x >= self.data[0].len() as i32 || y >= self.data.len() as i32 {
            return false;
        }
        let letter: char = self.data[y as usize][x as usize];
        static NEEDLE: [char; 4] = ['X', 'M', 'A', 'S'];
        if letter != NEEDLE[matches] {
            return false;
        }
        let (dx, dy) = direction.value();
        return self.part1_check(x + dx, y + dy, direction, matches + 1);
    }

    fn part2(&self) -> u32 {
        let mut result: u32 = 0;

        for (y, row) in self.data.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != 'A' {
                    continue;
                }
                if self.part2_check(x, y) {
                    result += 1;
                }
            }
        }

        result
    }

    fn part2_check(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.data[0].len() - 1 || y == 0 || y == self.data.len() - 1 {
            return false;
        }
        static DIRS: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
        static OPTIONS: [&str; 4] = ["MSMS", "SMSM", "MMSS", "SSMM"];
        let corners = DIRS
            .iter()
            .map(|(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
            .map(|(x, y)| self.data[y][x])
            .collect::<String>();
        OPTIONS.iter().any(|o| *o == corners)
    }
}

fn read_input() -> String {
    tools::read_file("04", false)
}

pub fn part1() -> u32 {
    Solution::from_str(&read_input()).unwrap().part1()
}

pub fn part2() -> u32 {
    Solution::from_str(&read_input()).unwrap().part2()
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
