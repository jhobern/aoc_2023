fn main() {
    let input = include_str!("input.txt");

    println!("{}", input.lines().map(|l| process_one(l)).sum::<i32>());
}

fn values(s: &str) -> Vec<i32> {
    s.split(' ')
        .filter_map(|num| num.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn process_one(s: &str) -> i32 {
    let mut s = s.split(':');
    let (_, game) = (s.next(), s.next().unwrap());

    let mut game = game.split('|');
    let (lhs, rhs) = (game.next().unwrap(), game.next().unwrap());

    let lhs = values(lhs);
    let rhs = values(rhs);

    let mut matches = 0;
    for v in lhs {
        if rhs.contains(&v) {
            matches += 1;
        }
    }

    if matches == 0 {
        0
    } else {
        (2 as i32).pow(matches - 1)
    }
}
