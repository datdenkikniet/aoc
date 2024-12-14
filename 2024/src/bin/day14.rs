#[derive(Debug, Clone, PartialEq)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    room_dims: (isize, isize),
}

impl Robot {
    pub fn make_move(&mut self, count: isize) {
        let (x, y) = self.position;
        let (x_len, y_len) = self.room_dims;
        let (mut x_vel, mut y_vel) = self.velocity;

        if x_vel < 0 {
            x_vel = self.room_dims.0 + x_vel;
        }

        if y_vel < 0 {
            y_vel = self.room_dims.1 + y_vel;
        }

        let new_x = x.wrapping_add(x_vel * count) % x_len;
        let new_y = y.wrapping_add(y_vel * count) % y_len;

        self.position = (new_x, new_y);
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let x_len = 101;
    let y_len = 103;

    let mut robots = Vec::new();
    for line in lines {
        let (pos, vel) = line.split_once(' ').unwrap();

        let (x_pos, y_pos) = pos[2..].split_once(',').unwrap();
        let (x_vel, y_vel) = vel[2..].split_once(',').unwrap();

        let (x_pos, y_pos): (isize, isize) = (x_pos.parse().unwrap(), y_pos.parse().unwrap());
        let (x_vel, y_vel): (isize, isize) = (x_vel.parse().unwrap(), y_vel.parse().unwrap());
        robots.push(Robot {
            position: (x_pos, y_pos),
            velocity: (x_vel, y_vel),
            room_dims: (x_len, y_len),
        });
    }

    part1(&robots);
}

fn part1(robots: &[Robot]) {
    let (x_len, y_len) = robots[0].room_dims;
    let mut robots = robots.to_vec();

    robots.iter_mut().for_each(|r| r.make_move(100));

    let quad_mid_x = if x_len % 2 == 0 {
        x_len / 2
    } else {
        (x_len / 2) + 1
    };

    let quad_min_y = if y_len % 2 == 0 {
        y_len / 2
    } else {
        (y_len / 2) + 1
    };

    let mut quadrant_sums = [0, 0, 0, 0];
    let quadrants = [
        (0..x_len / 2, 0..y_len / 2),
        (quad_mid_x..x_len, 0..y_len / 2),
        (quad_mid_x..x_len, quad_min_y..y_len),
        (0..x_len / 2, quad_min_y..y_len),
    ];

    for (idx, (x_range, y_range)) in quadrants.into_iter().enumerate() {
        for x in x_range {
            for y in y_range.clone() {
                for robot in &robots {
                    if robot.position.0 == x && robot.position.1 == y {
                        quadrant_sums[idx] += 1;
                    }
                }
            }
        }
    }

    let prod: usize = quadrant_sums.into_iter().product();

    println!("Part 1: {prod}");
}
