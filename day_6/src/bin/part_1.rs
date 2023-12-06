fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn extract_numbers(s: &str) -> Vec<i64> {
    let numbers = s.split(':').nth(1).unwrap();
    numbers
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect()
}

fn process(s: &str) -> i64 {
    let mut lines = s.lines();

    let times = extract_numbers(lines.next().unwrap());
    let distances = extract_numbers(lines.next().unwrap());

    let pairs = times.into_iter().zip(distances);

    pairs
        .into_iter()
        .map(|(t, d)| (0..t).filter(|t_n| t_n * (t - t_n) > d).count() as i64)
        .product()
}
