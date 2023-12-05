fn main() {
    let input = include_str!("input.txt");

    println!(
        "{}",
        process(input).into_iter().map(|s| s.start).min().unwrap()
    );
}

#[derive(Debug)]
struct MappingElement {
    source: i64,
    dest: i64,
    range: i64,
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64,
}

fn get_seeds(s: &str) -> Vec<SeedRange> {
    s.split(' ')
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|info| {
            let start = info[0];
            SeedRange {
                start,
                end: start + info[1] - 1,
            }
        })
        .collect()
}

fn process(s: &str) -> Vec<SeedRange> {
    let mut lines = s.lines().peekable();

    let seeds = lines.next().unwrap();
    let mut seeds = get_seeds(seeds.split(": ").nth(1).unwrap());

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

        let f = |seed: SeedRange| -> Vec<SeedRange> {
            let mut ranges = Vec::new();
            let mut unused_seeds = vec![seed];
            for mapping in &mappings {
                if unused_seeds.is_empty() {
                    break;
                }

                unused_seeds = unused_seeds
                    .into_iter()
                    .flat_map(|seed| {
                        if seed.end >= mapping.source
                            && seed.start <= mapping.source + mapping.range
                        {
                            let overlap_start = seed.start.max(mapping.source);
                            let overlap_end = seed.end.min(mapping.source + mapping.range);

                            ranges.push(SeedRange {
                                start: overlap_start + mapping.dest - mapping.source,
                                end: overlap_end + mapping.dest - mapping.source,
                            });

                            let mut output = Vec::new();
                            if seed.start < overlap_start {
                                output.push(SeedRange {
                                    start: seed.start,
                                    end: overlap_start - 1,
                                });
                            }
                            if seed.end > overlap_end {
                                output.push(SeedRange {
                                    start: overlap_end + 1,
                                    end: seed.end,
                                })
                            }
                            output
                        } else {
                            vec![seed]
                        }
                    })
                    .collect();
            }

            ranges.append(&mut unused_seeds);

            ranges
        };

        seeds = seeds.into_iter().flat_map(f).collect();
    }

    seeds
}
