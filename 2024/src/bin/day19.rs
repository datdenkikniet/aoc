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
