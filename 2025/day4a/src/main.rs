use std::fs;

struct Grid {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = map.len();
        let width = if height > 0 { map[0].len() } else { 0 };

        Grid { map, height, width }
    }

    fn get_char(&self, r: isize, c: isize) -> Option<char> {
        if r < 0 || c < 0 {
            return None;
        }

        let r = r as usize;
        let c = c as usize;

        if r < self.height && c < self.width {
            Some(self.map[r][c])
        } else {
            None
        }
    }

    fn count_neighbors(&self, r: usize, c: usize) -> usize {
        let mut count = 0;

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dr, dc) in directions {
            let check_r = r as isize + dr;
            let check_c = c as isize + dc;

            if let Some(ch) = self.get_char(check_r, check_c) {
                if ch == '@' {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid = Grid::new(&input);

    let mut count = 0;

    for r in 0..grid.height {
        for c in 0..grid.width {
            if grid.map[r][c] == '@' {
                let neighbors = grid.count_neighbors(r, c);

                if neighbors < 4 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
