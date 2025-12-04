use std::fs;

pub fn first() {
    let file = fs::read_to_string("input.txt")
        .expect("cannot read file");

    let grid: Vec<Vec<char>> = file
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut accessible_rolls = 0;

    for row_idx in 0..grid.len() {
        let row = &grid[row_idx];

        for col_idx in 0..row.len() {
            let c = row[col_idx];
            println!("{}", c);

            if c == '@' {
                let mut neighbor_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let new_row = row_idx as isize + dr;
                        let new_col = col_idx as isize + dc;

                        if new_row >= 0
                            && new_row < grid.len() as isize
                            && new_col >= 0
                            && new_col < row.len() as isize
                        {
                            if grid[new_row as usize][new_col as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }
                }

                if neighbor_count < 4 {
                    accessible_rolls += 1;
                    println!("Accessible @ at row {}, col {}", row_idx, col_idx);
                }
            }
        }

        println!("---");
    }

    println!("{}", accessible_rolls);
}
