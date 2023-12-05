fn main() {
    let input = include_str!("input.txt");
    println!("{}", input.lines().map(process_one).sum::<u32>());
}

fn extract_numeric(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn process_one(s: &str) -> u32 {
    let mut s = s.split(':');
    s.next().unwrap();
    let draws = s.next().unwrap().split(';');

    let mut max_green = 0;
    let mut max_blue = 0;
    let mut max_red = 0;

    for draw in draws {
        let individuals = draw.split(',');
        for individual in individuals {
            let count = extract_numeric(individual);
            if individual.contains("blue") {
                max_blue = max_blue.max(count);
            }
            if individual.contains("green") {
                max_green = max_green.max(count);
            }
            if individual.contains("red") {
                max_red = max_red.max(count);
            }
        }
    }

    max_blue * max_red * max_green
}
