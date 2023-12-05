fn main() {
    let input = include_str!("input.txt");

    println!("{}", input.lines().map(|l| process_one(l)).sum::<u32>());
}

fn process_one(s: &str) -> u32 {
    let mut replaced = s;
    let mut digits = Vec::new();

    let replacements = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    while !replaced.is_empty() {
        for (r, v) in replacements {
            if replaced.starts_with(r) {
                digits.push(v);
            }
        }
        replaced = &replaced[1..];
    }
    digits[0] * 10 + digits.last().unwrap()
}
