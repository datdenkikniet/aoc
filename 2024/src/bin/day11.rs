use std::{collections::HashMap, time::Instant};

fn main() {
    let input: String = std::io::stdin().lines().map(|v| v.unwrap()).next().unwrap();
    let mut stones = Vec::new();

    for stone in input.split(' ') {
        let stone: usize = stone.parse().unwrap();
        stones.push(stone);
    }

    part1(&stones);
    part2(&stones);
}

fn part1(stones: &[usize]) {
    let total_start = Instant::now();
    let mut old_stones = stones.to_vec();
    let mut new_stones = Vec::with_capacity(2 * old_stones.len());

    for blink in 0..25 {
        let start = Instant::now();

        new_stones.clear();

        for stone in old_stones.iter().cloned() {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let digits = stone.ilog10() + 1;

                if digits % 2 == 0 {
                    let mask = 10usize.pow(digits / 2);
                    new_stones.push(stone / mask);
                    new_stones.push(stone % mask);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }

        std::mem::swap(&mut old_stones, &mut new_stones);

        println!(
            "Blink {} took {} ms",
            blink + 1,
            start.elapsed().as_millis()
        );
    }

    println!(
        "Part 1: {}. {} ms",
        old_stones.len(),
        total_start.elapsed().as_millis()
    );
}

fn part2(stones: &[usize]) {
    let mut known = HashMap::new();
    let sum: usize = stones
        .iter()
        .map(|v| expanded_count(&mut known, 75, *v))
        .sum();

    println!("Part 2: {sum}");
}

fn expanded_count(
    known: &mut HashMap<(usize, usize), usize>,
    iterations: usize,
    stone: usize,
) -> usize {
    if iterations == 0 {
        return 1;
    } else if let Some(known) = known.get(&(iterations, stone)) {
        return *known;
    }

    let next_iterations = iterations - 1;

    let count = if stone == 0 {
        expanded_count(known, next_iterations, 1)
    } else {
        let digits = stone.ilog10() + 1;

        if digits % 2 == 0 {
            let mask = 10usize.pow(digits / 2);
            expanded_count(known, next_iterations, stone / mask)
                + expanded_count(known, next_iterations, stone % mask)
        } else {
            expanded_count(known, next_iterations, stone * 2024)
        }
    };

    known.insert((iterations, stone), count);

    count
}
