use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    string::ParseError,
};

use crate::tools;

#[derive(Debug)]
struct Area {
    data: Vec<Vec<u32>>,
    starting_points: Vec<(usize, usize)>,
    ending_points: HashSet<(usize, usize)>,
}
impl FromStr for Area {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let starting_points: Vec<(usize, usize)> = data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, h)| {
                        if *h == 0 {
                            Some((x.clone(), y.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();
        Ok(Area {
            data,
            starting_points,
            ending_points: HashSet::new(),
        })
    }
}
impl Area {
    fn go(&self, start: &(usize, usize)) -> usize {
        let mut ending_points: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(start.clone());
        while let Some((x, y)) = queue.pop_back() {
            let Some(v) = self.data.get(y).and_then(|r| r.get(x)) else {
                continue;
            };
            if *v == 9 {
                ending_points.insert((x, y));
                continue;
            }
            let next = *v + 1;
            if x > 0 {
                if self.data[y][x - 1] == next {
                    queue.push_back((x - 1, y));
                }
            }
            if y > 0 {
                if self.data[y - 1][x] == next {
                    queue.push_back((x, y - 1));
                }
            }
            if x < self.data[0].len() - 1 {
                if self.data[y][x + 1] == next {
                    queue.push_back((x + 1, y));
                }
            }
            if y < self.data.len() - 1 {
                if self.data[y + 1][x] == next {
                    queue.push_back((x, y + 1));
                }
            }
        }
        ending_points.len()
    }

    fn part1(&self) -> usize {
        self.starting_points.iter().map(|p| self.go(p)).sum()
    }
}

fn part1() -> usize {
    read_input().part1()
}

fn part2() -> u64 {
    0
}

fn read_input() -> Area {
    Area::from_str(&tools::read_file("10", false)).unwrap()
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
