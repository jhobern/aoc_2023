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
    Gt(i64),
    Lt(i64),
}

impl Filter {
    fn apply(&self, part: &Part) -> bool {
        let val = match self.attribute {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
            Attribute::None => return true,
        };

        match self.filter {
            FilterFunc::Always => true,
            FilterFunc::Gt(n) => val > n,
            FilterFunc::Lt(n) => val < n,
        }
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

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
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

fn parse_part(s: &str) -> Part {
    //trim the braces
    let s = &s[1..s.len() - 1];
    let mut s = s
        .split(',')
        .map(|a| a.split_once('=').unwrap().1.parse().unwrap());

    Part {
        x: s.next().unwrap(),
        m: s.next().unwrap(),
        a: s.next().unwrap(),
        s: s.next().unwrap(),
    }
}

fn parse_input(s: &str) -> (Vec<WorkFlow>, Vec<Part>) {
    let (workflows, parts) = s.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow);
    let parts = parts.lines().map(parse_part);

    (workflows.collect(), parts.collect())
}

fn process(s: &str) -> i64 {
    let (workflows, parts) = parse_input(s);

    let mut sum = 0;

    for part in parts {
        let mut curr_workflow = workflows
            .iter()
            .find(|workflow| &workflow.name == "in")
            .unwrap()
            .clone();

        loop {
            let mut processed = false;
            let mut new_workflow = None;
            for condition in &curr_workflow.conditions {
                if condition.filter.apply(&part) {
                    match &condition.destination {
                        Destination::Accept => {
                            processed = true;
                            sum += part.x + part.m + part.a + part.s;
                        }
                        Destination::Reject => processed = true,
                        Destination::WorkFlow(dest) => {
                            new_workflow = workflows.iter().find(|workflow| workflow.name == **dest)
                        }
                    };
                    break;
                }
            }
            if processed {
                break;
            }
            curr_workflow = new_workflow.unwrap().clone();
        }
    }
    sum
}
