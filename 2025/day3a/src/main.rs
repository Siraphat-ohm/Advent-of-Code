use ::std::fs;

fn find_joltage(bank: &str) -> u32 {
    for candidate in (11..=99).rev() {
        let candidate_str = candidate.to_string();
        let bytes = candidate_str.as_bytes();

        let digit1 = bytes[0] as char;
        let digit2 = bytes[1] as char;

        if digit1 == '0' || digit2 == '0' {
            continue;
        }

        if let Some(first_idx) = bank.find(digit1) {
            let remaining_slice = &bank[first_idx + 1..];

            if remaining_slice.contains(digit2) {
                return candidate;
            }
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut joltage = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let max_val = find_joltage(line);

        joltage += max_val;
    }

    println!("Joltage : {}", joltage);
}
