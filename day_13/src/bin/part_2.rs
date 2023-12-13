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

fn difference(ps: &[bool]) -> usize {
    let mut rev = ps.to_owned();
    rev.reverse();
    rev.into_iter().zip(ps).filter(|(a, b)| a != *b).count() / 2
}

fn single_symmetry(row: &[bool], index: usize) -> usize {
    let shortest_end = index.min(row.len() - index);
    let start = index - shortest_end;
    let end = index + shortest_end - 1;

    difference(&row[start..=end])
}

fn symmetry_count(grid: RockGrid) -> usize {
    fn worker(grid: &RockGrid) -> usize {
        let width = grid[0].len();

        let mut potential_symmetry_points = (1..width).collect::<Vec<usize>>();
        let mut errors = vec![0; width];

        for row in grid.iter() {
            potential_symmetry_points.retain(|index| {
                errors[*index] += single_symmetry(row, *index);
                errors[*index] <= 1
            });
        }

        for (i, v) in errors.into_iter().enumerate() {
            if v == 1 {
                return i;
            }
        }
        0
    }

    let horizontal = worker(&grid);

    if horizontal != 0 {
        return horizontal;
    }

    let rotated_grid = (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    worker(&rotated_grid) * 100
}

fn process(s: &str) -> usize {
    let input = parse_input(s);
    input.into_iter().map(symmetry_count).sum()
}
