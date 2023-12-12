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
    just_finished_a_broken_streak: bool,
) -> usize {
    let key = (
        springs.len(),
        brokens_remaining,
        brokens.len(),
        just_finished_a_broken_streak,
    );
    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let mut value = 0;

    if springs.is_empty() {
        if brokens_remaining != 0 || !brokens.is_empty() {
            value = 0;
        } else {
            value = 1;
        }
    } else if just_finished_a_broken_streak {
        value = match springs[0] {
            Spring::Broken => 0,
            _ => worker(cache, &springs[1..], brokens_remaining, brokens, false),
        };
    } else if brokens_remaining > 0 {
        value = if springs[0] == Spring::Fixed {
            0
        } else {
            worker(
                cache,
                &springs[1..],
                brokens_remaining - 1,
                brokens,
                brokens_remaining == 1,
            )
        };
    } else if brokens_remaining == 0 {
        value = match springs[0] {
            Spring::Fixed => worker(cache, &springs[1..], brokens_remaining, brokens, false),
            Spring::Broken => {
                if brokens.is_empty() {
                    0
                } else {
                    worker(
                        cache,
                        &springs[1..],
                        brokens[0] - 1,
                        &brokens[1..],
                        brokens[0] == 1,
                    )
                }
            }
            Spring::Unknown => {
                if brokens.is_empty() {
                    worker(cache, &springs[1..], brokens_remaining, brokens, false)
                } else {
                    worker(cache, &springs[1..], brokens_remaining, brokens, false)
                        + worker(
                            cache,
                            &springs[1..],
                            brokens[0] - 1,
                            &brokens[1..],
                            brokens[0] == 1,
                        )
                }
            }
        };
    }

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
            let mut line = line.split(' ');

            let springs = line.next().unwrap();
            let springs = format!("{springs}?");
            let mut springs = springs.repeat(5);
            springs.pop();
            let springs: Vec<Spring> = springs
                .chars()
                .map(|c| match c {
                    '#' => Spring::Broken,
                    '.' => Spring::Fixed,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect();

            let contiguous_broken_streaks = line
                .next()
                .unwrap()
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
    let input = parse_input(s);
    input.into_iter().map(combinations).sum()
}
