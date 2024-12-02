use crate::tools;

fn map_line_to_i32_vec(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

fn read_entries() -> Vec<Vec<i32>> {
    let as_string = tools::read_file("02", false);
    as_string.lines().map(map_line_to_i32_vec).collect()
}

fn is_entry_safe_part1(entry: &Vec<i32>) -> bool {
    let mut increasing: Option<bool> = None;

    for window in entry.as_slice().windows(2) {
        let (a, b) = (window[0], window[1]);
        let diff = a.abs_diff(b);
        if diff < 1 || diff > 3 {
            return false;
        }
        match increasing {
            None => increasing = Some(a < b),
            Some(inc) => {
                if (inc && a > b) || (!inc && a < b) {
                    return false;
                }
            }
        }
    }

    true
}

fn is_entry_safe_part2(entry: &Vec<i32>) -> bool {
    if is_entry_safe_part1(entry) {
        return true;
    }

    for i in 0..entry.len() {
        let mut copy = entry.clone();
        copy.remove(i);
        if is_entry_safe_part1(&copy) {
            return true;
        }
    }

    false
}

fn run(filter: &dyn Fn(&Vec<i32>) -> bool) -> usize {
    read_entries().into_iter().filter(filter).count()
}

pub fn part1() -> usize {
    run(&is_entry_safe_part1)
}

pub fn part2() -> usize {
    run(&is_entry_safe_part2)
}
