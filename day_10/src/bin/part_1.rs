use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn next_dir(pipe: char, entry: (i64, i64)) -> (i64, i64) {
    match pipe {
        '|' | '-' => entry,
        'L' => {
            if entry == (0, 1) {
                (1, 0)
            } else {
                (0, -1)
            }
        }
        'J' => {
            if entry == (1, 0) {
                (0, -1)
            } else {
                (-1, 0)
            }
        }
        '7' => {
            if entry == (1, 0) {
                (0, 1)
            } else {
                (-1, 0)
            }
        }
        'F' => {
            if entry == (-1, 0) {
                (0, 1)
            } else {
                (1, 0)
            }
        }
        _ => {
            dbg!(pipe, entry);
            unreachable!()
        }
    }
}

fn add((a, b): (i64, i64), (x, y): (i64, i64)) -> (i64, i64) {
    (a + x, b + y)
}

fn pipes_grid(s: &str) -> HashMap<(i64, i64), char> {
    let mut pipes: HashMap<(i64, i64), char> = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            pipes.insert((x as i64, y as i64), c);
        }
    }

    pipes
}

fn process(s: &str) -> i64 {
    let pipes = pipes_grid(s);

    let mut curr_pos = (0, 0);

    for (k, v) in &pipes {
        if *v == 'S' {
            curr_pos = *k;
            break;
        }
    }

    let mut velocity = if ['J', '-', '7'].contains(&pipes[&add(curr_pos, (1, 0))]) {
        (1, 0)
    } else if curr_pos.0 > 0 && ['L', '-', 'F'].contains(&pipes[&add(curr_pos, (-1, 0))]) {
        (-1, 0)
    } else if ['|', '7', 'F'].contains(&pipes[&add(curr_pos, (0, -1))]) {
        (0, -1)
    } else {
        (0, 1)
    };

    let mut path_len = 1;
    curr_pos = add(curr_pos, velocity);
    while pipes[&curr_pos] != 'S' {
        velocity = next_dir(pipes[&curr_pos], velocity);
        curr_pos = add(curr_pos, velocity);
        path_len += 1;
    }

    path_len / 2
}
