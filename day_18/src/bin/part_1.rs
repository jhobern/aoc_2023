fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn shoelace_formula_area(points: &[(i64, i64)]) -> i64 {
    // Weird cross product magic to give the area of a polygon
    // If the points are clockwise the area is positive, but if the points
    // are counterclockwise the area is negative, hence the `abs()`
    ((1..points.len() - 1)
        .map(|i| points[i].0 * (points[i + 1].1 - points[i - 1].1))
        .sum::<i64>()
        / 2)
    .abs()
}

fn exterior_point_count(points: &[(i64, i64)]) -> i64 {
    let mut point_count = 0;
    for i in 0..points.len() - 1 {
        point_count +=
            (points[i].0 - points[i + 1].0).abs() + (points[i].1 - points[i + 1].1).abs();
    }

    point_count
}

fn picks_theorem(points: &[(i64, i64)]) -> i64 {
    // By Pick's theorem for grid aligned polygons:
    // area(polygon) = interior_point_count + exterior_point_count / 2 - 1
    // => interior_point_count = area(polygon) - exterior_point_count / 2 + 1
    shoelace_formula_area(points) - exterior_point_count(points) / 2 + 1
}

fn parse_input(s: &str) -> Vec<(i64, i64)> {
    let mut curr_pos = (0, 0);

    s.lines()
        .map(|line| line.split(' '))
        .map(|mut vals| {
            (
                vals.next().unwrap(),
                vals.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .map(|(dir, dist)| {
            curr_pos = match dir {
                "D" => (curr_pos.0, curr_pos.1 + dist),
                "U" => (curr_pos.0, curr_pos.1 - dist),
                "L" => (curr_pos.0 - dist, curr_pos.1),
                "R" => (curr_pos.0 + dist, curr_pos.1),
                _ => unreachable!(),
            };
            curr_pos
        })
        .collect::<Vec<_>>()
}

fn process(s: &str) -> i64 {
    let mut points = parse_input(s);

    points.insert(0, (0, 0));

    picks_theorem(&points) + exterior_point_count(&points)
}
