use aoc2019::ProgramState;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
    part2(&program);
}

fn part1(program: &[isize]) {
    let mut program = program.to_vec();

    program[1] = 12;
    program[2] = 2;

    let mut program = ProgramState::new(0, &mut program);
    program.run_to_exit();
    let result = program.program()[0];

    println!("Part 1: {}", result);
}

fn part2(program: &[isize]) {
    let mut result = (0, 0);
    for noun in 0..99 {
        for verb in 0..99 {
            let mut run_program = program.to_vec();
            run_program[1] = noun;
            run_program[2] = verb;

            let mut program = ProgramState::new(0, &mut run_program);
            program.run_to_exit();

            if program.program()[0] == 19690720 {
                result = (noun, verb);
                break;
            }
        }
    }

    println!("Part 2: {}", 100 * result.0 + result.1);
}
