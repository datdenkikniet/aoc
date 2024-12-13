#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

const A_COST: usize = 3;
const B_COST: usize = 1;

impl Machine {
    pub fn solve(&self) -> Option<usize> {
        let Self { a, b, prize } = self;

        let prize_gcd = Self::gcd(prize.x, prize.y);

        let (a_x, b_x) = ((a.x * prize.y) / prize_gcd, (b.x * prize.y) / prize_gcd);
        let (a_y, b_y) = ((a.y * prize.x) / prize_gcd, (b.y * prize.x) / prize_gcd);

        let total_a = a_x.max(a_y) - a_x.min(a_y);
        let total_b = b_x.max(b_y) - b_x.min(b_y);

        let gcd = Self::gcd(total_a, total_b);

        let a_mul = (total_a / gcd).max(1);
        let b_mul = (total_b / gcd).max(1);

        let mut result = None;

        for i in 0.. {
            let a_x = a.x * i * b_mul;
            let a_y = a.y * i * b_mul;

            let b_x = b.x * i * a_mul;
            let b_y = b.y * i * a_mul;

            let x = a_x + b_x;
            let y = a_y + b_y;

            if x == prize.x && y == prize.y {
                result = Some(i);
                break;
            } else if x > prize.x || y > prize.y {
                break;
            }
        }

        result.map(|iter| (iter * b_mul * A_COST) + (iter * a_mul * B_COST))
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else if a < b {
            Self::gcd(b, a)
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut machines = Vec::new();
    for machine in lines.chunks(4) {
        let a = parse_line(&machine[0]);
        let b = parse_line(&machine[1]);
        let prize = parse_line(&machine[2]);

        machines.push(Machine { a, b, prize });
    }

    let result: usize = machines.iter().flat_map(Machine::solve).sum();

    println!("Part 1: {result}");
}

fn parse_line(input: &str) -> Point {
    let (_, xy) = input.split_once(": ").unwrap();
    let (x, y) = xy.split_once(", ").unwrap();

    let x = x[2..].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Point { x, y }
}
