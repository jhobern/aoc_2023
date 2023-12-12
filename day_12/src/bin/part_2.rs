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
    memoised_table: &mut HashMap<T, usize>,
    springs: &[Spring],
    current_brokens_remaining_in_streak: usize,
    brokens: &[usize],
    just_finished_a_broken_streak: bool,
) -> usize {
    let key = (
        springs.len(),
        current_brokens_remaining_in_streak,
        brokens.len(),
        just_finished_a_broken_streak,
    );
    if let Some(v) = memoised_table.get(&key) {
        return *v;
    }

    let mut value = 0;

    if springs.is_empty() {
        if current_brokens_remaining_in_streak != 0 || !brokens.is_empty() {
            value = 0;
        } else {
            value = 1;
        }
    } else if just_finished_a_broken_streak {
        value = match springs[0] {
            Spring::Broken => 0,
            _ => worker(
                memoised_table,
                &springs[1..],
                current_brokens_remaining_in_streak,
                brokens,
                false,
            ),
        };
    } else if current_brokens_remaining_in_streak > 0 {
        value = if springs[0] == Spring::Fixed {
            0
        } else {
            worker(
                memoised_table,
                &springs[1..],
                current_brokens_remaining_in_streak - 1,
                brokens,
                current_brokens_remaining_in_streak == 1,
            )
        };
    } else if current_brokens_remaining_in_streak == 0 {
        value = match springs[0] {
            Spring::Fixed => worker(
                memoised_table,
                &springs[1..],
                current_brokens_remaining_in_streak,
                brokens,
                false,
            ),
            Spring::Broken => {
                if brokens.is_empty() {
                    0
                } else {
                    worker(
                        memoised_table,
                        &springs[1..],
                        brokens[0] - 1,
                        &brokens[1..],
                        brokens[0] == 1,
                    )
                }
            }
            Spring::Unknown => {
                if brokens.is_empty() {
                    worker(
                        memoised_table,
                        &springs[1..],
                        current_brokens_remaining_in_streak,
                        brokens,
                        false,
                    )
                } else {
                    worker(
                        memoised_table,
                        &springs[1..],
                        current_brokens_remaining_in_streak,
                        brokens,
                        false,
                    ) + worker(
                        memoised_table,
                        &springs[1..],
                        brokens[0] - 1,
                        &brokens[1..],
                        brokens[0] == 1,
                    )
                }
            }
        };
    }

    memoised_table.insert(key, value);

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
