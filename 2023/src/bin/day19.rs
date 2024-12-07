use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone)]
pub struct Workflow {
    rules: Vec<Rule>,
    otherwise: WorkflowName,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> &WorkflowName {
        for rule in &self.rules {
            if rule.matches(part) {
                return &rule.destination;
            }
        }

        &self.otherwise
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    operand: RatingCategory,
    operator: Operator,
    num: usize,
    destination: WorkflowName,
}

impl Rule {
    pub fn matches(&self, part: &Part) -> bool {
        let operand = match self.operand {
            RatingCategory::X => part.x,
            RatingCategory::M => part.m,
            RatingCategory::A => part.a,
            RatingCategory::S => part.s,
        };

        let operator = match self.operator {
            Operator::Gt => usize::gt,
            Operator::Lt => usize::lt,
        };

        operator(&operand, &self.num)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Gt,
    Lt,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RatingCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkflowName(String);

impl std::fmt::Display for WorkflowName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WorkflowName {
    pub fn is_end(&self) -> bool {
        self.is_accepted() || self.is_rejected()
    }

    pub fn is_accepted(&self) -> bool {
        self.0 == "A"
    }

    pub fn is_rejected(&self) -> bool {
        self.0 == "R"
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<[usize; 4]> for Part {
    fn from(value: [usize; 4]) -> Self {
        Self {
            x: value[0],
            m: value[1],
            a: value[2],
            s: value[3],
        }
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let (workflows, ratings) = parse(&lines);

    part1(&workflows, &ratings);
    part2(&workflows);
}

fn part1(workflows: &HashMap<WorkflowName, Workflow>, parts: &[Part]) {
    let in_workflow = WorkflowName("in".to_string());
    let mut sum = 0;
    for part in parts {
        let mut workflow = workflows.get(&in_workflow).unwrap();
        loop {
            let next = workflow.process(&part);

            if next.is_accepted() {
                sum += part.x + part.m + part.a + part.s;
                break;
            } else if next.is_rejected() {
                break;
            }

            workflow = workflows.get(next).unwrap();
        }
    }

    println!("Part 1: {sum}");
}

fn part2<'a>(workflows: &HashMap<WorkflowName, Workflow>) {
    const TOTAL: usize = 4000 * 4000 * 4000 * 4000;

    fn calculate_acceptance(
        workflows: &HashMap<WorkflowName, Workflow>,
        current_workflow: &WorkflowName,
        available_ranges: &mut [Range<usize>; 4],
    ) -> (usize, usize) {
        use RatingCategory::*;

        let total: usize = available_ranges.iter().map(|v| v.len()).product();

        let workflow = workflows.get(&current_workflow).unwrap();

        let mut rejected = 0;
        let mut accepted = 0;

        for rule in &workflow.rules {
            let rest_available: usize = [X, M, A, S]
                .into_iter()
                .filter(|v| v != &rule.operand)
                .map(|v| available_ranges[v as usize].len())
                .product();

            let available_range = &mut available_ranges[rule.operand as usize];

            let consumed_range = match rule.operator {
                Operator::Gt => {
                    let start = rule.num.max(available_range.start);
                    let end = available_range.end;
                    available_range.end = start;
                    start..end
                }
                Operator::Lt => {
                    let start = available_range.start;
                    let end = (rule.num - 1).min(available_range.end);
                    available_range.start = end;
                    start..end
                }
            };

            let count = consumed_range.len();

            if rule.destination.is_accepted() {
                accepted += count * rest_available;
            } else if rule.destination.is_rejected() {
                rejected += count * rest_available;
            } else {
                let mut copy = available_ranges.clone();
                copy[rule.operand as usize] = consumed_range.clone();

                let (dest_accepted, dest_rejected) =
                    calculate_acceptance(workflows, &rule.destination, &mut copy);

                assert_eq!(
                    dest_accepted + dest_rejected,
                    consumed_range.len() * rest_available
                );

                accepted += dest_accepted;
                rejected += dest_rejected;
            }
        }

        let left = total - (rejected + accepted);
        if workflow.otherwise.is_accepted() {
            accepted += left;
        } else if workflow.otherwise.is_rejected() {
            rejected += left;
        } else {
            let count: usize = available_ranges.iter().map(|v| v.len()).product();
            assert_eq!(count, left, "{current_workflow}, {available_ranges:?}");

            let (dest_accepted, dest_rejected) =
                calculate_acceptance(workflows, &workflow.otherwise, available_ranges);

            accepted += dest_accepted;
            rejected += dest_rejected;
            assert_eq!(dest_accepted + dest_rejected, count);
        }

        assert_eq!(accepted + rejected, total, "{current_workflow}");

        (accepted, rejected)
    }

    let in_workflow = WorkflowName("in".to_string());
    let mut ranges = [0..4000, 0..4000, 0..4000, 0..4000];
    let (accepted, rejected) = calculate_acceptance(&workflows, &in_workflow, &mut ranges);

    assert_eq!(accepted + rejected, TOTAL);

    println!("Part 2: {accepted}");
}

fn parse(lines: &[String]) -> (HashMap<WorkflowName, Workflow>, Vec<Part>) {
    let mut lines = lines.iter();

    let mut workflows = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (name, mut input) = line.split_once('{').unwrap();
        let mut rules = Vec::new();

        let otherwise = loop {
            if !input.contains(',') {
                let otherwise = input[..input.len() - 1].to_string();
                break otherwise;
            }

            let mut chars = input.chars();

            let operand = match chars.next().unwrap() {
                'x' => RatingCategory::X,
                'm' => RatingCategory::M,
                'a' => RatingCategory::A,
                's' => RatingCategory::S,
                _ => panic!(),
            };

            let operator = match chars.next().unwrap() {
                '>' => Operator::Gt,
                '<' => Operator::Lt,
                _ => panic!(),
            };

            let mut count = String::new();

            for char in &mut chars {
                if char == ':' {
                    break;
                }
                count.push(char);
            }

            let count: usize = count.parse().unwrap();

            let (_, destination) = input.split_once(':').unwrap();
            let (destination, rest) = destination.split_once(',').unwrap();

            rules.push(Rule {
                operand,
                operator,
                num: count,
                destination: WorkflowName(destination.to_string()),
            });

            input = rest;
        };

        workflows.insert(
            WorkflowName(name.to_string()),
            Workflow {
                rules,
                otherwise: WorkflowName(otherwise),
            },
        );
    }

    let mut ratings = Vec::new();

    for rating in lines {
        let rating_parts = rating.split(',');
        let mut output = [0usize; 4];
        for (idx, part) in rating_parts.enumerate() {
            let (_, mut num) = part.split_once('=').unwrap();

            if idx == 3 {
                num = &num[..num.len() - 1];
            }

            let num: usize = num.parse().unwrap();
            output[idx] = num;
        }

        ratings.push(Part::from(output));
    }

    (workflows, ratings)
}
