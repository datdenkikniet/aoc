use std::{collections::HashMap, usize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Wall,
    Empty,
}

impl Space {
    pub fn is_wall(&self) -> bool {
        self == &Self::Wall
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    start: (isize, isize),
    end: (isize, isize),
    map: Vec<Vec<Space>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn cost(&self, other: &Direction) -> Option<usize> {
        let value = match (self, other) {
            (Direction::North, Direction::South) => return None,
            (Direction::East, Direction::West) => return None,
            (Direction::South, Direction::North) => return None,
            (Direction::West, Direction::East) => return None,

            (Direction::North, Direction::East) => 1000,
            (Direction::North, Direction::West) => 1000,
            (Direction::East, Direction::North) => 1000,
            (Direction::East, Direction::South) => 1000,
            (Direction::South, Direction::East) => 1000,
            (Direction::South, Direction::West) => 1000,
            (Direction::West, Direction::North) => 1000,
            (Direction::West, Direction::South) => 1000,

            (Direction::North, Direction::North) => 0,
            (Direction::East, Direction::East) => 0,
            (Direction::West, Direction::West) => 0,
            (Direction::South, Direction::South) => 0,
        };

        Some(value)
    }

    pub fn step(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

impl Map {
    pub fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let mut map = Vec::new();
        let mut end = (0, 0);
        let mut reindeer = (0, 0);

        for (y, map_line) in (&mut lines).take_while(|l| !l.is_empty()).enumerate() {
            let mut row = Vec::new();
            for (x, char) in map_line.chars().enumerate() {
                let space = match char {
                    '#' => Space::Wall,
                    '.' => Space::Empty,
                    'S' => {
                        reindeer = (x as isize, y as isize);
                        Space::Empty
                    }
                    'E' => {
                        end = (x as isize, y as isize);
                        Space::Empty
                    }
                    _ => panic!(),
                };

                row.push(space);
            }

            map.push(row);
        }

        Self {
            map,
            start: reindeer,
            end,
        }
    }

    fn neighbors(
        &self,
        current_direction: Direction,
        (x, y): (isize, isize),
    ) -> impl Iterator<Item = (usize, Direction, (isize, isize))> + '_ {
        let directions = [
            Direction::South,
            Direction::North,
            Direction::West,
            Direction::East,
        ];

        directions.into_iter().filter_map(move |dir| {
            let (x, y) = dir.step((x, y));

            if self.map[y as usize][x as usize].is_wall() {
                return None;
            }

            let cost = dir.cost(&current_direction)?;
            let cost = cost.checked_add(1)?;

            Some((cost, dir, (x, y)))
        })
    }

    pub fn shortest_route(&self) -> usize {
        let mut path = Vec::new();
        let mut memoized = HashMap::new();

        self.shortest_route_cost(self.start, Direction::East, &mut path, &mut memoized)
    }

    fn shortest_route_cost(
        &self,
        pos: (isize, isize),
        dir: Direction,
        path: &mut Vec<((isize, isize), Direction)>,
        memoized: &mut HashMap<((isize, isize), Direction), usize>,
    ) -> usize {
        if pos == self.end {
            return 0;
        }

        if let Some(memoized) = memoized.get(&(pos, dir)) {
            return *memoized;
        }

        let mut min = usize::MAX;
        for (cost, nb_dir, nb) in self.neighbors(dir, pos) {
            if path.contains(&(nb, nb_dir)) {
                return usize::MAX;
            }

            path.push((nb, nb_dir));

            let cost_from_here = self.shortest_route_cost(nb, nb_dir, path, memoized);

            if let Some(new_cost) = cost.checked_add(cost_from_here) {
                min = min.min(new_cost);
            }

            path.pop();
        }

        let memoized = memoized.entry((pos, dir)).or_insert(usize::MAX);
        *memoized = (*memoized).min(min);

        min
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());
    let map = Map::parse(lines);

    println!("Part 1: {}", map.shortest_route());
}
