use std::ops::RangeInclusive;

fn main() {
    let mut lines = std::io::stdin().lines().map(|v| v.unwrap());

    let mut fresh_ranges = Vec::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();

        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());

        fresh_ranges.push(start..=end);
    }

    let ingredients: Vec<usize> = lines.map(|v| v.parse().unwrap()).collect();

    part1(&fresh_ranges, &ingredients);
    part2(&fresh_ranges);
}

fn part1(fresh_ranges: &[RangeInclusive<usize>], ingredients: &[usize]) {
    let mut fresh = 0;
    for ingredient in ingredients.iter().copied() {
        for range in fresh_ranges {
            if range.contains(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }

    println!("Part 1: {fresh}");
}

fn part2(fresh_ranges: &[RangeInclusive<usize>]) {
    let mut ranges: Vec<_> = fresh_ranges.iter().cloned().collect();
    ranges.sort_unstable_by_key(|v| *v.start());

    let mut new_ranges = vec![ranges[0].clone(); 1];

    for range in ranges.into_iter().skip(1) {
        let prev = new_ranges.last().unwrap();

        if prev.contains(range.start()) && prev.contains(range.end()) {
            continue;
        }

        let range = if range.start() > prev.end() {
            range.clone()
        } else {
            prev.end() + 1..=*range.end()
        };

        new_ranges.push(range);
    }

    let sum: usize = new_ranges.into_iter().map(|v| v.count()).sum();

    println!("Part 2: {}", sum);
}
