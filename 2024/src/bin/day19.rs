use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());

    let available_towels: Vec<_> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_string)
        .collect();

    let desired_patterns: Vec<_> = lines.skip(1).collect();

    part1(&available_towels, &desired_patterns);
    part2(&available_towels, &desired_patterns);
}

fn part1(towels: &[String], desired_patterns: &[String]) {
    let mut sum = 0;

    for pattern in desired_patterns {
        if is_possible(towels, pattern) {
            sum += 1;
        }
    }

    println!("Part 1: {sum}");
}

fn is_possible(towels: &[String], pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for towel in towels {
        if pattern.starts_with(towel) && is_possible(towels, &pattern[towel.len()..]) {
            return true;
        }
    }

    false
}

fn part2(towels: &[String], desired_patterns: &[String]) {
    let mut sum = 0;

    let mut known = HashMap::new();
    for pattern in desired_patterns {
        sum += possible_patterns(towels, pattern, &mut known);
    }

    println!("Part 2: {sum}");
}

fn possible_patterns(
    towels: &[String],
    pattern: &str,
    known: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(known) = known.get(pattern).cloned() {
        return known;
    }

    let mut patterns = 0;

    for towel in towels {
        if pattern.starts_with(towel) {
            patterns += possible_patterns(towels, &pattern[towel.len()..], known);
        }
    }

    known.insert(pattern.to_string(), patterns);

    patterns
}
