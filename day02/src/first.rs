use std::fs;

pub fn first() {
    let input = fs::read_to_string("input.txt")
        .expect("should have been able to read the file.");

    let ranges = input.trim().split(',');

    let mut calculate: Vec<i64> = Vec::new();

    for range in ranges {
        if let Some((first_str, second_str)) = range.split_once('-') {
            let first: i64 = first_str.parse().expect("invalid number");
            let second: i64 = second_str.parse().expect("invalid number");

            let mut num = first;
            while num <= second {
                let s = num.to_string();

                if s.starts_with('0') {
                    num += 1;
                    continue;
                }

                if s.len() % 2 == 0 {
                    let half_len = s.len() / 2;
                    let first_half = &s[..half_len];
                    let second_half = &s[half_len..];
                    if first_half == second_half {
                        calculate.push(num);
                    }
                }

                num += 1;
            }

        } else {
            println!("Invalid range: {}", range);
        }
    }

    let final_answer: i64 = calculate.iter().sum();
    println!("Sum of all invalid IDs for part one: {}", final_answer);
}
