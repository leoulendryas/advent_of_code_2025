use std::collections::HashMap;
use std::fs;

pub fn second() {
    let file = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<&str> = file.trim_end_matches('\n').split('\n').collect();

    let rows = lines.len();
    if rows == 0 {
        println!("0");
        return;
    }
    let cols = lines[0].chars().count();

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let (mut sr, mut sc) = (0, 0);
    let mut found = false;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                sr = r;
                sc = c;
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

    let mut r = sr + 1;
    if r >= rows {
        println!("0");
        return;
    }

    let mut current: HashMap<i32, i64> = HashMap::new();
    current.insert(sc as i32, 1);

    while r < rows && !current.is_empty() {
        let mut next: HashMap<i32, i64> = HashMap::new();

        for (&col, &count) in current.iter() {
            if col < 0 || col as usize >= cols { continue; }

            let cell = grid[r][col as usize];

            if cell == '^' {
                if col - 1 >= 0 {
                    *next.entry(col - 1).or_insert(0) += count;
                }
                if (col + 1) < cols as i32 {
                    *next.entry(col + 1).or_insert(0) += count;
                }
            } else {
                *next.entry(col).or_insert(0) += count;
            }
        }

        current = next;
        r += 1;
    }

    let total: i64 = current.values().sum();
    println!("{}", total);
}
