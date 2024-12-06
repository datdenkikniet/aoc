use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    pub fn make_move(
        &self,
        (x, y): (usize, usize),
        (x_len, y_len): (usize, usize),
    ) -> Option<(usize, usize)> {
        let (x, y) = match self {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x.checked_sub(1)?, y),
        };

        if x >= x_len {
            return None;
        }

        if y >= y_len {
            return None;
        }

        return Some((x, y));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Location {
    Obstructed,
    NotVisited,
    Visited,
}

impl Location {
    pub fn is_obstructed(&self) -> bool {
        *self == Self::Obstructed
    }
}

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut map = Vec::new();
    let mut guard_position = (0, 0);

    let mut y = 0;
    for line in input {
        let mut row = Vec::new();
        let mut x = 0;
        for char in line.chars() {
            match char {
                '.' => row.push(Location::NotVisited),
                '^' => {
                    row.push(Location::Visited);
                    guard_position = (x, y);
                }
                _ => row.push(Location::Obstructed),
            }
            x += 1;
        }
        map.push(row);
        y += 1;
    }

    part1(map.clone(), guard_position);
    part2(map.clone(), guard_position);
}

fn walk(mut map: Vec<Vec<Location>>, mut guard_position: (usize, usize)) -> Option<usize> {
    let mut direction = Direction::North;
    let (y_len, x_len) = (map.len(), map[0].len());

    let mut walked_directions: Vec<Vec<[bool; 4]>> = (0..y_len)
        .map(|_| (0..x_len).map(|_| [false; 4]).collect())
        .collect();

    walked_directions[guard_position.1][guard_position.0][direction as usize] = true;

    loop {
        let mut next_position = None;

        for _ in 0..3 {
            if let Some((new_x, new_y)) = direction.make_move(guard_position, (x_len, y_len)) {
                if !map[new_y][new_x].is_obstructed() {
                    next_position = Some((new_x, new_y));
                    break;
                } else {
                    direction.rotate_right();
                }
            }
        }

        if let Some((new_x, new_y)) = next_position {
            guard_position = (new_x, new_y);
            map[new_y][new_x] = Location::Visited;

            if walked_directions[new_y][new_x][direction as usize] {
                return None;
            }

            walked_directions[new_y][new_x][direction as usize] = true;
        } else {
            break;
        }
    }

    let count = map
        .iter()
        .flat_map(|v| v.iter())
        .filter(|p| **p == Location::Visited)
        .count();

    Some(count)
}

fn part1(map: Vec<Vec<Location>>, guard_position: (usize, usize)) {
    let count = walk(map, guard_position).unwrap();
    println!("Part 1: {count}");
}

fn part2(map: Vec<Vec<Location>>, guard_position: (usize, usize)) {
    let (y_len, x_len) = (map.len(), map[0].len());

    let mut loops = 0;
    let start = Instant::now();
    for x in 0..x_len {
        for y in 0..y_len {
            let mut map = map.clone();

            if map[y][x].is_obstructed() {
                continue;
            } else if (x, y) == guard_position {
                continue;
            }

            map[y][x] = Location::Obstructed;
            if walk(map, guard_position).is_none() {
                loops += 1;
            }
        }
    }

    println!("Part 2: {loops}. {} ms", start.elapsed().as_millis());
}
