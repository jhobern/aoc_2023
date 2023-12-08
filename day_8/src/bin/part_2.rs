use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn get_directions(s: &str) -> Vec<Direction> {
    s.chars()
        .filter_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn process(s: &str) -> i64 {
    let mut lines = s.lines();
    let directions = get_directions(lines.next().unwrap()).into_iter().cycle();

    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.filter(|l| !l.is_empty()) {
        let parts = line
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        let mut parts = parts.split_ascii_whitespace();

        let (key, l, r) = (
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
        paths.insert(key, (l, r));
    }

    let a_nodes = paths
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|mut curr_location| {
            let mut count = 0;

            for direction in directions.clone() {
                count += 1;
                curr_location = match direction {
                    Direction::Left => &paths[curr_location].0,
                    Direction::Right => &paths[curr_location].1,
                };

                if curr_location.ends_with('Z') {
                    return count;
                }
            }

            unreachable!()
        })
        .collect::<Vec<_>>();

    a_nodes.into_iter().reduce(lcm).unwrap()
}
