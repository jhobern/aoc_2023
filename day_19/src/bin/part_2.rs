fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Clone)]
enum Attribute {
    X,
    M,
    A,
    S,
    None,
}

#[derive(Clone)]
struct Filter {
    attribute: Attribute,
    filter: FilterFunc,
}

#[derive(Clone)]
enum FilterFunc {
    Always,
    Gt(usize),
    Lt(usize),
}

impl Filter {
    fn split_parts(&self, part: &Part) -> (Part, Part) {
        let mut accepted_part = part.clone();
        let mut rejected_part = part.clone();
        fn getter<'a>(part: &'a mut Part, attribute: &Attribute) -> &'a mut (usize, usize) {
            match attribute {
                Attribute::X => &mut part.x,
                Attribute::M => &mut part.m,
                Attribute::A => &mut part.a,
                Attribute::S => &mut part.s,
                Attribute::None => unreachable!(),
            }
        }

        match self.filter {
            FilterFunc::Always => {}
            FilterFunc::Gt(n) => {
                getter(&mut accepted_part, &self.attribute).0 = n + 1;
                getter(&mut rejected_part, &self.attribute).1 = n
            }
            FilterFunc::Lt(n) => {
                getter(&mut accepted_part, &self.attribute).1 = n - 1;
                getter(&mut rejected_part, &self.attribute).0 = n
            }
        };

        (accepted_part, rejected_part)
    }
}

#[derive(Clone)]
enum Destination {
    Accept,
    Reject,
    WorkFlow(String),
}

#[derive(Clone)]
struct Condition {
    filter: Filter,
    destination: Destination,
}

#[derive(Clone)]
struct WorkFlow {
    name: String,
    conditions: Vec<Condition>,
}

#[derive(Clone)]
struct Part {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(x:{}..{}, m:{}..{}, a:{}..{}, s:{}..{})",
            self.x.0, self.x.1, self.m.0, self.m.1, self.a.0, self.a.1, self.s.0, self.s.1,
        ))
    }
}

fn parse_attribute(s: &str) -> Attribute {
    match s {
        "x" => Attribute::X,
        "m" => Attribute::M,
        "a" => Attribute::A,
        "s" => Attribute::S,
        _ => Attribute::None,
    }
}

fn parse_filter(s: &str) -> Filter {
    if let Some((comparison, value)) = s.split_once('<') {
        Filter {
            attribute: parse_attribute(comparison),
            filter: FilterFunc::Lt(value.parse().unwrap()),
        }
    } else if let Some((comparison, value)) = s.split_once('>') {
        Filter {
            attribute: parse_attribute(comparison),
            filter: FilterFunc::Gt(value.parse().unwrap()),
        }
    } else {
        Filter {
            attribute: Attribute::None,
            filter: FilterFunc::Always,
        }
    }
}

fn parse_destination(s: &str) -> Destination {
    match s {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        _ => Destination::WorkFlow(s.to_string()),
    }
}

fn parse_workflow(s: &str) -> WorkFlow {
    let (name, mut rest) = s.split_once('{').unwrap();
    rest = &rest[0..rest.len() - 1];
    let conditions = rest.split(',');
    let conditions = conditions
        .map(|cond| {
            if cond.find(':').is_none() {
                Condition {
                    filter: Filter {
                        filter: FilterFunc::Always,
                        attribute: Attribute::None,
                    },
                    destination: parse_destination(cond),
                }
            } else {
                let (filter, dest) = cond.split_once(':').unwrap();
                Condition {
                    filter: parse_filter(filter),
                    destination: parse_destination(dest),
                }
            }
        })
        .collect::<Vec<_>>();

    WorkFlow {
        name: name.to_string(),
        conditions,
    }
}

fn parse_input(s: &str) -> Vec<WorkFlow> {
    let (workflows, _) = s.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow);

    workflows.collect()
}

fn process(s: &str) -> usize {
    let workflows = parse_input(s);
    let total_part = Part {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut workflow_part_pairs: Vec<(WorkFlow, Part)> = vec![(
        workflows
            .iter()
            .find(|workflow| &workflow.name == "in")
            .unwrap()
            .clone(),
        total_part,
    )];

    let mut accepted_ranges = Vec::new();
    while let Some((workflow, mut part)) = workflow_part_pairs.pop() {
        for condition in &workflow.conditions {
            let (accepted_part, rejected_part) = condition.filter.split_parts(&part);

            match &condition.destination {
                Destination::Accept => {
                    accepted_ranges.push(accepted_part);
                }
                Destination::Reject => {}
                Destination::WorkFlow(dest) => {
                    workflow_part_pairs.push((
                        workflows
                            .iter()
                            .find(|workflow| workflow.name == *dest)
                            .unwrap()
                            .clone(),
                        accepted_part,
                    ));
                }
            }
            part = rejected_part;
        }
    }

    fn volume(part: &Part) -> usize {
        fn range_width(range: (usize, usize)) -> usize {
            range.1 - range.0 + 1
        }

        range_width(part.x) * range_width(part.m) * range_width(part.a) * range_width(part.s)
    }

    accepted_ranges.iter().map(volume).sum()
}
