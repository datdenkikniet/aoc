fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let rotations: Vec<_> = lines
        .map(|v| {
            let count: usize = v[1..].parse().unwrap();

            if v.chars().next() == Some('L') {
                Rotation::Left(count)
            } else {
                Rotation::Right(count)
            }
        })
        .collect();

    part1(&rotations);
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

fn part1(rotation: &[Rotation]) {
    let mut zeros = 0;
    let mut current_position = 50;

    for rot in rotation.iter().cloned() {
        match rot {
            Rotation::Left(c) => {
                let c = c % 100;
                if c > current_position {
                    let diff = c - current_position;
                    current_position = 100 - diff;
                } else {
                    current_position -= c;
                }
            }
            Rotation::Right(c) => {
                let c = c % 100;
                current_position = (current_position + c) % 100;
            }
        }

        if current_position == 0 {
            zeros += 1;
        }
    }

    println!("Part 1: {zeros}");
}
