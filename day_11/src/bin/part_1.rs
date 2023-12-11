fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(Debug)]
struct Galaxy {
    x: i32,
    y: i32,
}

fn dilate(mut galaxies: Vec<Galaxy>, width: i32, height: i32) -> Vec<Galaxy> {
    let x_expansions = (0..width)
        .rev()
        .filter(|n| !galaxies.iter().any(|galaxy| galaxy.x == *n))
        .collect::<Vec<_>>();
    let y_expansions = (0..height)
        .rev()
        .filter(|n| !galaxies.iter().any(|galaxy| galaxy.y == *n))
        .collect::<Vec<_>>();

    for x in x_expansions {
        for galaxy in &mut galaxies {
            if galaxy.x > x {
                galaxy.x += 1;
            }
        }
    }
    for y in y_expansions {
        for galaxy in &mut galaxies {
            if galaxy.y > y {
                galaxy.y += 1;
            }
        }
    }
    galaxies
}

fn parse(s: &str) -> (Vec<Galaxy>, (i32, i32)) {
    let mut galaxies = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in s.lines().enumerate() {
        height = y + 1;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            if c == '#' {
                galaxies.push(Galaxy {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    (galaxies, (width as i32, height as i32))
}

fn manhattan_distance(galaxy_1: &Galaxy, galaxy_2: &Galaxy) -> i32 {
    (galaxy_1.x - galaxy_2.x).abs() + (galaxy_1.y - galaxy_2.y).abs()
}

fn distances(galaxies: &[Galaxy]) -> i32 {
    (0..galaxies.len())
        .map(|i| {
            (i + 1..galaxies.len())
                .map(|j| manhattan_distance(&galaxies[i], &galaxies[j]))
                .sum::<i32>()
        })
        .sum()
}

fn process(s: &str) -> i32 {
    let (mut galaxies, (width, height)) = parse(s);
    galaxies = dilate(galaxies, width, height);

    distances(&galaxies)
}
