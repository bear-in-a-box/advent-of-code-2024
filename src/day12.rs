use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use crate::tools;

type Area = Vec<Vec<char>>;
type Point = (usize, usize);

#[derive(Debug)]
struct Region {
    points: HashSet<Point>,
    plant: char,
    perimeter: usize,
}
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Region <{}, p={}>: {:?}",
            self.plant, self.perimeter, self.points
        )
    }
}

fn get_neighboring_points((x, y): &Point) -> Vec<Option<Point>> {
    let mut result: Vec<Option<Point>> = vec![];
    if let Some(nx) = x.checked_sub(1) {
        result.push(Some((nx, y.clone())));
    } else {
        result.push(None);
    }
    if let Some(ny) = y.checked_sub(1) {
        result.push(Some((x.clone(), ny)));
    } else {
        result.push(None);
    }
    result.push(Some((x + 1, y.clone())));
    result.push(Some((x.clone(), y + 1)));
    result
}

fn read_region(area: &Area, start_x: usize, start_y: usize) -> Region {
    let mut points: HashSet<Point> = HashSet::new();
    let plant = area[start_y][start_x];

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back((start_x.into(), start_y.into()));
    while let Some((x, y)) = queue.pop_back() {
        if points.contains(&(x, y)) {
            continue;
        }
        match area.get(y).and_then(|row| row.get(x)) {
            Some(c) if *c == plant => {
                points.insert((x.clone(), y.clone()));
                get_neighboring_points(&(x, y)).into_iter().for_each(|po| {
                    if let Some(p) = po {
                        queue.push_back(p);
                    }
                });
            }
            _ => {}
        }
    }

    let perimeter = points
        .iter()
        .flat_map(get_neighboring_points)
        .filter(|po| {
            if let Some((x, y)) = po {
                area.get(y.clone()).and_then(|row| row.get(x.clone())) != Some(&plant)
            } else {
                true
            }
        })
        .count();

    Region {
        plant,
        points,
        perimeter,
    }
}

fn read_regions(area: Area) -> Vec<Region> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Region> = vec![];

    for y in 0..area.len() {
        for x in 0..area[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let region = read_region(&area, x, y);
            for p in region.points.iter() {
                visited.insert(p.clone());
            }
            regions.push(region);
        }
    }

    regions
}

pub fn part1() -> usize {
    let x = read_regions(read_input());
    x.iter().map(|r| r.points.len() * r.perimeter).sum()
}

pub fn part2() -> u64 {
    0
}

fn read_input() -> Area {
    tools::read_file("12", false)
        .lines()
        .map(|line| line.chars().collect())
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
