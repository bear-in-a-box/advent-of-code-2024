use std::collections::HashSet;
use std::{str::FromStr, string::ParseError};

use rayon::prelude::*;

use crate::enums::Direction;
use crate::tools;

#[derive(PartialEq, Eq, Debug)]
enum StepResult {
    Loop,
    Continue,
    Quit,
}

#[derive(Clone, Debug)]
struct Ship {
    x: usize,
    y: usize,
    direction: Direction,
    visited: HashSet<(usize, usize, Direction)>,
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

    fn report_visit(&mut self) {
        self.visited
            .insert((self.x.clone(), self.y.clone(), self.direction.clone()));
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
        match self.direction {
            Direction::N => self.y -= 1,
            Direction::E => self.x += 1,
            Direction::S => self.y += 1,
            Direction::W => self.x -= 1,
            _ => {}
        }
    }

    fn get_travel_coverage(&self) -> HashSet<(usize, usize)> {
        self.visited.iter().map(|(x, y, _)| (*x, *y)).collect()
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    fn step(&mut self) -> StepResult {
        if self
            .ship
            .visited
            .contains(&(self.ship.x, self.ship.y, self.ship.direction))
        {
            return StepResult::Loop;
        }
        self.ship.report_visit();
        let (nx, ny) = match self.ship.direction {
            Direction::N => (self.ship.x as i32, self.ship.y as i32 - 1),
            Direction::E => (self.ship.x as i32 + 1, self.ship.y as i32),
            Direction::S => (self.ship.x as i32, self.ship.y as i32 + 1),
            Direction::W => (self.ship.x as i32 - 1, self.ship.y as i32),
            _ => (self.ship.x as i32, self.ship.y as i32),
        };
        let next = self.universe.get(nx, ny);

        match next {
            'X' => return StepResult::Quit,
            '#' => self.ship.rotate(),
            '.' => self.ship.go(),
            _ => {}
        };
        StepResult::Continue
    }
}

pub fn part1() -> usize {
    let mut data = Data::from_str(&read_input()).unwrap();
    loop {
        if data.step() == StepResult::Quit {
            break;
        }
    }
    data.ship.get_travel_coverage().len()
}

pub fn part2() -> usize {
    let base = Data::from_str(&read_input()).unwrap();
    let mut regular_path = base.clone();
    loop {
        if regular_path.step() == StepResult::Quit {
            break;
        }
    }
    regular_path
        .ship
        .get_travel_coverage()
        .par_iter()
        .filter_map(|&(x, y)| {
            let mut candidate = base.clone();
            if (x, y) != (base.ship.x, base.ship.y) {
                candidate.universe.data[y][x] = '#';
            }
            loop {
                match candidate.step() {
                    StepResult::Loop => return Some((x, y)),
                    StepResult::Quit => break,
                    _ => {}
                }
            }
            None
        })
        .count()
}

fn read_input() -> String {
    tools::read_file("06", false)
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
