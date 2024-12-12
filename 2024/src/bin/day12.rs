fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|v| v.unwrap()).collect();
    let regions = parse(&input);

    part1(&regions);
}

fn part1(regions: &[Region]) {
    let mut sum = 0;
    for region in regions {
        sum += region.area() * region.fence();
    }

    println!("Part 1: {sum}");
}

fn parse(input: &[String]) -> Vec<Region> {
    let mut regions = Vec::new();
    let input: Vec<Vec<char>> = input.into_iter().map(|v| v.chars().collect()).collect();

    let x_len = input[0].len();
    let y_len = input.len();
    let mut visited_positions = Vec::new();

    for y in 0..y_len {
        for x in 0..x_len {
            if visited_positions.contains(&(x, y)) {
                continue;
            }

            let (region, _) = Region::new(&input, (x, y));

            for pos in region.set_positions() {
                visited_positions.push(pos);
            }

            regions.push(region)
        }
    }

    regions
}

#[derive(Debug)]
struct Region {
    rows: Vec<Vec<bool>>,
}

impl Region {
    pub fn new(input: &Vec<Vec<char>>, start: (usize, usize)) -> (Self, char) {
        let width = input[0].len();
        let height = input.len();

        let rows: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        let mut me = Self { rows };
        let char = input[start.1][start.0];
        me.build_rec(char, start, input);

        (me, char)
    }

    fn build_rec(&mut self, v: char, pos: (usize, usize), input: &Vec<Vec<char>>) {
        if input[pos.1][pos.0] == v && !self.get(pos) {
            self.set(pos, true);

            for adjacent in self.adjacent_positions(pos).flat_map(|v| v) {
                self.build_rec(v, adjacent, input);
            }
        }
    }

    pub fn fence(&self) -> usize {
        let mut fence = 0;
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                if self.get((x, y)) {
                    for adjacent in self.adjacent_positions((x, y)) {
                        if let Some(pos) = adjacent {
                            if !self.get(pos) {
                                fence += 1;
                            }
                        } else {
                            fence += 1;
                        }
                    }
                }
            }
        }

        fence
    }

    pub fn adjacent_positions(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = Option<(usize, usize)>> {
        let xs = [
            Some(x),
            Some(x),
            x.checked_add(1).filter(|v| *v < self.x_len()),
            x.checked_sub(1),
        ];

        let ys = [
            y.checked_add(1).filter(|v| *v < self.y_len()),
            y.checked_sub(1),
            Some(y),
            Some(y),
        ];

        xs.into_iter()
            .zip(ys.into_iter())
            .map(|(x, y)| x.and_then(|x| y.map(|y| (x, y))))
    }

    pub fn area(&self) -> usize {
        self.set_positions().count()
    }

    fn y_len(&self) -> usize {
        self.rows.len()
    }
    fn x_len(&self) -> usize {
        self.rows[0].len()
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: bool) {
        self.rows[y][x] = value;
    }

    pub fn get(&self, (x, y): (usize, usize)) -> bool {
        self.rows[y][x]
    }

    pub fn set_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.y_len())
            .flat_map(|y| (0..self.x_len()).map(move |x| (x, y)))
            .filter(|(x, y)| self.rows[*y][*x])
    }
}
