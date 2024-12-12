fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<usize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
    part2(&program);
}

fn part1(program: &[usize]) {
    let mut program = program.to_vec();

    program[1] = 12;
    program[2] = 2;

    let result = run(&mut program);

    println!("Part 1: {}", result);
}

fn part2(program: &[usize]) {
    let mut result = (0, 0);
    for noun in 0..99 {
        for verb in 0..99 {
            let mut run_program = program.to_vec();
            run_program[1] = noun;
            run_program[2] = verb;

            if run(&mut run_program) == 19690720 {
                result = (noun, verb);
                break;
            }
        }
    }

    println!("Part 2: {}", 100 * result.0 + result.1);
}

fn run(program: &mut [usize]) -> usize {
    let mut ip = 0;

    loop {
        let data = &program[ip..(ip + 4).min(program.len())];

        let op = data[0];

        if op == 99 {
            break program[0];
        }

        let v1_idx = data[1];
        let v2_idx = data[2];
        let dest_idx = data[3];

        let v1 = program[v1_idx];
        let v2 = program[v2_idx];

        let result = match op {
            1 => v1 + v2,
            2 => v1 * v2,
            _ => todo!(),
        };

        program[dest_idx] = result;

        ip += 4;
    }
}
