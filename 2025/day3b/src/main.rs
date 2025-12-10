use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");

    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let max_val = find_max(line);

        total_joltage += max_val;
    }

    println!("{}", total_joltage);
}

fn find_max(bank: &str) -> u64 {
    let bytes = bank.as_bytes();
    let total_len = bytes.len();
    let target_len = 12;

    if total_len < target_len {
        return 0;
    }

    let mut result_str = String::with_capacity(target_len);
    let mut curr = 0;

    let max_digit = b'0' - 1;

    for i in 0..target_len {
        let remaining = target_len - 1 - i;

        let search_limit = total_len - 1 - remaining;

        let mut max_digit = b'0' - 1;
        let mut pick_idx = curr;

        for j in curr..=search_limit {
            if bytes[j] > max_digit {
                max_digit = bytes[j];
                pick_idx = j;

                if max_digit == b'9' {
                    break;
                }
            }
        }

        result_str.push(max_digit as char);

        curr = pick_idx + 1;
    }

    result_str.parse::<u64>().unwrap_or(0)
}
