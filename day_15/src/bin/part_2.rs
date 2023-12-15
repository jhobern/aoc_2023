fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens {
    key: String,
    value: usize,
}

fn process(s: &str) -> usize {
    let mut boxes = vec![vec![]; 256];

    for instruction in s.split(',') {
        if instruction.ends_with('-') {
            let key = instruction[0..instruction.len() - 1].to_string();
            let h = hash(&key);
            if let Some(position) = boxes[h].iter().position(|v: &Lens| v.key == key) {
                boxes[h].remove(position);
            }
        } else {
            let (key, value) = instruction.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let key = key.to_string();
            let h = hash(&key);
            if let Some(position) = boxes[h].iter().position(|v| v.key == key) {
                boxes[h][position].value = value;
            } else {
                boxes[h].push(Lens {
                    key: key.to_string(),
                    value,
                });
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1)
                * b.into_iter()
                    .enumerate()
                    .map(|(k, lens)| ((k + 1) * lens.value))
                    .sum::<usize>()
        })
        .sum()
}
