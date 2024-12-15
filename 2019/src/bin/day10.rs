use std::{collections::HashSet, usize};

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut map = Vec::new();

    for line in lines {
        let mut row = Vec::new();
        for cell in line.chars() {
            let cell = match cell {
                '.' => false,
                '#' => true,
                _ => panic!(),
            };

            row.push(cell);
        }
        map.push(row);
    }

    part1(&map);
}

fn part1(map: &Vec<Vec<bool>>) {
    let y_len = map.len() as isize;
    let x_len = map[0].len() as isize;

    let mut visible = usize::MIN;
    let mut pos = (0, 0);

    for y in 0..y_len {
        for x in 0..x_len {
            if map[y as usize][x as usize] {
                let new_visible = find_visible(map, (x, y));
                if new_visible > visible {
                    visible = new_visible;
                    pos = (x, y);
                }
            }
        }
    }

    println!("Part 1: {visible} @ {pos:?}");
}

fn find_visible(map: &Vec<Vec<bool>>, (x, y): (isize, isize)) -> usize {
    let mut visible = HashSet::new();
    let mut visited = HashSet::new();
    let y_len = map.len() as isize;
    let x_len = map[0].len() as isize;

    // TODO: something with multiples/prime factors

    for y_to in (0..y).rev().chain(y..y_len) {
        for x_to in (0..x).rev().chain(x..x_len) {
            if (x_to, y_to) == (x, y) {
                continue;
            }

            let x_diff = x_to - x;
            let y_diff = y_to - y;

            let ray_directions = [
                (x_diff, y_diff),
                (-x_diff, y_diff),
                (x_diff, -y_diff),
                (-x_diff, -y_diff),
            ];

            for (x_move, y_move) in ray_directions {
                let (mut x, mut y) = (x, y);
                let mut first = None;
                loop {
                    x += x_move;
                    y += y_move;

                    if map
                        .get(y as usize)
                        .map(|v| v.get(x as usize))
                        .flatten()
                        .is_none()
                    {
                        break;
                    }

                    if visited.insert((x, y)) {
                        if first.is_none() && map[y as usize][x as usize] {
                            first = Some((x, y));
                        }
                    }
                }

                if let Some(first) = first {
                    visible.insert(first);
                }
            }
        }
    }

    let total_cells = x_len * y_len;
    assert_eq!(total_cells - 1, visited.len() as _);

    visible.len()
}
