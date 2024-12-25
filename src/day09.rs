use std::collections::VecDeque;

use crate::tools;

#[derive(Clone, Copy, Debug)]
struct Entry {
    position: u64,
    length: u64,
    id: Option<u64>,
}

fn part1() -> u64 {
    let entries = read_entries();
    let mut result: Vec<Entry> = Vec::new();
    result.push(entries[0]);

    let mut filled: Vec<Entry> = entries
        .iter()
        .filter(|e| e.id.is_some())
        .skip(1)
        .map(|e| e.to_owned())
        .collect();
    let mut empty: VecDeque<Entry> = entries
        .iter()
        .filter(|e| e.id.is_none())
        .map(|e| e.to_owned())
        .collect();

    'empty_slots: while let Some(mut slot) = empty.pop_front() {
        let mut gap = slot.length;
        while gap > 0 {
            if filled.len() == 0 {
                break 'empty_slots;
            }
            let filler = filled.pop().expect("No entries to fill a gap with");
            if gap >= filler.length {
                gap -= filler.length;
                result.push(Entry {
                    id: filler.id,
                    length: filler.length,
                    position: slot.position,
                });
                slot.length -= filler.length;
                slot.position += filler.length;
            } else {
                result.push(Entry {
                    id: filler.id,
                    length: gap,
                    position: slot.position,
                });
                filled.push(Entry {
                    id: filler.id,
                    length: filler.length - gap,
                    position: filler.position,
                });
                gap = 0;
            }
        }
        result.push(filled.remove(0));
    }

    calculate_checksum(result)
}

fn part2() -> usize {
    0
}

fn calculate_checksum(entries: Vec<Entry>) -> u64 {
    entries
        .into_iter()
        .filter_map(|e| {
            if let Some(id) = e.id {
                Some(
                    (e.position..e.position + e.length)
                        .map(|p| p * id)
                        .sum::<u64>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn read_entries() -> Vec<Entry> {
    let mut result = Vec::new();
    let mut shift: u64 = 0;
    let mut id: u64 = 0;
    let mut is_content: bool = true;
    let input = read_input();
    let pipe = input.chars().filter_map(|c| c.to_digit(10));
    for d in pipe {
        result.push(Entry {
            position: shift,
            length: d as u64,
            id: if is_content { Some(id) } else { None },
        });
        if is_content {
            id += 1;
        }
        shift += d as u64;
        is_content = !is_content;
    }
    result
}

fn read_input() -> String {
    tools::read_file("09", false)
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
