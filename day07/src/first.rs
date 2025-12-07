use std::collections::HashSet;
use std::fs;

pub fn first() {
    let file = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<&str> = file.trim_end_matches('\n').split('\n').collect();

    let rows = lines.len();
    if rows == 0 {
        println!("0");
        return;
    }
    let cols = lines[0].chars().count();

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let mut found = false;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
                found = true;
                break;
            }
        }
        if found { break; }
    }
    if !found {
        eprintln!("Start 'S' not found");
        return;
    }

    let mut current: HashSet<i32> = HashSet::new();
    let mut r = start_row + 1;
    if r >= rows {
        println!("0");
        return;
    }
    current.insert(start_col as i32);

    let mut splits: i32 = 0;

    while !current.is_empty() && r < rows {
        let mut next: HashSet<i32> = HashSet::new();

        for &c_i32 in current.iter() {
            if c_i32 < 0 { continue; }
            let c_usize = c_i32 as usize;
            if c_usize >= cols { continue; }

            let cell = grid[r][c_usize];
            if cell == '^' {
                splits += 1;
                if c_i32 - 1 >= 0 {
                    next.insert(c_i32 - 1);
                }
                if (c_i32 + 1) < (cols as i32) {
                    next.insert(c_i32 + 1);
                }
            } else {
                next.insert(c_i32);
            }
        }

        current = next;
        r += 1;
    }

    println!("{}", splits);
}
