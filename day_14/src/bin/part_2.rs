fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

type BoulderGrid = Vec<Vec<char>>;

fn spin_90_degrees(grid: &mut BoulderGrid) {
    //abuse the fact that the grid is square
    let d = grid.len();

    for y in 0..d / 2 {
        for x in 0..d / 2 {
            let d = d - 1;
            (
                grid[y][d - x],
                grid[d - x][d - y],
                grid[d - y][x],
                grid[x][y],
            ) = (
                grid[x][y],
                grid[y][d - x],
                grid[d - x][d - y],
                grid[d - y][x],
            );
        }
    }
}

fn slide_to_top(grid: &mut BoulderGrid) {
    let mut blockers = vec![0; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == '#' {
                blockers[x] = y + 1;
            } else if grid[y][x] == 'O' {
                grid[y][x] = '.';
                grid[blockers[x]][x] = 'O';
                blockers[x] += 1;
            }
        }
    }
}

fn spin_cycle(grid: &mut BoulderGrid) {
    for _ in 0..4 {
        slide_to_top(grid);
        spin_90_degrees(grid);
    }
}

fn load(grid: &BoulderGrid) -> usize {
    let dim = grid.len();
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&c| *c == 'O').count() * (dim - i))
        .sum()
}

fn cycle_detector(loads: &[usize]) -> usize {
    let last_load = *loads.last().unwrap();

    let repetitions = loads
        .iter()
        .enumerate()
        .filter(|(_, &load)| load == last_load)
        .collect::<Vec<_>>();

    if repetitions.len() > 2 {
        let suspected_period = repetitions[1].0 - repetitions[0].0;

        for i in 0..suspected_period {
            if loads[repetitions[0].0 + i] != loads[repetitions[1].0 + i] {
                return 0;
            }
        }

        return suspected_period;
    }

    0
}

fn _dbg_print(grid: &BoulderGrid) {
    for line in grid {
        dbg!(line.iter().collect::<String>());
    }
}

fn process(s: &str) -> usize {
    const TOTAL_ITERATIONS: usize = 1_000_000_000;

    let mut grid = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut loads = vec![load(&grid)];
    let mut count = 0;

    //get at least 100 samples before we start calculating our cycles
    while count < 100 || cycle_detector(&loads) == 0 {
        spin_cycle(&mut grid);
        loads.push(load(&grid));
        count += 1;
    }

    let cycle_len = cycle_detector(&loads);

    let offset = (TOTAL_ITERATIONS - count) % cycle_len;

    loads[loads.len() - cycle_len + offset - 1]
}
