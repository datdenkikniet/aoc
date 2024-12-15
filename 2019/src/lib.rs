use std::{
    collections::{HashMap, VecDeque},
    task::Poll,
};

#[derive(Debug)]
pub struct ProgramState {
    inputs: VecDeque<isize>,
    output: isize,
    ip: usize,
    memory: Memory,
}

#[derive(Debug)]
struct Memory {
    program: Vec<isize>,
    extra_memory: HashMap<usize, isize>,
    relative_base: isize,
}

impl Memory {
    fn addr(&self, mode: ParamMode, value: isize) -> Option<usize> {
        match mode {
            ParamMode::Position => Some(value as _),
            ParamMode::Immediate => None,
            ParamMode::Relative => Some((self.relative_base + value) as _),
        }
    }

    pub fn get(&self, mode: ParamMode, value: isize) -> isize {
        let address = if let Some(addr) = self.addr(mode, value) {
            addr
        } else {
            return value;
        };

        if let Some(value) = self.program.get(address) {
            *value
        } else {
            self.extra_memory.get(&address).cloned().unwrap_or(0)
        }
    }

    pub fn set(&mut self, mode: ParamMode, address: isize, value: isize) {
        let address = self
            .addr(mode, address)
            .expect("Cannot make immediate writes");

        let dest = if let Some(destination) = self.program.get_mut(address) {
            destination
        } else {
            self.extra_memory.entry(address).or_insert(0)
        };

        *dest = value;
    }
}

impl ProgramState {
    pub fn new(input: isize, program: &[isize]) -> Self {
        Self::new_multi_input(vec![input], program.to_vec())
    }

    pub fn new_empty(program: &[isize]) -> Self {
        Self::new_multi_input(Vec::new(), program.to_vec())
    }

    pub fn new_multi_input(inputs: Vec<isize>, program: Vec<isize>) -> Self {
        Self {
            inputs: inputs.into_iter().collect(),
            output: 0,
            ip: 0,
            memory: Memory {
                program: program.to_vec(),
                extra_memory: HashMap::new(),
                relative_base: 0,
            },
        }
    }

    pub fn add_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn poll(&mut self) -> std::task::Poll<()> {
        loop {
            let program = &mut self.memory;

            let ip = self.ip;

            let op_in = program.get(ParamMode::Position, ip as isize);

            let op = op_in % 100;
            let mut modes = param_modes(op_in as usize);

            macro_rules! get {
                ($ip_offset:literal) => {{
                    let prm = modes.next().unwrap();
                    let address_val = program.get(ParamMode::Position, (ip + $ip_offset) as _);
                    program.get(prm, address_val)
                }};
            }

            macro_rules! set {
                ($ip_offset:literal, $value:expr) => {{
                    let prm = modes.next().unwrap();
                    let address_val = program.get(ParamMode::Position, (ip + $ip_offset) as _);
                    program.set(prm, address_val, $value);
                }};
            }

            let size = match op {
                1 => {
                    let v1 = get!(1);
                    let v2 = get!(2);

                    set!(3, v1 + v2);
                    4
                }
                2 => {
                    let v1 = get!(1);
                    let v2 = get!(2);
                    set!(3, v1 * v2);
                    4
                }
                3 => {
                    let input = self.inputs.pop_front();

                    if let Some(input) = input {
                        set!(1, input);
                        2
                    } else {
                        break Poll::Pending;
                    }
                }
                4 => {
                    self.output = get!(1);
                    2
                }
                5 | 6 => {
                    let should_be_zero = op == 6;

                    let is_zero = get!(1) == 0;
                    let branch = !(should_be_zero ^ is_zero);

                    if branch {
                        self.ip = get!(2) as usize;
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

                    let v1 = get!(1);
                    let v2 = get!(2);

                    set!(3, check(&v1, &v2) as _);
                    4
                }
                9 => {
                    let rb_offset = get!(1);
                    program.relative_base += rb_offset;
                    2
                }
                99 => {
                    break Poll::Ready(());
                }
                _ => todo!(),
            };

            self.ip += size;
        }
    }

    pub fn run_to_exit(&mut self) {
        self.ip = 0;

        let run_once = self.poll();
        assert!(run_once.is_ready());
    }

    pub fn program(&self) -> &[isize] {
        &self.memory.program
    }

    pub fn output(&self) -> isize {
        self.output
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

fn param_modes(op: usize) -> impl Iterator<Item = ParamMode> + Clone {
    #[derive(Clone)]
    struct Iter {
        value: usize,
    }

    impl Iterator for Iter {
        type Item = ParamMode;

        fn next(&mut self) -> Option<Self::Item> {
            let mode = match self.value % 10 {
                2 => ParamMode::Relative,
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
