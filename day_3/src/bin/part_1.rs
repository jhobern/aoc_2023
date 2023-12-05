#[derive(Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct NumRange {
    value: usize,
    start: usize,
    end: usize,
    row: usize,
}

fn main() {
    let input = include_str!("input.txt");

    let lines = input.lines().enumerate();
    let mut numbers = Vec::new();
    let mut parts = Vec::new();

    for (row, line) in lines {
        let mut offset = 0;
        let line_chars = line.chars().collect::<Vec<_>>();

        while offset < line.len() {
            if line_chars[offset].is_numeric() {
                let mut i = 1;
                while offset + i < line.len() && line_chars[offset + i].is_numeric() {
                    i += 1;
                }

                let n = line.get(offset..offset + i).unwrap().parse().unwrap();

                numbers.push(NumRange {
                    row: row,
                    value: n,
                    start: offset,
                    end: offset + i - 1,
                });
                offset += i;
                continue;
            }

            if line_chars[offset] != '.' {
                parts.push(Pos { x: offset, y: row });
            }

            offset += 1;
        }
    }
    let mut sum = 0;
    for number in numbers {
        for part in parts.iter() {
            if part.y.abs_diff(number.row) <= 1
                && part.x + 1 >= number.start
                && number.end + 1 >= part.x
            {
                sum += number.value;
                break;
            }
        }
    }

    println!("{}", sum);
}
