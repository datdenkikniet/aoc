type Modifier = fn((usize, usize)) -> Option<(usize, usize)>;

const TESTS: [fn((usize, usize)) -> Option<(usize, usize)>; 8] = [
    |(x, y): (usize, usize)| Some((x + 1, y)),
    |(x, y): (usize, usize)| Some((x, y + 1)),
    |(x, y): (usize, usize)| Some((x + 1, y + 1)),
    |(x, y): (usize, usize)| x.checked_sub(1).map(|x| (x, y)),
    |(x, y): (usize, usize)| y.checked_sub(1).map(|y| (x, y)),
    |(x, y): (usize, usize)| {
        x.checked_sub(1)
            .and_then(|x| y.checked_sub(1).map(|y| (x, y)))
    },
    |(x, y): (usize, usize)| x.checked_sub(1).map(|x| (x, y + 1)),
    |(x, y): (usize, usize)| y.checked_sub(1).map(|y| (x + 1, y)),
];

fn main() {
    let lines: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().chars().collect())
        .collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<Vec<char>>) {
    let mut starts = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            for test in &TESTS {
                if is_start(['X', 'M', 'A', 'S'], (x, y), lines, test) {
                    starts += 1;
                }
            }
        }
    }

    println!("Part 1: {starts}");
}

fn is_start<const N: usize>(
    str: [char; N],
    mut location: (usize, usize),
    lines: &Vec<Vec<char>>,
    offset_op: &Modifier,
) -> bool {
    let eq_and_inbounds = |(x, y): (usize, usize), val: char| {
        let y = if let Some(y) = lines.get(y) {
            y
        } else {
            return false;
        };

        if let Some(x) = y.get(x) {
            x == &val
        } else {
            false
        }
    };

    for idx in 0..str.len() {
        let next = str[idx];
        if !eq_and_inbounds(location, next) {
            return false;
        }

        if next != 'S' {
            location = if let Some(location) = offset_op(location) {
                location
            } else {
                return false;
            };
        }
    }

    return true;
}

fn part2(lines: &Vec<Vec<char>>) {
    let mut count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if is_x_mas_start((x, y), lines) {
                count += 1;
            }
        }
    }

    println!("Part 2: {count}");
}

fn is_x_mas_start(location: (usize, usize), lines: &Vec<Vec<char>>) -> bool {
    let (x, y) = location;
    let corner_ops = [
        ((x, y), TESTS[2]),
        ((x + 2, y), TESTS[6]),
        ((x, y + 2), TESTS[7]),
        ((x + 2, y + 2), TESTS[5]),
    ];

    let mas_es = corner_ops
        .into_iter()
        .filter(|(location, offset_op)| is_start(['M', 'A', 'S'], *location, lines, offset_op))
        .count();

    mas_es == 2
}
