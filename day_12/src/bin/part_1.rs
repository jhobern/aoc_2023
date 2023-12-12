fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

fn satisfied(springs: &[Spring], counts: &[usize]) -> bool {
    let mut runs = vec![];
    let mut i = 0;
    while i < springs.len() {
        let mut count = 0;
        while i < springs.len() && springs[i] == Spring::Broken {
            i += 1;
            count += 1;
        }
        if count != 0 {
            runs.push(count);
        } else {
            i += 1;
        }
    }

    runs == counts
}

fn combinations(row: SpringRow) -> usize {
    fn substitutions(row: Vec<Spring>) -> Vec<Vec<Spring>> {
        if row.iter().all(|spring| *spring != Spring::Unknown) {
            return vec![row];
        }

        for i in 0..row.len() {
            if row[i] == Spring::Unknown {
                let mut fixed = row.clone();
                let mut broken = row;
                fixed[i] = Spring::Fixed;
                broken[i] = Spring::Broken;

                let mut result = substitutions(fixed);
                result.extend(substitutions(broken));
                return result;
            }
        }
        unreachable!()
    }

    let counts = row.contiguous_broken_streaks;
    let subs = substitutions(row.springs);

    subs.into_iter()
        .filter(|set| satisfied(set, &counts))
        .count()
}

fn parse_input(s: &str) -> Vec<SpringRow> {
    s.lines()
        .map(|line| {
            let mut line = line.split(' ');

            let springs = line
                .next()
                .unwrap()
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
                .collect::<Vec<_>>();
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
