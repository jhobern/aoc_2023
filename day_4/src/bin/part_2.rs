fn main() {
    let input = include_str!("input.txt");

    let mut scores = input.lines().map(process_one).collect::<Vec<_>>();

    for i in (0..scores.len()).rev() {
        scores[i] = scores[i..=i + scores[i] as usize].iter().sum::<i32>();
    }

    println!("{}", scores.len() as i32 + scores.iter().sum::<i32>());
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

    lhs.into_iter().filter(|v| rhs.contains(v)).count() as i32
}
