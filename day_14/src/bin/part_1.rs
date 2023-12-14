fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn process(s: &str) -> usize {
    let mut lines = s.lines().peekable();
    let height = lines.clone().count();
    let width = lines.peek().unwrap().len();
    let mut blockers = vec![0; width];

    let mut load = 0;
    for (h, line) in lines.enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                blockers[i] = h + 1;
            } else if c == 'O' {
                load += height - blockers[i];
                blockers[i] += 1;
            }
        }
    }

    load
}
