use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Ray {
    dir: Direction,
    pos: (i32, i32),
}

type Mirror = HashMap<Direction, Vec<Direction>>;

fn to_mirror(c: char) -> Mirror {
    use Direction::*;
    match c {
        '.' => [
            (Up, [Up].to_vec()),
            (Down, [Down].to_vec()),
            (Left, [Left].to_vec()),
            (Right, [Right].to_vec()),
        ]
        .into(),
        '|' => [
            (Up, [Up].to_vec()),
            (Down, [Down].to_vec()),
            (Left, [Up, Down].to_vec()),
            (Right, [Down, Up].to_vec()),
        ]
        .into(),
        '/' => [
            (Up, [Right].to_vec()),
            (Down, [Left].to_vec()),
            (Left, [Down].to_vec()),
            (Right, [Up].to_vec()),
        ]
        .into(),
        '\\' => [
            (Up, [Left].to_vec()),
            (Down, [Right].to_vec()),
            (Left, [Up].to_vec()),
            (Right, [Down].to_vec()),
        ]
        .into(),

        '-' => [
            (Up, [Left, Right].to_vec()),
            (Down, [Left, Right].to_vec()),
            (Left, [Left].to_vec()),
            (Right, [Right].to_vec()),
        ]
        .into(),
        _ => todo!(),
    }
}

fn process(s: &str) -> usize {
    let mut mirrors: Vec<Vec<_>> = s
        .lines()
        .map(|line| line.chars().map(to_mirror).collect())
        .collect();

    let orig_chars: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();

    let mut rays = vec![Ray {
        dir: Direction::Right,
        pos: (-1, 0),
    }];

    let width = mirrors[0].len();
    let height = mirrors.len();

    let mut energized = vec![vec![false; width]; height];

    while let Some(ray) = rays.pop() {
        let (mut x, mut y) = ray.pos;
        match ray.dir {
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Down => y += 1,
            Direction::Right => x += 1,
        };
        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            continue;
        }

        if ['|', '-'].contains(&orig_chars[y as usize][x as usize])
            && energized[y as usize][x as usize]
        {
            continue;
        }
        energized[y as usize][x as usize] = true;
        let new_directions = mirrors[y as usize][x as usize]
            .get(&ray.dir)
            .unwrap()
            .to_owned();

        if ['|', '-'].contains(&orig_chars[y as usize][x as usize]) {
            mirrors[y as usize][x as usize] = to_mirror('.');
        }

        for new_dir in new_directions {
            rays.push(Ray {
                dir: new_dir,
                pos: (x, y),
            })
        }
    }

    energized
        .into_iter()
        .map(|row| row.into_iter().filter(|x| *x).count())
        .sum()
}
