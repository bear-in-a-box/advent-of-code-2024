use crate::tools;

type Entry = (i32, i32);

fn map_line_to_entry(line: &str) -> Entry {
    let vec: Vec<i32> = line
        .split_whitespace()
        .take(2)
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    (vec[0], vec[1])
}

fn read_entries() -> (Vec<i32>, Vec<i32>) {
    let as_string = tools::read_file("01", false);
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = as_string.lines().map(map_line_to_entry).unzip();

    a.sort_unstable();
    b.sort_unstable();

    (a, b)
}

pub fn part1() -> i32 {
    let (a, b) = read_entries();

    a.into_iter()
        .zip(b.into_iter())
        .map(|(va, vb)| (vb - va).abs())
        .sum::<i32>()
}

pub fn part2() -> i32 {
    let (a, b) = read_entries();

    a.into_iter()
        .map(|va| {
            let count = b.iter().filter(|vb| va == **vb).count() as i32;
            va * count
        })
        .sum()
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
