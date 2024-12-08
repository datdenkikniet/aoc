type AntennaMap = Vec<((usize, usize), char)>;

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let (dim, antenna_locations) = parse(&lines);

    part1(dim, &antenna_locations);
    part2(dim, &antenna_locations);
}

fn parse(input: &[String]) -> ((usize, usize), AntennaMap) {
    let mut antenna_locations = Vec::new();
    let mut x_len = 0;
    let mut y_len = 0;

    for (y, line) in input.into_iter().enumerate() {
        y_len += 1;
        let mut x_len_inner = 0;
        for (x, char) in line.chars().enumerate() {
            x_len_inner += 1;
            if char != '.' {
                antenna_locations.push(((x, y), char));
            }
        }
        x_len = x_len_inner
    }

    ((x_len, y_len), antenna_locations)
}

fn count(map: Vec<Vec<bool>>) -> usize {
    map.into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|v| *v)
        .count()
}

fn part1((x_len, y_len): (usize, usize), antenna_locations: &AntennaMap) {
    let mut map: Vec<_> = (0..y_len).map(|_| vec![false; x_len]).collect();

    for ((x1, y1), freq1) in antenna_locations.iter().cloned() {
        for ((x2, y2), freq2) in antenna_locations.iter().cloned() {
            if freq1 != freq2 || (x1, y1) == (x2, y2) {
                continue;
            }

            for (x, y) in calculate_antinode_positions_p1((x1, y1), (x2, y2)) {
                if x < x_len && y < y_len {
                    map[y][x] = true;
                }
            }
        }
    }

    let antinodes = count(map);

    println!("Part 1: {antinodes}");
}

fn calculate_antinode_positions_p1(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut positions = Vec::with_capacity(2);

    let x_distance = x1.max(x2) - x1.min(x2);
    let y_distance = y1.max(y2) - y1.min(y2);

    let (anx1, anx2) = if x2 >= x1 {
        (x1.checked_sub(x_distance), Some(x2 + x_distance))
    } else {
        (Some(x1 + x_distance), x2.checked_sub(x_distance))
    };

    let (any1, any2) = if y2 >= y1 {
        (y1.checked_sub(y_distance), Some(y2 + y_distance))
    } else {
        (Some(y1 + y_distance), y2.checked_sub(y_distance))
    };

    if let (Some(anx1), Some(any1)) = (anx1, any1) {
        positions.push((anx1, any1));
    }

    if let (Some(anx2), Some(any2)) = (anx2, any2) {
        positions.push((anx2, any2));
    }

    positions
}

fn part2((x_len, y_len): (usize, usize), antenna_locations: &AntennaMap) {
    let mut map: Vec<_> = (0..y_len).map(|_| vec![false; x_len]).collect();

    for (p1, freq1) in antenna_locations.iter().cloned() {
        for (p2, freq2) in antenna_locations.iter().cloned() {
            if freq1 == freq2 && p1 != p2 {
                for (x, y) in calculate_antinode_positions_p2(p1, p2, (x_len, y_len)) {
                    map[y][x] = true;
                }
            }
        }
    }

    let antinodes = count(map);

    println!("Part 2: {antinodes}");
}

fn calculate_antinode_positions_p2(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (x_len, y_len): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut positions = vec![(x1, y1), (x2, y2)];

    let x_distance = x1.max(x2) - x1.min(x2);
    let y_distance = y1.max(y2) - y1.min(y2);

    let (mut x, mut y) = (x1, y1);

    loop {
        let new_x = if x2 >= x1 {
            x.checked_sub(x_distance)
        } else {
            Some(x + x_distance)
        };

        let new_y = if y2 >= y1 {
            y.checked_sub(y_distance)
        } else {
            Some(y + y_distance)
        };

        if let (Some(new_x), Some(new_y)) = (new_x, new_y) {
            if new_x < x_len && new_y < y_len {
                positions.push((new_x, new_y));
                x = new_x;
                y = new_y;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    positions
}
