use std::{collections::HashMap, fmt::Debug};

use priority_queue::PriorityQueue;

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

fn moves(dir: &Direction, repetitions: u8) -> Vec<Direction> {
    use Direction::*;
    if repetitions < 4 {
        return vec![dir.clone()];
    }

    let mut dirs = match *dir {
        Up | Down => vec![Left, Right],
        Left | Right => vec![Up, Down],
    };

    if repetitions < 10 {
        dirs.push(dir.clone());
    }
    dirs
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.x, self.y))
    }
}

fn process(s: &str) -> usize {
    let costs: Vec<Vec<i64>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();
    let height = costs.len();
    let width = costs[0].len();

    let mut queue: PriorityQueue<(Pos, Direction, u8, u32), i64> =
        priority_queue::PriorityQueue::new();

    let destination = Pos {
        x: width as i64 - 1,
        y: height as i64 - 1,
    };

    queue.push(
        (Pos { x: 1, y: 0 }, Direction::Right, 1, costs[0][1] as u32),
        0,
    );
    queue.push(
        (Pos { x: 0, y: 1 }, Direction::Down, 1, costs[1][0] as u32),
        0,
    );

    let mut lowest_values = HashMap::new();

    while let Some(((pos, last_move, repetitions, cost), _)) = queue.pop() {
        if pos == destination {
            if repetitions < 4 {
                continue;
            }

            return cost as usize;
        }

        if lowest_values
            .get(&(pos.clone(), last_move.clone(), repetitions))
            .is_some_and(|c| *c <= cost)
        {
            continue;
        }
        lowest_values.insert((pos.clone(), last_move.clone(), repetitions), cost);

        let new_dirs = moves(&last_move, repetitions);

        for dir in new_dirs {
            let mut pos = pos.clone();

            match dir {
                Direction::Up => pos.y -= 1,
                Direction::Left => pos.x -= 1,
                Direction::Down => pos.y += 1,
                Direction::Right => pos.x += 1,
            };

            if pos.x < 0 || pos.x >= width as i64 || pos.y < 0 || pos.y >= height as i64 {
                continue;
            }

            let repetitions = if dir == last_move { repetitions + 1 } else { 1 };
            let heuristic = destination.x - pos.x + destination.y - pos.y;

            let cost = cost + costs[pos.y as usize][pos.x as usize] as u32;
            queue.push(
                (pos.clone(), dir, repetitions, cost),
                -(heuristic + cost as i64),
            );
        }
    }

    todo!()
}
