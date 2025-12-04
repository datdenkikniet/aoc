fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let mut grid = Vec::new();

    for line in lines {
        let mut row = Vec::with_capacity(line.len());

        for c in line.chars() {
            if c == '@' {
                row.push(true);
            } else {
                row.push(false)
            }
        }

        grid.push(row)
    }

    part1(&grid);
}

fn part1(grid: &[Vec<bool>]) {
    let width = grid[0].len();
    let mut total = 0;

    for x in 0..width {
        for y in 0..grid.len() {
            if is_accessible_roll(grid, x, y) {
                total += 1;
            }
        }
    }

    println!("Part 1: {total}");
}

fn is_accessible_roll(grid: &[Vec<bool>], x: usize, y: usize) -> bool {
    let mut total_occupied = 0;

    if !grid[y][x] {
        return false;
    }

    for x_off in [-1, 0, 1] {
        for y_off in [-1, 0, 1] {
            if x_off == 0 && y_off == 0 {
                continue;
            }

            let x = (x as isize + x_off) as usize;
            let y = (y as isize + y_off) as usize;

            let occupied = grid
                .get(y)
                .map(|v| v.get(x))
                .flatten()
                .copied()
                .unwrap_or(false);

            if occupied {
                total_occupied += 1;
            }
        }
    }

    total_occupied < 4
}
