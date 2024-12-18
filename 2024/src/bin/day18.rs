use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    time::Instant,
};

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

    let a_star = min_path_len_astar((0, 0), (WIDTH - 1, HEIGHT - 1), &map).unwrap() - 1;

    println!("Part 2: {a_star}");
}

fn part2(falls: &[(usize, usize)]) {
    let mut map: Vec<Vec<_>> = (0..HEIGHT)
        .map(|_| (0..WIDTH).map(|_| false).collect())
        .collect();

    let mut fallen_bytes = falls.iter().cloned();

    for (x, y) in (&mut fallen_bytes).take(FALLEN_BYTES) {
        map[y][x] = true;
    }

    let mut map_clone = map.clone();
    let bytes_clone = fallen_bytes.clone();

    let start = Instant::now();
    let mut first_result = None;
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
            println!("Part 2: ({x}, {y}) in {} ms", start.elapsed().as_millis());
            first_result = Some((x, y));
            break;
        }
    }

    let start = Instant::now();
    for (x, y) in bytes_clone {
        map_clone[y][x] = true;
        let result = min_path_len_astar((0, 0), (WIDTH - 1, HEIGHT - 1), &map_clone);

        if result.is_none() {
            println!(
                "Part 2 A*: ({x}, {y}) in {} ms",
                start.elapsed().as_millis()
            );
            assert_eq!(Some((x, y)), first_result);
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

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.abs_diff(to.0) + from.1.abs_diff(to.1)
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    mut current: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let mut total_path = VecDeque::new();
    total_path.push_front(current);

    while let Some(previous) = came_from.get(&current).cloned() {
        current = previous;
        total_path.push_front(current);
    }

    total_path.into_iter()
}

fn min_path_len_astar(
    start: (usize, usize),
    goal: (usize, usize),
    map: &Vec<Vec<bool>>,
) -> Option<usize> {
    struct HeapEntry {
        pos: (usize, usize),
        score: usize,
    }

    impl PartialEq for HeapEntry {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl Eq for HeapEntry {}

    impl PartialOrd for HeapEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.score.partial_cmp(&other.score)
        }
    }

    impl Ord for HeapEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.score.cmp(&other.score)
        }
    }

    // Using manhattan distance is OK: we cannot travel diagonally efficiently.
    let h = manhattan_distance;

    // Heap is normally max-heap. Use Reverse to get a min-heap :)
    let mut open_set = BinaryHeap::new();
    open_set.push(std::cmp::Reverse(HeapEntry {
        pos: start,
        score: h(start, goal),
    }));

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some(std::cmp::Reverse(HeapEntry { pos: current, .. })) = open_set.pop() {
        if current == goal {
            return Some(reconstruct_path(came_from, current).count());
        }

        for (nb_x, nb_y) in neighbours(current, (map[0].len(), map.len())) {
            if map[nb_y][nb_x] {
                continue;
            }

            let tentative_gscore = g_score.get(&current).unwrap() + 1;

            if tentative_gscore < g_score.get(&(nb_x, nb_y)).cloned().unwrap_or(usize::MAX) {
                came_from.insert((nb_x, nb_y), current);
                g_score.insert((nb_x, nb_y), tentative_gscore);

                let f_score = tentative_gscore + h((nb_x, nb_y), goal);
                if !open_set.iter().any(|e| e.0.pos == (nb_x, nb_y)) {
                    open_set.push(std::cmp::Reverse(HeapEntry {
                        pos: (nb_x, nb_y),
                        score: f_score,
                    }));
                }
            }
        }
    }

    None
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
