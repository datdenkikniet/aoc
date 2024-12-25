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
}
fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());
    let map = Map::parse(lines);
}
