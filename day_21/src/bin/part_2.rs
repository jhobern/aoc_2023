fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

// MUST ALWAYS BE ODD OR ELSE THE STARTING VALUE CHOSEN WILL ACTUALLY BE IN A CORNER
const REPEATS: usize = 5;
const STEPS: usize = 26501365;

fn parse_input(s: &str) -> Vec<Vec<bool>> {
    let s = s
        .lines()
        .map(|line| {
            let mut line = line.repeat(REPEATS);
            line.push('\n');
            line
        })
        .collect::<String>()
        .repeat(REPEATS);
    let garden = s
        .lines()
        .map(|line| line.chars().map(|c| c != '#').collect::<Vec<_>>())
        .collect();
    garden
}

// This solution uses the fact that the the increases in the total number of places the gardener can reach
// has a regular increase, when taken over a range of `board_width`
// i.e. After a while, the increase from step `n` to step `n + board_width` is a constant `c` more than
// the increase from `n - board_width` to `n`. This yields a quadratic equation that can be used to find values
// arbitrarily far ahead in multiples of `board_width`
// For example, for my board, with a `board_width` of 131, we find that the number of steps, 26501365, is
// 202300 * `board_width` + 65, so we just need to find some value for n * `board_width` * 65, and then find
// the constant step between the increases and we can extrapolate.
// In my case, the first 6 values for n * `board_width` + 65 along with the differences from the preview values were:
// 3941   -> undefined -> undefined
// 35259  -> 31318     -> undefined
// 97807  -> 62548     -> 31230
// 191585 -> 93778     -> 31230
// 316593 -> 125008    -> 31230
// 472831 -> 156238    -> 31230
//
// Which shows that the values for n extra `board_width`s above the third value, 97808, will be given by:
// 97807 + (62548 + 31230) + (62548 + 31230 + 31230) + (62548 + 31230 + 31230 + 31230) + ... + (62548 + 31230 * n)
// which reduces to:
// 97807 + (62548 * n) + (31230 * (n + 1) * n / 2)
//
// For grids that don't have the property of having direct empty lines to every edge I have found that you sometimes
// need to get a larger set of values before you can find the constant increase. The example data required about 6
// values before the difference converged. If you have one of these grids you just need to increase the value
// of `REPEATS` and increase the multiplier in `check_offset` until your value converges. `REPEATS` must ALWAYS be odd.
fn process(s: &str) -> u128 {
    let original_gardens = parse_input(s);
    let width = original_gardens[0].len();
    let height = original_gardens.len();
    let (start_x, start_y) = ((width) / 2, (height) / 2);

    let is_in_garden = |(x, y): (i64, i64)| -> bool {
        x >= 0
            && x < width as i64
            && y >= 0
            && y < height as i64
            && original_gardens[y as usize][x as usize]
    };

    let individual_width = width / REPEATS;

    let mut curr_garden = vec![vec![false; width]; height];
    let mut counts = Vec::new();
    curr_garden[start_y][start_x] = true;

    let check_offset = STEPS % individual_width + 2 * individual_width;

    for _ in 0..=check_offset {
        let count = curr_garden
            .iter()
            .map(|row: &Vec<bool>| row.iter().filter(|x| **x).count() as u128)
            .sum::<u128>();
        counts.push(count);

        let mut new_garden = vec![vec![false; width]; height];
        for (x, row) in curr_garden.iter().enumerate().take(width) {
            for (y, &element) in row.iter().enumerate().take(height) {
                if element {
                    for p in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                        if is_in_garden((p.0 as i64, p.1 as i64)) {
                            new_garden[p.1][p.0] = curr_garden[x][y];
                        }
                    }
                }
            }
        }
        curr_garden = new_garden;
    }

    let x1 = counts[STEPS % individual_width];
    let x2 = counts[STEPS % individual_width + individual_width];
    let x3 = counts[STEPS % individual_width + individual_width * 2];

    let repeat_difference = x3 - 2 * x2 + x1;
    let first_difference = x3 - x2;
    let first_value = x3;

    //subtract 4 because we are already 4 iterations deep at this point
    let n = (STEPS / individual_width - 4) as u128;

    first_value + n * first_difference + (n + 1) * n * repeat_difference / 2
}
