pub fn first() {
    let file = std::fs::read_to_string("input.txt")
        .expect("cannot read file");

    let input: Vec<&str> = file.trim().split("\n\n").collect();

    let ranges = input[0];
    let fresh_ingredient = input[1];

    let mut v: Vec<i64> = Vec::new();

    for i in ranges.lines() {
        let range: Vec<&str> = i.split("-").collect();
        let start = range[0].parse().unwrap();
        let end = range[1].parse().unwrap();

        for j in fresh_ingredient.lines() {
            let num: i64 = j.parse().unwrap();

            if num >= start && num <= end {
                v.push(num);
            }
        }
    }

    v.sort();
    v.dedup();
    
    let answer = v.len();

    println!("{}", answer);
}
