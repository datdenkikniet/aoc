use line::Line;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn do_move(&self, r#move: &Move) -> Self {
        let count = r#move.count;
        match r#move.direction {
            Direction::Up => Self::new(self.x, self.y - count),
            Direction::Right => Self::new(self.x + count, self.y),
            Direction::Down => Self::new(self.x, self.y + count),
            Direction::Left => Self::new(self.x - count, self.y),
        }
    }

    pub fn distance(&self, other: &Self) -> isize {
        let x = self.x.max(other.x) - self.x.min(other.x);
        let y = self.y.max(other.y) - self.y.min(other.y);
        x + y
    }
}

mod line {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Line {
        UpDown {
            x: isize,
            y_start: isize,
            y_end: isize,
        },
        LeftRight {
            y: isize,
            x_start: isize,
            x_end: isize,
        },
    }

    impl Line {
        pub fn new(from: Point, to: Point) -> Self {
            if from.x == to.x {
                Self::UpDown {
                    x: from.x,
                    y_start: from.y,
                    y_end: to.y,
                }
            } else if from.y == to.y {
                Self::LeftRight {
                    y: from.y,
                    x_start: from.x,
                    x_end: to.x,
                }
            } else {
                panic!();
            }
        }

        pub fn steps(&self) -> usize {
            match *self {
                Line::UpDown { y_start, y_end, .. } => y_start.abs_diff(y_end),
                Line::LeftRight { x_start, x_end, .. } => x_start.abs_diff(x_end),
            }
        }

        pub fn steps_to(&self, point: Point) -> Option<usize> {
            match *self {
                Line::UpDown { x, y_start, y_end } => {
                    let y_min = y_start.min(y_end);
                    let y_max = y_start.max(y_end);
                    if point.x == x && point.y >= y_min && point.y <= y_max {
                        Some(point.y.abs_diff(y_start))
                    } else {
                        None
                    }
                }
                Line::LeftRight { y, x_start, x_end } => {
                    let x_min = x_start.min(x_end);
                    let x_max = x_start.max(x_end);
                    if point.y == y && point.x >= x_min && point.x <= x_max {
                        Some(point.x.abs_diff(x_start))
                    } else {
                        None
                    }
                }
            }
        }

        pub fn intersection(&self, other: &Self) -> Option<Point> {
            let (
                updown_x,
                updown_y_min,
                updown_y_max,
                leftright_y,
                leftright_x_min,
                leftright_x_max,
            ) = match (*self, *other) {
                (Self::UpDown { x, y_start, y_end }, Self::LeftRight { y, x_start, x_end })
                | (Self::LeftRight { y, x_start, x_end }, Self::UpDown { x, y_start, y_end }) => (
                    x,
                    y_start.min(y_end),
                    y_start.max(y_end),
                    y,
                    x_start.min(x_end),
                    x_start.max(x_end),
                ),

                _ => return None,
            };

            if leftright_y <= updown_y_min || leftright_y >= updown_y_max {
                return None;
            }

            if updown_x <= leftright_x_min || updown_x >= leftright_x_max {
                return None;
            }

            // Intersecting :)

            let offset = updown_x - leftright_x_min;
            let x = leftright_x_min + offset;

            let offset = leftright_y - updown_y_min;
            let y = updown_y_min + offset;

            Some(Point::new(x, y))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    count: isize,
    direction: Direction,
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());
    let wire1 = parse(&lines.next().unwrap());
    let wire2 = parse(&lines.next().unwrap());

    let wire1 = build_lines(&wire1);
    let wire2 = build_lines(&wire2);

    let intersections: Vec<_> = wire1
        .iter()
        .flat_map(|w1| wire2.iter().map(move |w2| (w2, w1)))
        .flat_map(|(w1, w2)| w1.intersection(w2))
        .collect();

    part1(&intersections);
    part2(&wire1, &wire2, &intersections);
}

fn part1(intersections: &[Point]) {
    let origin = Point::new(0, 0);

    let min = intersections
        .iter()
        .map(|v| origin.distance(v))
        .min()
        .unwrap();

    println!("Part 1: {min:?}");
}

fn part2(wire1: &[Line], wire2: &[Line], intersections: &[Point]) {
    let mut min_distance = usize::MAX;
    for intersection in intersections.iter().cloned() {
        let sum = walk_to(&wire1, intersection) + walk_to(&wire2, intersection);
        min_distance = sum.min(min_distance);
    }

    println!("Part 2: {min_distance}");
}

fn walk_to(wire: &[Line], point: Point) -> usize {
    let mut distance = 0;

    for line in wire {
        if let Some(dist) = line.steps_to(point) {
            distance += dist;
            break;
        } else {
            distance += line.steps();
        }
    }

    distance
}

fn parse(line: &str) -> Vec<Move> {
    line.split(',')
        .map(|m| {
            let mut chars = m.chars();

            let direction = match chars.next().unwrap() {
                'U' => Direction::Up,
                'R' => Direction::Right,
                'D' => Direction::Down,
                'L' => Direction::Left,
                _ => panic!(),
            };

            let count: isize = chars.collect::<String>().parse().unwrap();

            Move { count, direction }
        })
        .collect()
}

fn build_lines(moves: &[Move]) -> Vec<Line> {
    let mut from = Point::new(0, 0);
    let mut lines = Vec::new();

    for r#move in moves {
        let to = from.do_move(r#move);
        lines.push(Line::new(from, to));
        from = to;
    }

    lines
}
