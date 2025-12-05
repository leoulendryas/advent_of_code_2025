/*
 pub fn second() {
    let file = std::fs::read_to_string("input.txt")
        .expect("cannot read file");

    let input: Vec<&str> = file.trim().split("\n\n").collect();

    let ranges = input[0];

    let mut v: Vec<i128> = Vec::new();

    for i in ranges.lines() {
        let range: Vec<&str> = i.split("-").collect();
        let start = range[0].parse().unwrap();
        let end = range[1].parse().unwrap();

        for num in start..=end {
            v.push(num)
        }

    }

    v.sort();
    v.dedup();
    
    let answer = v.len();

    println!("{} {:?}", answer, &v);
 }
*/


pub fn second() {
    let file = std::fs::read_to_string("input.txt")
        .expect("cannot read file");

    let input: Vec<&str> = file.trim().split("\n\n").collect();
    let ranges_str = input[0];

    let mut intervals: Vec<(i128, i128)> = ranges_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start: i128 = parts[0].parse().unwrap();
            let end: i128 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect();

    intervals.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(i128, i128)> = Vec::new();

    for (start, end) in intervals {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total_unique: i128 = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("Total unique fresh ingredient IDs: {}", total_unique);
}
