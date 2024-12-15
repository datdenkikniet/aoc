use std::{collections::VecDeque, task::Poll};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Wall,
    Empty,
    Box,
}

#[derive(Debug, Clone)]
pub struct Map {
    bot: (isize, isize),
    map: Vec<Vec<Space>>,
    moves: VecDeque<Move>,
}

impl Map {
    pub fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let mut map = Vec::new();
        let mut bot = (0, 0);

        for (y, map_line) in (&mut lines).take_while(|l| !l.is_empty()).enumerate() {
            let mut row = Vec::new();
            for (x, char) in map_line.chars().enumerate() {
                let space = match char {
                    '#' => Space::Wall,
                    'O' => Space::Box,
                    '.' => Space::Empty,
                    '@' => {
                        bot = (x as isize, y as isize);
                        Space::Empty
                    }
                    _ => panic!(),
                };

                row.push(space);
            }

            map.push(row);
        }

        let mut moves = VecDeque::new();
        for line in lines {
            for char in line.chars() {
                let the_move = match char {
                    '^' => Move::Up,
                    '>' => Move::Right,
                    '<' => Move::Left,
                    'v' => Move::Down,
                    v => panic!("{v}"),
                };

                moves.push_back(the_move);
            }
        }

        Self { map, bot, moves }
    }

    fn can_move(&self, (move_x, move_y): (isize, isize)) -> Option<(isize, isize)> {
        let (mut x, mut y) = self.bot;

        loop {
            x = x + move_x;
            y = y + move_y;

            let space = self.map[y as usize][x as usize];

            match space {
                Space::Wall => return None,
                Space::Empty => return Some((x, y)),
                Space::Box => continue,
            }
        }
    }

    pub fn poll_move(&mut self) -> core::task::Poll<()> {
        if let Some(the_move) = self.moves.pop_front() {
            let (x_diff, y_diff) = match the_move {
                Move::Up => (0, -1),
                Move::Down => (0, 1),
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
            };

            if let Some(mut first_empty) = self.can_move((x_diff, y_diff)) {
                let (x, y) = self.bot;
                self.bot = (x + x_diff, y + y_diff);

                let bot_pos = self.bot;

                while first_empty != bot_pos {
                    let (x_to, y_to) = first_empty;
                    let (x_from, y_from) = (x_to - x_diff, y_to - y_diff);

                    self.map[y_to as usize][x_to as usize] = Space::Box;
                    self.map[y_from as usize][x_from as usize] = Space::Empty;

                    first_empty = (x_from, y_from);
                }
            }
        }

        if self.moves.is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }

    pub fn box_gps_distances(&self) -> impl Iterator<Item = usize> + '_ {
        self.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, space)| {
                if space == &Space::Box {
                    Some(y * 100 + x)
                } else {
                    None
                }
            })
        })
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());
    let map = Map::parse(lines);

    part1(map.clone());
}

fn part1(mut map: Map) {
    while map.poll_move().is_pending() {}

    let box_sum: usize = map.box_gps_distances().sum();

    println!("Part 1: {box_sum}");
}
