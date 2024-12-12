use std::collections::HashSet;

fn main() {
    let input: Vec<String> = std::io::stdin().lines().map(|v| v.unwrap()).collect();
    let regions = parse(&input);

    part1(&regions);
    part2(&regions);
}

fn part1(regions: &[Region]) {
    let mut sum = 0;
    for region in regions {
        sum += region.area() * region.fence();
    }

    println!("Part 1: {sum}");
}

fn part2(regions: &[Region]) {
    let mut sum = 0;

    for region in regions {
        let sides = region.sides();
        let area = region.area();

        sum += area * sides;
    }

    println!("Part 2: {sum}");
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

            let region = Region::new(&input, (x, y));

            for pos in region.set_positions() {
                visited_positions.push(pos);
            }

            regions.push(region);
        }
    }

    regions
}

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    rows: Vec<Vec<bool>>,
}

impl Region {
    pub fn new(input: &Vec<Vec<char>>, start: (usize, usize)) -> Self {
        let width = input[0].len();
        let height = input.len();

        let rows: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        let plant = input[start.1][start.0];
        let mut me = Self { rows, plant };
        me.build_rec(start, input);

        me
    }

    fn build_rec(&mut self, pos: (usize, usize), input: &Vec<Vec<char>>) {
        if input[pos.1][pos.0] == self.plant && !self.get(pos) {
            self.set(pos, true);

            for adjacent in self.adjacent_positions(pos).flatten() {
                self.build_rec(adjacent, input);
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

    fn pad_one(&mut self) {
        self.rows
            .insert(0, (0..self.x_len()).map(|_| false).collect());
        self.rows.push((0..self.x_len()).map(|_| false).collect());
        for row in self.rows.iter_mut() {
            row.insert(0, false);
            row.push(false);
        }
    }

    pub fn sides(&self) -> usize {
        let mut fenced_spots = HashSet::new();
        let mut sides = 0;

        let mut clone = self.clone();
        clone.pad_one();

        for (px, py) in clone.set_positions() {
            for (fx, fy) in clone.adjacent_positions((px, py)).flatten() {
                if clone.get((fx, fy)) {
                    continue;
                }

                let mut insert = |fence| fenced_spots.insert(fence);

                if insert(((px, py), (fx, fy))) {
                    sides += 1;

                    let adjacent_plots =
                        |pos: (usize, usize)| clone.adjacent_positions(pos).flatten();

                    for left_x in (0..fx).rev() {
                        let new_fence = (left_x, fy);
                        let has_adjacent_plot =
                            adjacent_plots(new_fence).any(|(x, y)| y == py && clone.get((x, y)));

                        if clone.get(new_fence) || !has_adjacent_plot {
                            break;
                        }

                        insert(((left_x, py), new_fence));
                    }

                    for right_x in (fx + 1)..clone.x_len() {
                        let new_fence = (right_x, fy);
                        let has_adjacent_plot =
                            adjacent_plots(new_fence).any(|(x, y)| y == py && clone.get((x, y)));

                        if clone.get(new_fence) || !has_adjacent_plot {
                            break;
                        }

                        insert(((right_x, py), new_fence));
                    }

                    for up_y in (0..fy).rev() {
                        let new_fence = (fx, up_y);
                        let has_adjacent_plot =
                            adjacent_plots(new_fence).any(|(x, y)| x == px && clone.get((x, y)));

                        if clone.get(new_fence) || !has_adjacent_plot {
                            break;
                        }

                        insert(((px, up_y), new_fence));
                    }

                    for down_y in (fy + 1)..clone.y_len() {
                        let new_fence = (fx, down_y);
                        let has_adjacent_plot =
                            adjacent_plots(new_fence).any(|(x, y)| x == px && clone.get((x, y)));

                        if clone.get(new_fence) || !has_adjacent_plot {
                            break;
                        }

                        insert(((px, down_y), new_fence));
                    }
                }
            }
        }

        sides
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
