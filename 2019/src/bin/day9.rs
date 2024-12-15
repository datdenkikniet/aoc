use aoc2019::ProgramState;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
    part2(&program);
}

fn part1(program: &[isize]) {
    let mut program = ProgramState::new(1, program);
    program.run_to_exit();

    println!("Part 1: {}", program.output());
}

fn part2(program: &[isize]) {
    let mut program = ProgramState::new(2, program);
    program.run_to_exit();

    println!("Part 2: {}", program.output());
}
