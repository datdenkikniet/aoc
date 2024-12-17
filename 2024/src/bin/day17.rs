#[derive(Debug, Clone, Copy)]
pub struct Regs {
    a: isize,
    b: isize,
    c: isize,
}

impl Regs {
    fn combo(&self, value: isize) -> isize {
        match value {
            0..4 => value,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let a: isize = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let b: isize = lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let c: isize = lines[2].split_once(": ").unwrap().1.parse().unwrap();

    let program: Vec<isize> = lines[4]
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let regs = Regs { a, b, c };

    part1(regs, &program);
}

fn part1(mut regs: Regs, program: &[isize]) {
    let mut output: Vec<isize> = Vec::new();

    let mut ip = 0;
    loop {
        if ip >= program.len() {
            break;
        }

        let op = program[ip];

        let len = match op {
            0 => {
                let num = regs.a;
                let den = 2isize.pow(regs.combo(program[ip + 1]).try_into().unwrap());
                regs.a = num / den;
                2
            }
            1 => {
                let lhs = regs.b;
                let rhs = program[ip + 1];
                regs.b = lhs ^ rhs;
                2
            }
            2 => {
                let value = regs.combo(program[ip + 1]);
                regs.b = value % 8;
                2
            }
            3 => {
                if regs.a != 0 {
                    ip = program[ip + 1].try_into().unwrap();
                    continue;
                }
                2
            }
            4 => {
                let lhs = regs.b;
                let rhs = regs.c;
                regs.b = lhs ^ rhs;
                2
            }
            5 => {
                let value = regs.combo(program[ip + 1]);
                output.push(value % 8);
                2
            }
            6 => {
                let num = regs.a;
                let den = 2isize.pow(regs.combo(program[ip + 1]).try_into().unwrap());
                regs.b = num / den;
                2
            }
            7 => {
                let num = regs.a;
                let den = 2isize.pow(regs.combo(program[ip + 1]).try_into().unwrap());
                regs.c = num / den;
                2
            }
            _ => panic!(),
        };

        ip += len;
    }

    let output: Vec<_> = output.iter().map(|v| v.to_string()).collect();
    let output = output.join(",");

    println!("Part 1: {output}");
}
