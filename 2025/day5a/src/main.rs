use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    let mut reading_range = true;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            reading_range = false;
            continue;
        }

        if reading_range {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();

            ranges.push(start..=end);
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    let mut fresh_count = 0;

    for id in ids {
        let is_fresh = ranges.iter().any(|r| r.contains(&id));

        if is_fresh {
            fresh_count += 1;
        }
    }

    print!("{}", fresh_count);
}
