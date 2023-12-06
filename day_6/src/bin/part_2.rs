fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn extract_numbers(s: &str) -> f64 {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}

fn process(s: &str) -> i64 {
    let mut lines = s.lines();

    let time = extract_numbers(lines.next().unwrap());
    let distance = extract_numbers(lines.next().unwrap());

    //The distance that the boat travels is t_charge * (t_total - t_charge)
    //Putting this equal to distance lets us find the intercept numerically,
    //t_charge ^ 2 - t_total * t_charge + d = 0
    //Quadratic formula => t_charge = (t_total +/- (t_total^2 - 4 * distance).sqrt()) / 2
    //The distance between these points is the total number of distances that win

    let negative_soln = ((time - (time * time - 4. * distance).sqrt()) / 2.).ceil() as i64;
    let positive_soln = ((time + (time * time - 4. * distance).sqrt()) / 2.).floor() as i64;

    //add 1 for the fencepost error
    positive_soln - negative_soln + 1
}
