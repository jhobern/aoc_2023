fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn extract_numbers(s: &str) -> i64 {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn process(s: &str) -> i64 {
    let mut lines = s.lines();

    let time = extract_numbers(lines.next().unwrap());
    let distance = extract_numbers(lines.next().unwrap());

    //The distance that the boat travels is t_charge * (t_total - t_charge)
    //Substituting t_charge = (t_total - t_charge)
    //We find: (t_total - t_charge) * (t_total - (t_total - t_charge)) is the distance
    //which is the same as (t_total - t_charge) * t_charge
    //So we just need to find the first point at which the boat exceeds the record,
    //And then the last time it exceeds the record will be at (t_total - t_charge), and all
    //times in between will be what we are after
    for i in 0..time {
        if i * (time - i) > distance {
            return time - i * 2 + 1;
        }
    }

    0
}
