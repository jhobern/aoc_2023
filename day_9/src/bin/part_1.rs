fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn predict_next_value(values: Vec<i64>) -> i64 {
    let mut differences = Vec::new();

    let mut curr_difference = values;

    while !curr_difference.iter().all(|n| *n == 0) {
        differences.push(curr_difference.clone());
        curr_difference = curr_difference[1..]
            .iter()
            .zip(curr_difference.iter())
            .map(|(a, b)| a - b)
            .collect();
    }

    differences.into_iter().map(|v| *v.last().unwrap()).sum()
}

fn process(s: &str) -> i64 {
    let lines = s.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });

    lines.map(predict_next_value).sum()
}
