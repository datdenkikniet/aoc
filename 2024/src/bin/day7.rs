#[derive(Clone)]
struct Equation {
    answer: usize,
    parts: Vec<usize>,
    operators: Vec<&'static str>,
}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.answer, self.parts[0])?;

        for (part, op) in self.parts.iter().skip(1).zip(self.operators.iter()) {
            write!(f, " {op} {part}")?;
        }

        Ok(())
    }
}

const PLUS: &'static str = "+";
const STAR: &'static str = "*";
const CONCAT: &'static str = "||";

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let mut equations = Vec::new();

    for line in lines {
        let (answer, parts) = line.split_once(": ").unwrap();
        let answer: usize = answer.parse().unwrap();

        let parts = parts.split(' ').map(|p| p.parse().unwrap()).collect();

        equations.push(Equation {
            answer,
            parts,
            operators: Vec::new(),
        })
    }

    part1(equations.clone());
    part2(equations.clone());
}

type MapFn = fn(usize, usize) -> usize;

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn multiply(a: usize, b: usize) -> usize {
    a * b
}

fn concat(a: usize, b: usize) -> usize {
    let log = b.ilog10() + 1;

    let multiplication_factor = 10usize.pow(log);
    let current_shifted = a * multiplication_factor;
    current_shifted + b
}

fn part1(mut equations: Vec<Equation>) {
    let mut sum = 0;

    let operators = [(PLUS, add as MapFn), (STAR, multiply as MapFn)];

    for equation in equations.iter_mut() {
        if is_possible(equation.answer, equation.parts[0], 1, &operators, equation) {
            sum += equation.answer;
            println!("{equation}");
        }
    }

    println!("Part 1: {sum}");
}

fn part2(mut equations: Vec<Equation>) {
    let mut sum = 0;

    let operators = [
        (PLUS, add as MapFn),
        (STAR, multiply as MapFn),
        (CONCAT, concat as MapFn),
    ];

    for equation in equations.iter_mut() {
        if is_possible(equation.answer, equation.parts[0], 1, &operators, equation) {
            sum += equation.answer;
            println!("{equation}");
        }
    }

    println!("Part 2: {sum}");
}

fn is_possible(
    goal: usize,
    current: usize,
    idx: usize,
    operators: &[(&'static str, fn(usize, usize) -> usize)],
    equation: &mut Equation,
) -> bool {
    let current_part = if let Some(current_part) = equation.parts.get(idx) {
        *current_part
    } else {
        return goal == current;
    };

    for (name, operator) in operators {
        let next = operator(current, current_part);
        if is_possible(goal, next, idx + 1, operators, equation) {
            equation.operators.push(name);
            return true;
        }
    }

    false
}
