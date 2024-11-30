use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pipe {
    Vertical,
    Horiztonal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let pipe = match value {
            '|' => Self::Vertical,
            '-' => Self::Horiztonal,
            'L' => Self::NorthAndEast,
            'J' => Self::NorthAndWest,
            '7' => Self::SouthAndWest,
            'F' => Self::SouthAndEast,
            _ => return Err(()),
        };

        Ok(pipe)
    }
}

impl From<Pipe> for char {
    fn from(value: Pipe) -> Self {
        match value {
            Pipe::Vertical => '|',
            Pipe::Horiztonal => '-',
            Pipe::NorthAndEast => 'L',
            Pipe::NorthAndWest => 'J',
            Pipe::SouthAndWest => '7',
            Pipe::SouthAndEast => 'F',
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = char::from(*self);
        f.write_char(c)
    }
}

macro_rules ! pipe_directions {
    ($([$pipe:ident, $dir1:ident, $dir2:ident]),*) => {
        impl Pipe {
            pub fn directions(&self) -> &'static [Direction; 2] {
                match self {
                    $(
                        Pipe::$pipe => &[Direction::$dir1, Direction::$dir2],
                    )*
                }
            }

            pub fn connects_to(&self, direction: &Direction) -> bool {
                self.directions().contains(direction)
            }

            pub fn connecting(d1: Direction, d2: Direction) -> Option<Self> {
                if d1 == d2 {
                    return None;
                }

                let pipe = match (d1, d2) {
                    $(
                        (Direction::$dir1, Direction::$dir2) | (Direction::$dir2, Direction::$dir1) => {
                            Self::$pipe
                        }
                    )*
                    _ => unreachable!(),
                };

                Some(pipe)
            }

            pub fn other_dir(&self, direction: &Direction) -> Direction {
                match self {
                    $(
                        Pipe::$pipe => if direction == &Direction::$dir1 {
                            Direction::$dir2
                        } else {
                            Direction::$dir1
                        }
                    )*
                }
            }
        }
    }
}

pipe_directions!(
    [Vertical, North, South],
    [Horiztonal, West, East],
    [NorthAndEast, North, East],
    [NorthAndWest, North, West],
    [SouthAndWest, South, West],
    [SouthAndEast, South, East]
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Pipe(p) => p.fmt(f),
            Tile::Ground => f.write_char('.'),
            Tile::Start => f.write_char('S'),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = if let Ok(pipe) = Pipe::try_from(value) {
            Self::Pipe(pipe)
        } else {
            match value {
                '.' => Self::Ground,
                'S' => Self::Start,
                _ => return Err(()),
            }
        };

        Ok(tile)
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    starting_pos: (usize, usize),
}

impl Map {
    fn get_pipe(&self, row: usize, col: usize) -> Option<Pipe> {
        if (row, col) == self.starting_pos {
            return Some(self.starting_type());
        }

        if row < self.tiles.len() {
            let row = &self.tiles[row];
            if col < row.len() {
                let value = row[col];
                if let Tile::Pipe(p) = value {
                    return Some(p);
                }
            }
        }

        None
    }

    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let starting_pos = tiles
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(col_idx, col)| {
                    if *col == Tile::Start {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        Self {
            tiles,
            starting_pos,
        }
    }

    pub fn starting_type(&self) -> Pipe {
        let (r, c) = self.starting_pos;

        let ch = |row, col, connecting: &[Pipe]| {
            if let Some(pipe) = self.get_pipe(row, col) {
                connecting.iter().any(|d| d == &pipe)
            } else {
                false
            }
        };

        let above_pipes = &[Pipe::Vertical, Pipe::SouthAndEast, Pipe::SouthAndWest];
        let above = (r > 0).then(|| ch(r - 1, c, above_pipes)).unwrap_or(false);

        let below_pipes = &[Pipe::Vertical, Pipe::NorthAndEast, Pipe::NorthAndWest];
        let below = ch(r + 1, c, below_pipes);

        let left_pipes = &[Pipe::Horiztonal, Pipe::NorthAndWest, Pipe::SouthAndWest];
        let left = (c > 0).then(|| ch(r, c - 1, left_pipes)).unwrap_or(false);

        let right_pipes = &[Pipe::Horiztonal, Pipe::NorthAndEast, Pipe::SouthAndEast];
        let right = ch(r, c + 1, right_pipes);

        match (above, below, left, right) {
            (true, true, _, _) => Pipe::Vertical,
            (true, _, true, _) => Pipe::NorthAndWest,
            (true, _, _, true) => Pipe::NorthAndEast,
            (_, true, true, _) => Pipe::SouthAndWest,
            (_, true, _, true) => Pipe::SouthAndEast,
            (_, _, true, true) => Pipe::Horiztonal,
            _ => unreachable!(),
        }
    }

    pub fn walk(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        struct PipeIter<'a> {
            map: &'a Map,
            previous_pos: Option<(usize, usize, Direction)>,
        }

        impl Iterator for PipeIter<'_> {
            type Item = (usize, usize);

            fn next(&mut self) -> Option<Self::Item> {
                if let Some((row, col, incoming_dir)) = self.previous_pos {
                    let pipe = self.map.get_pipe(row, col).unwrap();
                    let outgoing_dir = pipe.other_dir(&incoming_dir.opposite());

                    let next_pos = match outgoing_dir {
                        Direction::North => (row - 1, col),
                        Direction::East => (row, col + 1),
                        Direction::South => (row + 1, col),
                        Direction::West => (row, col - 1),
                    };

                    if next_pos == self.map.starting_pos {
                        return None;
                    }

                    self.previous_pos = Some((next_pos.0, next_pos.1, outgoing_dir));
                    Some(next_pos)
                } else {
                    let row = self.map.starting_pos.0;
                    let col = self.map.starting_pos.1;
                    let incoming_dir = match self.map.starting_type() {
                        Pipe::Vertical => Direction::North,
                        Pipe::Horiztonal => Direction::East,
                        Pipe::NorthAndEast => Direction::East,
                        Pipe::NorthAndWest => Direction::North,
                        Pipe::SouthAndWest => Direction::West,
                        Pipe::SouthAndEast => Direction::East,
                    };
                    self.previous_pos = Some((row, col, incoming_dir));

                    Some(self.map.starting_pos)
                }
            }
        }

        PipeIter {
            map: self,
            previous_pos: None,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.tiles.iter().peekable();
        while let Some(row) = rows.next() {
            for column in row {
                column.fmt(f)?;
            }

            if rows.peek().is_some() {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let mut tiles = Vec::new();

    for line in lines {
        tiles.push(line.chars().map(|v| Tile::try_from(v).unwrap()).collect());
    }

    let map = Map::new(tiles);

    let steps = map.walk().count();
    println!("{}", steps / 2);

    Ok(())
}
