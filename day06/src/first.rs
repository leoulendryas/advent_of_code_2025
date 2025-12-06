use std::num::ParseIntError; 

pub fn calculate_columns(data_rows: Vec<Vec<i128>>, operations: Vec<char>) -> Result<Vec<i128>, String> {

    let num_columns = data_rows[0].len();
    let num_rows = data_rows.len();
    let mut results = Vec::new();

    for col_index in 0..num_columns {
        let op = operations[col_index];
        
        let mut accumulator = data_rows[0][col_index];
        
        for row_index in 1..num_rows {
            let current_value = data_rows[row_index][col_index];
            
            match op {
                '*' => accumulator = accumulator.checked_mul(current_value)
                                                .unwrap_or_else(|| panic!("Integer overflow during multiplication at column {}", col_index)),
                '+' => accumulator = accumulator.checked_add(current_value)
                                                .unwrap_or_else(|| panic!("Integer overflow during addition at column {}", col_index)),
                _ => return Err(format!("Unknown operation '{}' at column {}", op, col_index)),
            }
        }
        
        results.push(accumulator);
    }

    Ok(results)
}


pub fn first() {
    let file = std::fs::read_to_string("input.txt")
        .expect("can not read file");

    let input: Vec<&str> = file.trim().split('\n').collect();

    let len = input.len();

    let operations_str = input[len - 1];
    let data_lines = &input[0..len - 1];

    let operations: Vec<char> = operations_str
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    let data_rows: Vec<Vec<i128>> = data_lines.iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<i128>().unwrap_or_else(|e| {
                        panic!("Failed to parse number '{}' in line: {}. Error: {}", s, line, e)
                    })
                })
                .collect()
        })
        .collect();
    
    let first_row_len = data_rows.get(0).map_or(0, |r| r.len());

    match calculate_columns(data_rows, operations) {
        Ok(final_results) => {
            
            let grand_total: i128 = final_results.iter().sum();
            
            
            let raw_ops: Vec<&str> = operations_str.split_whitespace().collect();
            let mut summation_string = Vec::new();

            for (i, result) in final_results.iter().enumerate() {
                let column_values: Vec<String> = input[0..len - 1].iter()
                    .filter_map(|line| line.split_whitespace().nth(i))
                    .map(|s| s.to_string())
                    .collect();

                let op_char = raw_ops.get(i).unwrap_or(&"?");
                let calculation_string = column_values.join(&format!(" {} ", op_char));
                
                
                summation_string.push(result.to_string());
            }
            
            let equation = summation_string.join(" + ");
            
            println!("{} = {}", equation, grand_total);
        },
        Err(e) => {
            eprintln!("Calculation Error: {}", e);
        }
    }
}
