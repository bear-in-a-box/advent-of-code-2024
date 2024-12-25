use crate::tools;

fn blink(line: Vec<String>) -> Vec<String> {
    line.iter()
        .flat_map(|item| {
            if item == "0" {
                return vec!["1".to_string()].into_iter();
            }
            if item.len() % 2 == 0 {
                let (left, right) = item.split_at(item.len() / 2);
                let real_right = right.parse::<u64>().unwrap().to_string();
                return vec![left.to_string(), real_right].into_iter();
            }
            let multiplied = (item.parse::<u128>().unwrap() * 2024).to_string();
            vec![multiplied].into_iter()
        })
        .collect()
}

fn part1() -> usize {
    let line = read_input();
    let result = (0..25).fold(line, |l, _| blink(l));
    result.len()
}

fn part2() -> usize {
    0
}

fn read_input() -> Vec<String> {
    tools::read_file("11", false)
        .split_whitespace()
        .map(str::to_string)
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
