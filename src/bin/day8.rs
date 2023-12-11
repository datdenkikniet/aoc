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

fn main() -> std::io::Result<()> {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());

    let steps: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|v| Step::try_from(v).unwrap())
        .collect();

    let mut nodes = HashMap::new();
    let mut first_node = None;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (name, r_l) = line.split_once(" = ").unwrap();
        let (l, r) = r_l.split_once(", ").unwrap();
        let left = &l[1..];
        let right = &r[..r.len() - 1];

        nodes.insert(
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

    let mut step_iter = steps.iter();
    let mut current_node = &String::from("AAA");
    let mut step_count = 0;

    loop {
        if current_node == "ZZZ" {
            break;
        }

        step_count += 1;
        let step = if let Some(step) = step_iter.next() {
            step
        } else {
            step_iter = steps.iter();
            step_iter.next().unwrap()
        };
        let step = step;

        let decision = nodes.get(current_node).unwrap();

        current_node = match step {
            Step::Left => &decision.left,
            Step::Right => &decision.right,
        };
    }

    println!("Took {step_count} steps.");

    Ok(())
}
