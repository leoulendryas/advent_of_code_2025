pub fn calculate_cephalopod_math(input: &str) -> i128 {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut blocks: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_block: Vec<Vec<char>> = Vec::new();

    for col in (0..width).rev() {
        let column_chars: Vec<char> = lines
            .iter()
            .map(|line| line.chars().nth(col).unwrap_or(' '))
            .collect();

        let is_blank = column_chars.iter().all(|c| c.is_whitespace());

        if is_blank {
            if !current_block.is_empty() {
                blocks.push(current_block.clone());
                current_block.clear();
            }
        } else {
            current_block.push(column_chars);
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    let mut results = Vec::new();

    for block in blocks {
        let mut numbers = Vec::new();

        for col in block.iter() {
            let mut digits = String::new();
            for r in 0..height - 1 {
                let c = col[r];
                if c.is_ascii_digit() {
                    digits.push(c);
                }
            }
            if !digits.is_empty() {
                numbers.push(digits.parse::<i128>().unwrap());
            }
        }

        let operator = block
            .iter()
            .find_map(|col| {
                let c = col[height - 1];
                if !c.is_whitespace() {
                    Some(c)
                } else {
                    None
                }
            })
            .expect("No operator found in block");

        let result = match operator {
            '+' => numbers.iter().fold(0i128, |acc, &v| acc + v),
            '*' => numbers.iter().fold(1i128, |acc, &v| acc * v),
            _ => panic!("Unknown operator"),
        };

        results.push(result);
    }

    results.into_iter().sum()
}

pub fn second() {
    let input = std::fs::read_to_string("input.txt")
        .expect("can not read file");

    let answer = calculate_cephalopod_math(&input);

    println!("Part Two Answer: {}", answer);
}
