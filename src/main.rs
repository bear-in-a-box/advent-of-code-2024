#![allow(dead_code)]

mod enums;
mod tools;

fn main() {}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

#[cfg(test)]
mod tests {
    #[test]
    fn everything() {
        println!("Day 01 part 1: {}", super::day01::part1());
        println!("Day 01 part 2: {}", super::day01::part2());
        println!("Day 02 part 1: {}", super::day02::part1());
        println!("Day 02 part 2: {}", super::day02::part2());
        println!("Day 03 part 1: {}", super::day03::part1());
        println!("Day 03 part 2: {}", super::day03::part2());
        println!("Day 04 part 1: {}", super::day04::part1());
        println!("Day 04 part 2: {}", super::day04::part2());
        println!("Day 05 part 1: {}", super::day05::part1());
        println!("Day 05 part 2: {}", super::day05::part2());
        println!("Day 06 part 1: {}", super::day06::part1());
        println!("Day 06 part 2: {}", super::day06::part2());
        println!("Day 07 part 1: {}", super::day07::part1());
        println!("Day 07 part 2: {}", super::day07::part2());
        println!("Day 08 part 1: {}", super::day08::part1());
        println!("Day 08 part 2: {}", super::day08::part2());
        println!("Day 09 part 1: {}", super::day09::part1());
        println!("Day 09 part 2: {}", super::day09::part2());
        println!("Day 10 part 1: {}", super::day10::part1());
        println!("Day 10 part 2: {}", super::day10::part2());
        println!("Day 11 part 1: {}", super::day11::part1());
        println!("Day 11 part 2: {}", super::day11::part2());
        println!("Day 12 part 1: {}", super::day12::part1());
        println!("Day 12 part 2: {}", super::day12::part2());
    }
}
