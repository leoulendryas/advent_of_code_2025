use std::fs;

pub fn second() {
    let file = fs::read_to_string("input.txt")
        .expect("cannot read file");

    let mut grid: Vec<Vec<char>> = file
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for row_idx in 0..rows {
            for col_idx in 0..cols {
                if grid[row_idx][col_idx] != '@' {
                    continue;
                }

                let mut neighbor_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = row_idx as isize + dr;
                        let nc = col_idx as isize + dc;

                        if nr >= 0 && nr < rows as isize &&
                           nc >= 0 && nc < cols as isize
                        {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }
                }

                if neighbor_count < 4 {
                    to_remove.push((row_idx, col_idx));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
        }

        total_removed += to_remove.len();
    }

    println!("Total removable @: {}", total_removed);
}
