use std::collections::{HashMap, HashSet};

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
    let mut rec_sum = 0;

    for pattern in desired_patterns {
        if is_possible(towels, pattern) {
            sum += 1;
        }

        if is_possible_rec(towels, pattern) {
            rec_sum += 1;
        }
    }

    assert_eq!(sum, rec_sum);

    println!("Part 1: {sum}");
}

fn is_possible(towels: &[String], pattern: &str) -> bool {
    let mut patterns_to_check = HashSet::new();
    patterns_to_check.insert(pattern);

    while let Some(to_check) = patterns_to_check.iter().next().cloned() {
        if to_check.is_empty() {
            return true;
        }

        patterns_to_check.remove(to_check);

        for towel in towels {
            if to_check.starts_with(towel) {
                patterns_to_check.insert(&to_check[towel.len()..]);
            }
        }
    }

    return false;
}

fn is_possible_rec(towels: &[String], pattern: &str) -> bool {
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
    let mut rec_sum = 0;

    let mut known = HashMap::new();
    let mut known_rec = HashMap::new();
    for pattern in desired_patterns {
        sum += possible_patterns(towels, pattern, &mut known);
        rec_sum += possible_patterns_rec(towels, pattern, &mut known_rec);
    }

    assert_eq!(sum, rec_sum);

    println!("Part 2: {sum}");
}

fn possible_patterns(
    towels: &[String],
    pattern: &str,
    known: &mut HashMap<String, usize>,
) -> usize {
    let mut patterns_to_check = Vec::new();
    patterns_to_check.push(pattern);

    let mut sum = 0;
    while let Some(left) = patterns_to_check.pop() {
        let mut sum_for_me = Some(0);

        for towel in towels {
            if left.starts_with(towel) {
                let next_pattern = &left[towel.len()..];

                let count = if next_pattern.is_empty() {
                    1
                } else if let Some(known) = known.get(next_pattern).cloned() {
                    known
                } else {
                    sum_for_me.take();
                    patterns_to_check.push(next_pattern);
                    continue;
                };

                if let Some(sum_for_me) = sum_for_me.as_mut() {
                    *sum_for_me += count;
                }

                sum += count;
            }
        }

        if let Some(sum_for_me) = sum_for_me {
            known.insert(left.to_string(), sum_for_me);
        }
    }

    sum
}

fn possible_patterns_rec(
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
