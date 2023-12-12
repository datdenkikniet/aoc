use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Left,
    Right,
}

impl TryFrom<char> for Step {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' | 'r' => Ok(Self::Right),
            'L' | 'l' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decision {
    left: String,
    right: String,
}

fn part1<'a>(
    start_node: &str,
    steps: impl Iterator<Item = &'a Step> + Clone,
    nodes: &HashMap<String, Decision>,
    any_z: bool,
) -> usize {
    let mut step_iter = steps.cycle();
    let mut current_node = &String::from(start_node);
    let mut step_count = 0;

    loop {
        if current_node == "ZZZ" || any_z && current_node.ends_with('Z') {
            break;
        }

        step_count += 1;
        let step = step_iter.next().unwrap();

        let decision = nodes.get(current_node).unwrap();

        current_node = match step {
            Step::Left => &decision.left,
            Step::Right => &decision.right,
        };
    }

    step_count
}

fn part2<'a>(
    steps: impl Iterator<Item = &'a Step> + Clone,
    decisions: &HashMap<String, Decision>,
) -> usize {
    let nodes: Vec<_> = decisions.keys().collect();

    let nodes_with = |char| {
        nodes
            .iter()
            .filter(move |v| v.ends_with(char))
            .map(|v| nodes.iter().position(|n| n == v).unwrap())
    };

    let starting_nodes: Vec<_> = nodes_with('A').collect();

    let nodes_and_steps = starting_nodes.iter().map(|node| {
        let name = nodes[*node];
        part1(name, steps.clone(), decisions, true) as u64
    });

    println!("{}", lcm(nodes_and_steps).unwrap());

    0
}

fn lcm(values: impl Iterator<Item = u64> + Clone) -> Option<u64> {
    values.reduce(|a, b| {
        if a == 0 || b == 0 {
            0
        } else {
            a / (gcd(a, b)) * b
        }
    })
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let store_b = b;
        b = a % b;
        a = store_b;
    }

    a
}

fn main() -> std::io::Result<()> {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());

    let steps: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|v| Step::try_from(v).unwrap())
        .collect();

    let mut decisions = HashMap::new();
    let mut first_node = None;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (name, r_l) = line.split_once(" = ").unwrap();
        let (l, r) = r_l.split_once(", ").unwrap();
        let left = &l[1..];
        let right = &r[..r.len() - 1];

        decisions.insert(
            name.to_string(),
            Decision {
                left: left.to_string(),
                right: right.to_string(),
            },
        );

        if first_node.is_none() {
            first_node = Some(name.to_string());
        }
    }

    let part1 = part1("AAA", steps.iter(), &decisions, false);
    let part2 = part2(steps.iter(), &decisions);

    println!("Part 1 took {part1} steps.");
    println!("Part 2 took {part2} steps");

    Ok(())
}
