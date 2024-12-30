use std::collections::{HashMap, HashSet};

use crate::tools;

type Point = (i64, i64);

struct Universe {
    antennas: HashMap<char, Vec<Point>>,
    antinodes: HashSet<Point>,
    width: u64,
    height: u64,
}

impl Universe {
    fn is_point_in_bounds(&self, (x, y): &(i64, i64)) -> bool {
        *x >= 0 && *y >= 0 && *x < self.width as i64 && *y < self.height as i64
    }
}

fn create_pairs(freq: &Vec<(i64, i64)>) -> Vec<(&Point, &Point)> {
    (0..freq.len())
        .flat_map(|a| (0..freq.len()).map(move |b| (a.clone(), b.clone())))
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (&freq[a], &freq[b]))
        .collect()
}

fn work(part2: bool) -> usize {
    let mut universe = read_input();

    for freq in universe.antennas.values() {
        for (a, b) in create_pairs(&freq) {
            let (ax, ay) = a;
            let (bx, by) = b;
            let dx = ax - bx;
            let dy = ay - by;
            if part2 {
                let mut tx = *ax;
                let mut ty = *ay;
                loop {
                    let p = (tx, ty);
                    if !universe.is_point_in_bounds(&p) {
                        break;
                    }
                    universe.antinodes.insert(p);
                    tx += dx;
                    ty += dy;
                }
            } else {
                let p = (*ax + dx, *ay + dy);
                if !universe.is_point_in_bounds(&p) {
                    continue;
                }
                universe.antinodes.insert(p);
            }
        }
    }

    universe.antinodes.len()
}

pub fn part1() -> usize {
    work(false)
}

pub fn part2() -> usize {
    work(true)
}

fn read_input() -> Universe {
    let raw = tools::read_file("08", false);
    let height: u64 = raw.lines().count() as u64;
    let width: u64 = raw.lines().last().unwrap().len() as u64;
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    raw.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, frequency)| {
            if frequency == '.' {
                return;
            }
            antennas
                .entry(frequency)
                .or_insert(Vec::new())
                .push((x as i64, y as i64));
        });
    });
    let antinodes: HashSet<Point> = HashSet::new();
    Universe {
        width,
        height,
        antennas,
        antinodes,
    }
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
