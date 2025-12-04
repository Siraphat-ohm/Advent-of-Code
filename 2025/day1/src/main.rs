use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut current_pos = 50;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let (cmd, num_str) = line.split_at(1);
        let num: i32 = num_str.parse().unwrap();

        match cmd {
            "L" => current_pos = (current_pos - num).rem_euclid(100),
            "R" => current_pos = (current_pos + num).rem_euclid(100),
            _ => (),
        }

        if current_pos == 0 {
            count += 1;
        }
    }

    println!("Password: {}", count);
}
