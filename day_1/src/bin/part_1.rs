fn main() {
    let input = include_str!("input.txt");
    println!("{}", input.lines().map(|l| process_one(l)).sum::<u32>());
}

fn process_one(s: &str) -> u32 {
    let ns = s.chars().filter(|c| c.is_numeric());
    let tens = ns.clone().next().unwrap().to_digit(10).unwrap();
    let digits = ns.rev().next().unwrap().to_digit(10).unwrap();
    tens * 10 + digits
}
