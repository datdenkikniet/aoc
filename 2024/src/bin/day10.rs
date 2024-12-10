use std::{collections::HashSet, usize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn do_move(
        &self,
        (x, y): (usize, usize),
        (x_len, y_len): (usize, usize),
    ) -> Option<(usize, usize)> {
        let (new_x, new_y) = match self {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x.checked_sub(1)?, y),
        };

        if new_x < x_len && new_y < y_len {
            Some((new_x, new_y))
        } else {
            None
        }
    }

    fn moves(pos: (usize, usize), dims: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        Self::all().flat_map(move |d| d.do_move(pos, dims))
    }

    pub fn all() -> impl Iterator<Item = Direction> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut height_map = Vec::new();

    for line in lines {
        let mut this_line = Vec::new();
        for char in line.chars() {
            if char == '.' {
                this_line.push(usize::MAX);
            } else {
                this_line.push(char as usize - '0' as usize);
            }
        }
        height_map.push(this_line);
    }

    part1(&height_map);
    part2(&height_map);
}

fn walk<F, T>((x, y): (usize, usize), map: &Vec<Vec<usize>>, f: &mut F) -> T
where
    F: FnMut((usize, usize)) -> T,
    T: std::ops::Add<Output = T> + Default,
{
    let height = map[y][x];
    let dims = (map[0].len(), map.len());

    if height == 0 {
        return f((x, y));
    } else {
        let target_height = height - 1;

        let moves = Direction::moves((x, y), dims).filter(|(x, y)| map[*y][*x] == target_height);

        let mut sum = T::default();
        for (x, y) in moves {
            sum = sum + walk((x, y), map, f);
        }
        sum
    }
}

fn part1(height_map: &Vec<Vec<usize>>) {
    let starts = (0..height_map.len())
        .flat_map(|y| (0..height_map[0].len()).map(move |x| (x, y)))
        .filter(|(x, y)| height_map[*y][*x] == 9);

    let score: usize = starts
        .map(|start| {
            let mut trail_heads = HashSet::new();
            let mut found_trail_head = |pos| trail_heads.insert(pos) as usize;
            walk(start, height_map, &mut found_trail_head)
        })
        .sum();

    println!("Part 1: {score}");
}

fn part2(height_map: &Vec<Vec<usize>>) {
    let starts = (0..height_map.len())
        .flat_map(|y| (0..height_map[0].len()).map(move |x| (x, y)))
        .filter(|(x, y)| height_map[*y][*x] == 9);

    let unique_paths: usize = starts
        .map(|start| {
            let mut found_path = |_| 1;
            walk(start, height_map, &mut found_path)
        })
        .sum();

    println!("Part 2: {unique_paths}");
}
