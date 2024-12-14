use std::{isize, ops::Range};

use aoc2019::ProgramState;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let program: Vec<isize> = line.split(',').map(|v| v.parse().unwrap()).collect();

    part1(&program);
    part2(&program);
}

fn part1(program: &[isize]) {
    let mut max_thruster_value = isize::MIN;

    for settings in settings(0..5) {
        let mut input = 0;

        for setting in settings {
            let mut program = ProgramState::new_multi_input(vec![setting, input], program.to_vec());
            program.run_to_exit();
            input = program.output();
        }

        max_thruster_value = max_thruster_value.max(input);
    }
    println!("Part 1: {max_thruster_value}");
}

fn part2(program: &[isize]) {
    let mut max_thruster_value = isize::MIN;

    for setting in settings(5..10) {
        let mut amplifiers = [
            ProgramState::new_multi_input(vec![setting[0]], program.to_vec()),
            ProgramState::new_multi_input(vec![setting[1]], program.to_vec()),
            ProgramState::new_multi_input(vec![setting[2]], program.to_vec()),
            ProgramState::new_multi_input(vec![setting[3]], program.to_vec()),
            ProgramState::new_multi_input(vec![setting[4]], program.to_vec()),
        ];

        amplifiers.iter_mut().for_each(|a| {
            assert!(a.poll().is_pending());
        });

        let mut input = 0;

        loop {
            let mut any_done = false;
            for amplifier in amplifiers.iter_mut() {
                assert_eq!(amplifier.input_len(), 0);

                amplifier.add_input(input);

                if amplifier.poll().is_ready() {
                    any_done = true;
                }

                input = amplifier.output();
            }

            max_thruster_value = max_thruster_value.max(input);

            if any_done {
                break;
            }
        }
    }

    println!("Part 2: {max_thruster_value}");
}

fn settings(range: Range<isize>) -> impl Iterator<Item = [isize; 5]> {
    fn rec(range: Range<isize>, path: &mut Vec<isize>, out: &mut Vec<[isize; 5]>) {
        for v in range.clone() {
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

            rec(range.clone(), path, out);

            path.pop();
        }
    }

    let mut out = Vec::new();
    let mut path = Vec::new();
    rec(range, &mut path, &mut out);

    out.into_iter()
}
