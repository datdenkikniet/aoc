fn main() {
    let modules: Vec<usize> = std::io::stdin()
        .lines()
        .map(|v| v.unwrap().parse().unwrap())
        .collect();

    part1(&modules);
    part2(&modules);
}

fn part1(modules: &[usize]) {
    let fuel_reqs: usize = modules.into_iter().map(|m| (m / 3) - 2).sum();

    println!("Part 1: {fuel_reqs}");
}

fn part2(modules: &[usize]) {
    let mut sum = 0;
    let mut modules: Vec<_> = modules.iter().cloned().collect();

    while !modules.is_empty() {
        for idx in (0..modules.len()).rev() {
            let value = modules[idx];
            if let Some(next_value) = (value / 3).checked_sub(2) {
                sum += next_value;
                modules[idx] = next_value;
            } else {
                modules.remove(idx);
            }
        }
    }

    println!("Part 2: {sum}");
}
