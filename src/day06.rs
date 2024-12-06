use std::collections::HashSet;
use std::{str::FromStr, string::ParseError};

use crate::enums::Direction;
use crate::tools;

#[derive(Debug)]
struct Ship {
    x: usize,
    y: usize,
    direction: Direction,
    visited: HashSet<(usize, usize)>,
}
impl Ship {
    fn init(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::N,
            visited: HashSet::new(),
        }
    }

    fn rotate(&mut self) {
        match self.direction {
            Direction::N => self.direction = Direction::E,
            Direction::E => self.direction = Direction::S,
            Direction::S => self.direction = Direction::W,
            Direction::W => self.direction = Direction::N,
            _ => {}
        }
    }

    fn go(&mut self) {
        self.visited.insert((self.x, self.y));
        match self.direction {
            Direction::N => self.y -= 1,
            Direction::E => self.x += 1,
            Direction::S => self.y += 1,
            Direction::W => self.x -= 1,
            _ => {}
        }
    }

    fn get_distance_traveled(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Debug)]
struct Universe {
    data: Vec<Vec<char>>,
}
impl Universe {
    fn get(&self, x: i32, y: i32) -> char {
        if y < 0 || x < 0 || y as usize >= self.data.len() || x as usize >= self.data[0].len() {
            return 'X';
        }
        return self.data[y as usize][x as usize];
    }
}

#[derive(Debug)]
struct Data {
    universe: Universe,
    ship: Ship,
}
impl FromStr for Data {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut y: usize = 0;
        let mut x: usize = 0;

        for my in 0..map.len() {
            for mx in 0..map[0].len() {
                if map[my][mx] == '^' {
                    x = mx;
                    y = my;
                    map[my][mx] = '.';
                    break;
                }
            }
        }

        let universe = Universe { data: map };

        let ship = Ship::init(x, y);

        Ok(Data { universe, ship })
    }
}

impl Data {
    fn step(&mut self) -> bool {
        let next = match self.ship.direction {
            Direction::N => self
                .universe
                .get(self.ship.x as i32, self.ship.y as i32 - 1),
            Direction::E => self
                .universe
                .get(self.ship.x as i32 + 1, self.ship.y as i32),
            Direction::S => self
                .universe
                .get(self.ship.x as i32, self.ship.y as i32 + 1),
            Direction::W => self
                .universe
                .get(self.ship.x as i32 - 1, self.ship.y as i32),
            _ => '#',
        };
        match next {
            'X' => {
                return true;
            }
            '#' => self.ship.rotate(),
            '.' => self.ship.go(),
            _ => {}
        };
        false
    }
}

fn read_input() -> String {
    tools::read_file("06", false)
}

pub fn part1() -> usize {
    let mut data = Data::from_str(&read_input()).unwrap();
    loop {
        if data.step() {
            break;
        }
    }
    data.ship.get_distance_traveled() + 1
}

pub fn part2() -> u32 {
    0
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
