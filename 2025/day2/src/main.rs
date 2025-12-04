use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_repeat(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;

    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut total_sum: u64 = 0;

    for line in reader.lines() {
        for range_str in line.unwrap().split(',') {
            let (start_str, end_str) = range_str.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();

            for num in start..end {
                if is_repeat(num) {
                    total_sum += num;
                }
            }
        }
    }

    println!("{}", total_sum);
}
