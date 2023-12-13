use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Spring {
    Fixed,
    Broken,
    Unknown,
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<Spring>,
    contiguous_broken_streaks: Vec<usize>,
}

type T<'a> = (usize, usize, usize, bool);

fn worker(
    cache: &mut HashMap<T, usize>,
    springs: &[Spring],
    brokens_remaining: usize,
    brokens: &[usize],
    just_finished_broken_streak: bool,
) -> usize {
    let key = (
        springs.len(),
        brokens_remaining,
        brokens.len(),
        just_finished_broken_streak,
    );
    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let value = if springs.is_empty() {
        if brokens_remaining == 0 && brokens.is_empty() {
            1
        } else {
            0
        }
    } else if just_finished_broken_streak {
        match springs[0] {
            Spring::Broken => 0,
            _ => worker(cache, &springs[1..], brokens_remaining, brokens, false),
        }
    } else if brokens_remaining > 0 {
        match springs[0] {
            Spring::Fixed => 0,
            _ => worker(
                cache,
                &springs[1..],
                brokens_remaining - 1,
                brokens,
                brokens_remaining == 1,
            ),
        }
    } else if brokens_remaining == 0 {
        let (fixed, broken) = match springs[0] {
            Spring::Fixed => (true, false),
            Spring::Broken => (false, true),
            Spring::Unknown => (true, true),
        };

        let mut total = 0;
        if fixed {
            total += worker(cache, &springs[1..], brokens_remaining, brokens, false);
        }
        if broken && !brokens.is_empty() {
            total += worker(
                cache,
                &springs[1..],
                brokens[0] - 1,
                &brokens[1..],
                brokens[0] == 1,
            );
        }

        total
    } else {
        unreachable!()
    };

    cache.insert(key, value);

    value
}

fn combinations(row: SpringRow) -> usize {
    let mut memoised = HashMap::new();
    worker(
        &mut memoised,
        &row.springs,
        0,
        &row.contiguous_broken_streaks,
        false,
    )
}

fn parse_input(s: &str) -> Vec<SpringRow> {
    s.lines()
        .map(|line| {
            let (springs, contiguous_broken_streaks) = line.split_once(' ').unwrap();

            let springs = [springs].repeat(5).join("?");
            let springs: Vec<Spring> = springs
                .chars()
                .map(|c| match c {
                    '#' => Spring::Broken,
                    '.' => Spring::Fixed,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect();

            let contiguous_broken_streaks = contiguous_broken_streaks
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            SpringRow {
                springs,
                contiguous_broken_streaks,
            }
        })
        .collect()
}

fn process(s: &str) -> usize {
    parse_input(s).into_iter().map(combinations).sum()
}
