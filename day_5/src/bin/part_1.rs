fn main() {
    let input = include_str!("input.txt");

    println!("{}", process(input).into_iter().min().unwrap());
    //println!("{}", input.lines().map(|l| process_one(l)).sum::<u32>());
}

#[derive(Debug)]
struct MappingElement {
    source: i64,
    dest: i64,
    range: i64,
}

fn process(s: &str) -> Vec<i64> {
    let mut lines = s.lines().peekable();

    let seeds = lines.next().unwrap();
    let mut seeds = values(seeds.split(':').nth(1).unwrap());

    for _ in 0..7 {
        lines.next().unwrap();
        lines.next().unwrap();
        let mut mappings = Vec::new();
        while let Some(line) = lines.peek() {
            if line.is_empty() {
                break;
            }
            let mut vals = lines
                .next()
                .unwrap()
                .split(' ')
                .map(|v| v.parse::<i64>().unwrap());
            mappings.push(MappingElement {
                dest: vals.next().unwrap().to_owned(),
                source: vals.next().unwrap().to_owned(),
                range: vals.next().unwrap().to_owned(),
            });
        }

        seeds = seeds
            .into_iter()
            .map(|seed| {
                for mapping in &mappings {
                    if seed >= mapping.source && seed <= mapping.source + mapping.range {
                        return seed + mapping.dest - mapping.source;
                    }
                }
                seed
            })
            .collect();
    }

    seeds
}

fn values(s: &str) -> Vec<i64> {
    s.split(' ')
        .filter_map(|num| num.parse::<i64>().ok())
        .collect::<Vec<_>>()
}
