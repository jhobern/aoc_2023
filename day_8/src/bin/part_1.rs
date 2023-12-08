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

fn process(s: &str) -> i32 {
    let mut lines = s.lines();
    let directions = get_directions(lines.next().unwrap()).into_iter().cycle();

    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.filter(|l| !l.is_empty()) {
        let parts = line
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect::<String>();
        let mut parts = parts.split_ascii_whitespace();

        let (key, l, r) = (
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
        paths.insert(key, (l, r));
    }

    let mut curr_location = "AAA";
    let mut count = 0;

    for direction in directions {
        count += 1;
        curr_location = match direction {
            Direction::Left => &paths[curr_location].0,
            Direction::Right => &paths[curr_location].1,
        };

        if curr_location == "ZZZ" {
            return count;
        }
    }

    unreachable!()
}
