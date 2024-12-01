mod tools;

#[cfg(feature = "day01")]
mod day01;
#[cfg(feature = "day01")]
use day01 as day;

#[cfg(not(any(feature = "day01")))]
mod day {
    const INFO: &'static str = "No day specified";
    pub fn part1() -> &'static str {
        INFO
    }
    pub fn part2() -> &'static str {
        INFO
    }
}

fn main() {
    println!("Part 1: {}", day::part1());
    println!("Part 2: {}", day::part2());
}
