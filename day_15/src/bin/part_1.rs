fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn process(s: &str) -> usize {
    s.split(',').map(hash).sum()
}
