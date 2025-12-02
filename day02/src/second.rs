use std::fs;

fn is_repeated_sequence(s: &str) -> bool {
    let len = s.len();
    for seq_len in 1..=len / 2 {
        if len % seq_len != 0 {
            continue;
        }
        let repeat_count = len / seq_len;
        let seq = &s[..seq_len];
        if seq.repeat(repeat_count) == s {
            return true;
        }
    }
    false
}

pub fn second() {
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

                if is_repeated_sequence(&s) {
                    calculate.push(num);
                }

                num += 1;
            }
        } else {
            println!("Invalid range: {}", range);
        }
    }

    let final_answer: i64 = calculate.iter().sum();
    println!("Sum of all invalid IDs for part two: {}", final_answer);
}
