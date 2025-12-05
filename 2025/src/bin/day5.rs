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
