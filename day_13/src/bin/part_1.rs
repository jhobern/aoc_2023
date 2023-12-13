fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

type RockGrid = Vec<Vec<bool>>;

fn parse_input(s: &str) -> Vec<RockGrid> {
    s.split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

fn is_mirror(ps: &[bool]) -> bool {
    let mut rev = ps.to_owned();
    rev.reverse();
    ps == rev
}

fn single_symmetry(row: &[bool], index: usize) -> bool {
    let shortest_end = index.min(row.len() - index);
    let start = index - shortest_end;
    let end = index + shortest_end - 1;

    is_mirror(&row[start..=end])
}

fn symmetry_count(grid: RockGrid) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    let mut potential_symmetry_points = (1..width).collect::<Vec<usize>>();

    for row in grid.iter() {
        potential_symmetry_points.retain(|index| single_symmetry(row, *index));
    }

    if potential_symmetry_points.len() == 1 {
        return *potential_symmetry_points.first().unwrap();
    }

    let mut potential_symmetry_points = (1..height).collect::<Vec<usize>>();

    for col in (0..width).map(|i| grid.iter().map(|row| row[i]).collect::<Vec<bool>>()) {
        potential_symmetry_points.retain(|index| single_symmetry(&col, *index));
    }

    if potential_symmetry_points.len() == 1 {
        return *potential_symmetry_points.first().unwrap() * 100;
    }

    unreachable!()
}

fn process(s: &str) -> usize {
    let input = parse_input(s);
    input.into_iter().map(symmetry_count).sum()
}
