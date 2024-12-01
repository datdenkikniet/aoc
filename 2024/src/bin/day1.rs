use std::collections::HashMap;

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[String]) {
    let mut lhs_list = Vec::with_capacity(lines.len());
    let mut rhs_list = Vec::with_capacity(lines.len());

    for line in lines {
        let (lhs, rhs) = line.split_once("   ").unwrap();
        let lhs = lhs.parse::<usize>().unwrap();
        let rhs = rhs.parse::<usize>().unwrap();

        push_sorted(&mut lhs_list, lhs);
        push_sorted(&mut rhs_list, rhs);
    }

    let mut sum = 0;
    for (lhs, rhs) in lhs_list.into_iter().zip(rhs_list.into_iter()) {
        let min = usize::min(lhs, rhs);
        let max = usize::max(lhs, rhs);
        let distance = max - min;
        sum += distance;
    }

    println!("Part 1: {sum}");
}

fn push_sorted(list: &mut Vec<usize>, value: usize) {
    let idx = match list.binary_search(&value) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };

    list.insert(idx, value);
}

fn part2(lines: &[String]) {
    let mut lhs_list = Vec::with_capacity(lines.len());
    let mut rhs_counts = HashMap::with_capacity(lines.len());

    for line in lines {
        let (lhs, rhs) = line.split_once("   ").unwrap();
        let lhs = lhs.parse::<usize>().unwrap();
        let rhs = rhs.parse::<usize>().unwrap();

        lhs_list.push(lhs);

        let entry = rhs_counts.entry(rhs).or_insert(0);
        *entry += 1;
    }

    let mut score = 0;
    for lhs in lhs_list {
        let count = rhs_counts.get(&lhs).unwrap_or(&0);
        score += lhs * *count;
    }

    println!("Part 2: {score}");
}
