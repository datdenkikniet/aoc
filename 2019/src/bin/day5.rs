use aoc2019::ProgramState;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
    part2(&program);
}

fn part1(program: &[isize]) {
    let mut program = program.to_vec();
    let mut program = ProgramState::new(1, &mut program);
    program.run_to_exit();
    let result = program.output();
    println!("Part 1: {}", result);
}

fn part2(program: &[isize]) {
    let mut program = program.to_vec();
    let mut program = ProgramState::new(5, &mut program);
    program.run_to_exit();
    let result = program.output();
    println!("Part 1: {}", result);
}
