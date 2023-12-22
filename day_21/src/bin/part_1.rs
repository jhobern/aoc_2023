fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

fn parse_input(s: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut start = (0, 0);
    let garden = s
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }

                    c != '#'
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (garden, start)
}

fn process(s: &str) -> usize {
    let (original_gardens, (start_x, start_y)) = parse_input(s);
    let width = original_gardens[0].len();
    let height = original_gardens.len();

    let is_in_garden = |(x, y): (usize, usize)| -> bool { y < height && original_gardens[y][x] };

    let mut curr_garden = vec![vec![false; width]; height];
    curr_garden[start_y][start_x] = true;

    for _ in 0..64 {
        let mut new_garden = vec![vec![false; width]; height];
        for (x, row) in curr_garden.iter().enumerate().take(width) {
            for (y, &element) in row.iter().enumerate().take(height) {
                if element {
                    for p in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                        if is_in_garden(p) {
                            new_garden[p.1][p.0] = curr_garden[x][y];
                        }
                    }
                }
            }
        }
        curr_garden = new_garden;
    }

    curr_garden
        .into_iter()
        .map(|row| row.into_iter().filter(|x| *x).count())
        .sum()
}
