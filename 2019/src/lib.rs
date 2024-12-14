#[derive(Debug, PartialEq)]
pub struct ProgramState<'a> {
    program: &'a mut [isize],
    input: isize,
    output: isize,
}

impl<'a> ProgramState<'a> {
    pub fn new(input: isize, program: &'a mut [isize]) -> Self {
        Self {
            input,
            output: 0,
            program,
        }
    }

    pub fn run_to_exit(&mut self) {
        let mut ip = 0;
        let program = &mut self.program;

        let get = |program: &[isize], value: isize, param_mode| match param_mode {
            ParamMode::Position => program[value as usize],
            ParamMode::Immediate => value,
        };

        loop {
            let op_in = program[ip];

            let op = op_in % 100;
            let mut modes = param_modes(op_in as usize);
            let mut prm = || modes.next().unwrap();

            let size = match op {
                1 => {
                    let (v1, v1_prm) = (program[ip + 1], prm());
                    let (v2, v2_prm) = (program[ip + 2], prm());
                    let (dest_idx, dst_prm) = (program[ip + 3] as usize, prm());

                    let v1 = get(program, v1, v1_prm);
                    let v2 = get(program, v2, v2_prm);

                    assert_eq!(dst_prm, ParamMode::Position);

                    program[dest_idx] = v1 + v2;
                    4
                }
                2 => {
                    let (v1, v1_prm) = (program[ip + 1], prm());
                    let (v2, v2_prm) = (program[ip + 2], prm());
                    let (dest_idx, dst_prm) = (program[ip + 3] as usize, prm());

                    let v1 = get(program, v1, v1_prm);
                    let v2 = get(program, v2, v2_prm);

                    assert_eq!(dst_prm, ParamMode::Position);

                    program[dest_idx] = v1 * v2;
                    4
                }
                3 => {
                    let destination = program[ip + 1] as usize;

                    assert_eq!(prm(), ParamMode::Position);

                    program[destination] = self.input;
                    2
                }
                4 => {
                    let source = program[ip + 1];
                    self.output = get(program, source, prm());
                    2
                }
                5 | 6 => {
                    let should_be_zero = op == 6;

                    let (val, val_prm) = (program[ip + 1], prm());
                    let (dst, dst_prm) = (program[ip + 2], prm());

                    let is_zero = get(&program, val, val_prm) == 0;
                    let branch = !(should_be_zero ^ is_zero);

                    if branch {
                        ip = get(&program, dst, dst_prm) as usize;
                        0
                    } else {
                        3
                    }
                }
                7 | 8 => {
                    let check: fn(&isize, &isize) -> bool = if op == 7 {
                        isize::lt as _
                    } else {
                        isize::eq as _
                    };

                    let (v1, v1_prm) = (program[ip + 1], prm());
                    let (v2, v2_prm) = (program[ip + 2], prm());
                    let (dst, dst_prm) = (program[ip + 3], prm());

                    assert_eq!(dst_prm, ParamMode::Position);

                    let v1 = get(program, v1, v1_prm);
                    let v2 = get(program, v2, v2_prm);

                    program[dst as usize] = check(&v1, &v2) as isize;

                    4
                }
                99 => {
                    break;
                }
                _ => todo!(),
            };

            ip += size;

            if program.get(ip) != Some(&99) {
                assert_eq!(self.output, 0);
            }
        }
    }

    pub fn program(&self) -> &[isize] {
        self.program
    }

    pub fn output(&self) -> isize {
        self.output
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

fn param_modes(op: usize) -> impl Iterator<Item = ParamMode> {
    struct Iter {
        value: usize,
    }

    impl Iterator for Iter {
        type Item = ParamMode;

        fn next(&mut self) -> Option<Self::Item> {
            let mode = match self.value % 10 {
                1 => ParamMode::Immediate,
                0 => ParamMode::Position,
                _ => panic!(),
            };

            self.value /= 10;

            Some(mode)
        }
    }

    Iter { value: op / 100 }
}