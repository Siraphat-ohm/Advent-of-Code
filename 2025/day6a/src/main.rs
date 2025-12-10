use std::fs;

struct Worksheet {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Worksheet {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = grid.len();

        let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

        Worksheet {
            grid,
            height,
            width,
        }
    }

    fn get_char(&self, r: usize, c: usize) -> char {
        if r < self.height && c < self.grid[r].len() {
            self.grid[r][c]
        } else {
            ' '
        }
    }

    fn is_column_empty(&self, c: usize) -> bool {
        for r in 0..self.height {
            if self.get_char(r, c) != ' ' {
                return false;
            }
        }
        true
    }

    fn solve(&self, col_indices: &[usize]) -> u64 {
        let mut numbers: Vec<u64> = Vec::new();
        let mut operator = None;

        for r in 0..self.height {
            let row_string: String = col_indices.iter().map(|&c| self.get_char(r, c)).collect();

            let trimmed = row_string.trim();

            if trimmed.is_empty() {
                continue;
            }

            if let Ok(num) = trimmed.parse::<u64>() {
                numbers.push(num);
            } else if trimmed == "+" {
                operator = Some('+');
            } else if trimmed == "*" {
                operator = Some('*');
            }
        }

        let op = operator.unwrap();

        match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => panic!("unknown"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sheet = Worksheet::new(&input);

    let mut grand_total: u64 = 0;

    let mut current_prob_cols: Vec<usize> = Vec::new();

    for c in 0..sheet.width {
        if sheet.is_column_empty(c) {
            if !current_prob_cols.is_empty() {
                grand_total += sheet.solve(&current_prob_cols);

                current_prob_cols.clear();
            }
        } else {
            current_prob_cols.push(c);
        }
    }

    if !current_prob_cols.is_empty() {
        grand_total += sheet.solve(&current_prob_cols);
    }

    println!("{}", grand_total);
}
