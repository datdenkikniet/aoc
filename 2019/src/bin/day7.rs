use std::isize;

use aoc2019::ProgramState;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
}

fn part1(program: &[isize]) {
    let mut max_thruster_value = isize::MIN;

    for settings in settings() {
        let mut input = 0;

        for setting in settings {
            let mut program = ProgramState::new_multi_input(vec![setting, input], program.to_vec());
            program.run_to_exit();
            input = program.output();
        }

        max_thruster_value = max_thruster_value.max(input);
    }
    println!("Thruster input: {max_thruster_value}");
}

fn settings() -> impl Iterator<Item = [isize; 5]> {
    fn rec(path: &mut Vec<isize>, out: &mut Vec<[isize; 5]>) {
        for v in 0..5 {
            if path.contains(&v) {
                continue;
            }

            path.push(v);

            if path.len() == 5 {
                let mut value = [0isize; 5];

                path.iter()
                    .cloned()
                    .enumerate()
                    .for_each(|(idx, v)| value[idx] = v);

                out.push(value);
            }

            rec(path, out);

            path.pop();
        }
    }

    let mut out = Vec::new();
    let mut path = Vec::new();
    rec(&mut path, &mut out);

    out.into_iter()
}
