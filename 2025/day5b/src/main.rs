use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let (start_str, end_str) = line.split_once('-').unwrap();
        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        ranges.push((start, end));
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    if !ranges.is_empty() {
        let (mut current_start, mut current_end) = ranges[0];

        for &(next_start, next_end) in ranges.iter().skip(1) {
            if next_start <= current_end + 1 {
                current_end = cmp::max(current_end, next_end);
            } else {
                merged_ranges.push((current_start, current_end));
                current_start = next_start;
                current_end = next_end;
            }
        }

        merged_ranges.push((current_start, current_end));
    }

    let mut total_fresh = 0;
    for (start, end) in merged_ranges {
        total_fresh += (end - start) + 1;
    }

    println!("{}", total_fresh);
}
