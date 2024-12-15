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
    BoxLeft,
    BoxRight,
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

    fn expand(&mut self) {
        assert!(!self
            .map
            .iter()
            .flat_map(|r| r)
            .any(|c| c == &Space::BoxLeft || c == &Space::BoxRight));

        let mut new_map = Vec::new();
        for row in &self.map {
            let mut new_row = Vec::new();
            for cell in row {
                let new_cells = match cell {
                    Space::Wall => [Space::Wall, Space::Wall],
                    Space::Empty => [Space::Empty, Space::Empty],
                    Space::Box => [Space::BoxLeft, Space::BoxRight],
                    _ => unreachable!(),
                };

                new_row.extend(new_cells);
            }
            new_map.push(new_row);
        }

        self.map = new_map;
        self.bot.0 = self.bot.0 * 2;
    }

    fn can_move(
        &self,
        (current_x, current_y): (isize, isize),
        (move_x, move_y): (isize, isize),
    ) -> bool {
        let (to_x, to_y) = (current_x + move_x, current_y + move_y);
        let to_space = self.map[to_y as usize][to_x as usize];
        let is_sideways = move_y == 0;

        match to_space {
            Space::Empty => true,
            Space::Wall => false,
            Space::Box => self.can_move((to_x, to_y), (move_x, move_y)),
            Space::BoxLeft | Space::BoxRight if is_sideways => {
                self.can_move((to_x, to_y), (move_x, move_y))
            }
            Space::BoxLeft => {
                self.can_move((to_x + 1, to_y), (move_x, move_y))
                    && self.can_move((to_x, to_y), (move_x, move_y))
            }
            Space::BoxRight => {
                self.can_move((to_x - 1, to_y), (move_x, move_y))
                    && self.can_move((to_x, to_y), (move_x, move_y))
            }
        }
    }

    fn do_move(
        &mut self,
        (current_x, current_y): (isize, isize),
        (move_x, move_y): (isize, isize),
    ) {
        let (to_x, to_y) = (current_x + move_x, current_y + move_y);
        let to_space = self.map[to_y as usize][to_x as usize];
        let is_sideways = move_y == 0;

        match to_space {
            Space::Wall => panic!(),
            Space::Empty => {}
            Space::Box => self.do_move((to_x, to_y), (move_x, move_y)),
            Space::BoxLeft | Space::BoxRight if is_sideways => {
                self.do_move((to_x, to_y), (move_x, move_y));
            }
            Space::BoxLeft => {
                self.do_move((to_x, to_y), (move_x, move_y));
                self.do_move((to_x + 1, to_y), (move_x, move_y));
            }
            Space::BoxRight => {
                self.do_move((to_x, to_y), (move_x, move_y));
                self.do_move((to_x - 1, to_y), (move_x, move_y));
            }
        }

        let current = self.get((current_x, current_y));
        self.set((to_x, to_y), current);
        self.set((current_x, current_y), Space::Empty);
    }

    fn try_move(&mut self, current: (isize, isize), to_move: (isize, isize)) -> bool {
        if self.can_move(current, to_move) {
            self.do_move(current, to_move);
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, (x, y): (isize, isize), space: Space) {
        self.map[y as usize][x as usize] = space;
    }

    pub fn get(&self, (x, y): (isize, isize)) -> Space {
        self.map[y as usize][x as usize]
    }

    pub fn poll_move(&mut self) -> core::task::Poll<()> {
        if let Some(the_move) = self.moves.pop_front() {
            let (x_diff, y_diff) = match the_move {
                Move::Up => (0, -1),
                Move::Down => (0, 1),
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
            };

            let (x, y) = self.bot;

            if self.try_move(self.bot, (x_diff, y_diff)) {
                self.bot = (x + x_diff, y + y_diff);
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
                if space == &Space::BoxLeft || space == &Space::Box {
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
    part2(map.clone());
}

fn part1(mut map: Map) {
    while map.poll_move().is_pending() {}

    let box_sum: usize = map.box_gps_distances().sum();

    println!("Part 1: {box_sum}");
}

fn part2(mut map: Map) {
    map.expand();

    while map.poll_move().is_pending() {}

    let box_sum: usize = map.box_gps_distances().sum();

    println!("Part 2: {box_sum}");
}
