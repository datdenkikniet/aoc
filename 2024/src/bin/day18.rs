use std::collections::HashMap;

fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let input: Vec<_> = lines
        .map(|l| {
            let (lhs, rhs) = l.split_once(',').unwrap();

            let (lhs, rhs): (usize, usize) = (lhs.parse().unwrap(), rhs.parse().unwrap());

            (lhs, rhs)
        })
        .collect();

    part1(&input);
    part2(&input);
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const FALLEN_BYTES: usize = 1024;

fn part1(falls: &[(usize, usize)]) {
    let mut map: Vec<Vec<_>> = (0..HEIGHT)
        .map(|_| (0..WIDTH).map(|_| false).collect())
        .collect();

    for (x, y) in falls.iter().take(FALLEN_BYTES).cloned() {
        map[y][x] = true;
    }

    let mut path = Vec::new();
    let mut memoized = HashMap::new();
    let result = min_path_len(
        (0, 0),
        (WIDTH - 1, HEIGHT - 1),
        &mut path,
        &map,
        &mut memoized,
    )
    .unwrap();

    println!("Part 1: {result}");
}

fn part2(falls: &[(usize, usize)]) {
    let mut map: Vec<Vec<_>> = (0..HEIGHT)
        .map(|_| (0..WIDTH).map(|_| false).collect())
        .collect();

    let mut fallen_bytes = falls.iter().cloned();

    for (x, y) in (&mut fallen_bytes).take(FALLEN_BYTES) {
        map[y][x] = true;
    }

    for (x, y) in fallen_bytes {
        let mut path = Vec::new();
        let mut memoized = HashMap::new();
        map[y][x] = true;
        let result = min_path_len(
            (0, 0),
            (WIDTH - 1, HEIGHT - 1),
            &mut path,
            &map,
            &mut memoized,
        );

        if result.is_none() {
            println!("Part 2: ({x}, {y})");
            break;
        }
    }
}

fn neighbours(
    (x, y): (usize, usize),
    (x_len, y_len): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [
        ((x + 1 < x_len).then_some(x + 1), Some(y)),
        (Some(x), (y + 1 < y_len).then_some(y + 1)),
        (x.checked_sub(1), Some(y)),
        (Some(x), y.checked_sub(1)),
    ]
    .into_iter()
    .flat_map(|(x, y)| x.and_then(|x| y.map(|y| (x, y))))
}

fn min_path_len(
    current: (usize, usize),
    destination: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    map: &Vec<Vec<bool>>,
    memoized: &mut HashMap<(usize, usize), Option<usize>>,
) -> Option<usize> {
    let x_len = map[0].len();
    let y_len = map.len();

    if current == destination {
        return Some(0);
    }

    if path.contains(&current) {
        return None;
    }

    if let Some(memoized) = memoized.get(&current) {
        return memoized.clone();
    }

    path.push(current);

    let mut min = None;
    for (nb_x, nb_y) in neighbours(current, (x_len, y_len)) {
        if map[nb_y][nb_x] {
            continue;
        }

        let min_from_nb = min_path_len((nb_x, nb_y), destination, path, map, memoized);

        if let Some(min_from_nb) = min_from_nb {
            let len = min_from_nb + 1;
            let min = min.get_or_insert(len);
            *min = (*min).min(len);
        }
    }

    path.pop();

    memoized.insert(current, min);

    min
}
