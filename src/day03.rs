use regex::Regex;

use crate::tools;

fn read_input() -> String {
    tools::read_file("03", false)
}

pub fn part1() -> i32 {
    let input = read_input();
    let regex = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();

    let mut result: i32 = 0;

    for capture in regex.captures_iter(&input) {
        let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();

        result += x * y;
    }

    result
}

pub fn part2() -> i32 {
    let input = read_input();
    let regex =
        Regex::new(r"(?<enable>do\(\))|(?<disable>don't\(\))|(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))")
            .unwrap();

    let mut enabled: bool = true;
    let mut result: i32 = 0;

    for capture in regex.captures_iter(&input) {
        if let Some(_) = capture.name("enable") {
            enabled = true;
            continue;
        }
        if let Some(_) = capture.name("disable") {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }

        let x: i32 = capture.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = capture.name("y").unwrap().as_str().parse().unwrap();

        result += x * y;
    }

    result
}
