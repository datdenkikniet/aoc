use std::collections::{HashMap, VecDeque};

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let broadcast_dests: Vec<_> = lines[0]
        .split_once(" -> ")
        .unwrap()
        .1
        .split(", ")
        .map(|v| v.to_string())
        .collect();

    let mut modules = Vec::new();

    for line in lines.iter().skip(1) {
        let module = LogicalModule::parse(&line).unwrap();
        modules.push(module);
    }

    part1(broadcast_dests.clone(), &modules);
}

fn part1(broadcast_dests: Vec<String>, modules: &[LogicalModule]) {
    let mut broadcaster = Broadcaster::build(broadcast_dests, &modules);

    for _ in 0..1000 {
        broadcaster.button();
    }

    let low_pulses = broadcaster.counter_state.low;
    let high_pulses = broadcaster.counter_state.high;

    println!("HI: {high_pulses}, LO: {low_pulses}");
    println!("Part 1: {}", high_pulses * low_pulses);
}

#[derive(Debug, Clone)]
pub struct Broadcaster {
    modules: HashMap<String, Module>,
    destinations: Vec<String>,
    counter_state: CounterState,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CounterState {
    low: usize,
    high: usize,
}

impl Broadcaster {
    pub fn build(broadcast_dests: Vec<String>, modules: &[LogicalModule]) -> Self {
        let mut module_map = HashMap::new();

        for module in modules {
            let concrete_module = Module::new(module.ty);
            module_map.insert(module.name.to_string(), concrete_module);
        }

        for module in modules {
            let destinations = module.destinations.iter().map(|dest| dest.to_string());

            module_map
                .entry(module.name.to_string())
                .and_modify(|entry| {
                    entry.destinations = destinations.collect();
                });

            for destination in module.destinations.iter() {
                module_map
                    .entry(destination.to_string())
                    .and_modify(|entry| match &mut entry.state {
                        ModuleState::Conjunction(hash_map) => {
                            hash_map.insert(module.name.to_string(), State::Low);
                        }
                        _ => {}
                    })
                    .or_insert_with(|| Module {
                        state: ModuleState::End,
                        destinations: Vec::new(),
                    });
            }
        }

        let destinations = broadcast_dests.iter().map(|s| s.to_string()).collect();

        Self {
            destinations,
            modules: module_map,
            counter_state: Default::default(),
        }
    }

    pub fn button(&mut self) {
        // Low signal for button press
        self.counter_state.low += 1;

        println!("button -low-> broadcaster");

        self.counter_state.low += self.destinations.len();

        let mut pulses = VecDeque::new();

        for input in self.destinations.iter_mut() {
            println!("broadcaster -low-> {input}");

            let input_module = self.modules.get_mut(input).unwrap();
            if input_module.pulse(&"".to_string(), State::Low) {
                pulses.push_back(input.to_string());
            }
        }

        while let Some(pulsed) = pulses.pop_front() {
            let module = self.modules.get(&pulsed).unwrap();
            let output = module.output();

            let destinations = module.destinations.clone();

            if output.is_low() {
                self.counter_state.low += destinations.len();
            } else {
                self.counter_state.high += destinations.len();
            }

            let text = if output.is_low() { "-low" } else { "-high" };
            for destination in destinations {
                println!("{pulsed} {text}-> {destination}");

                let input_module = self.modules.get_mut(&destination).unwrap();
                if input_module.pulse(&pulsed, output) {
                    pulses.push_back(destination);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    state: ModuleState,
    destinations: Vec<String>,
}

impl Module {
    pub fn new(ty: ModuleType) -> Self {
        let state = match ty {
            ModuleType::FlipFlop => ModuleState::FlipFlop(State::Low),
            ModuleType::Conjunction => ModuleState::Conjunction(HashMap::new()),
        };

        Self {
            state,
            destinations: Vec::new(),
        }
    }

    pub fn pulse(&mut self, from: &String, input_state: State) -> bool {
        let state = &mut self.state;

        match state {
            ModuleState::FlipFlop(state) => {
                if input_state.is_low() {
                    state.flip();
                    true
                } else {
                    false
                }
            }
            ModuleState::Conjunction(map) => {
                *map.get_mut(from).unwrap() = input_state;
                true
            }
            ModuleState::End => false,
        }
    }

    pub fn output(&self) -> State {
        match &self.state {
            ModuleState::FlipFlop(state) => *state,
            ModuleState::Conjunction(vec) => {
                if vec.values().all(State::is_high) {
                    State::Low
                } else {
                    State::High
                }
            }
            ModuleState::End => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Low,
    High,
}

impl State {
    pub fn is_low(&self) -> bool {
        matches!(self, Self::Low)
    }

    pub fn is_high(&self) -> bool {
        matches!(self, Self::High)
    }

    pub fn flip(&mut self) {
        if self.is_low() {
            *self = Self::High;
        } else {
            *self = Self::Low;
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleState {
    FlipFlop(State),
    Conjunction(HashMap<String, State>),
    End,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleType {
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
pub struct LogicalModule<'a> {
    name: &'a str,
    ty: ModuleType,
    destinations: Vec<&'a str>,
}

impl<'a> LogicalModule<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        let (name, destinations) = input.split_once(" -> ")?;

        let ty = match name.chars().next()? {
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjunction,
            _ => return None,
        };

        let name = &name[1..];

        let destinations = destinations.split(", ").collect();

        Some(Self {
            name,
            ty,
            destinations,
        })
    }
}
