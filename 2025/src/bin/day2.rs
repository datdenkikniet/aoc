use std::ops::RangeInclusive;

fn main() {
    let range_strs = std::io::stdin().lines().map(|v| v.unwrap()).next().unwrap();

    let mut ranges = Vec::new();
    for range in range_strs.split(',') {
        let (from, to) = range.split_once('-').unwrap();

        let (from, to): (usize, usize) = (from.parse().unwrap(), to.parse().unwrap());
        ranges.push(from..=to);
    }

    part1(&ranges);
}

fn part1(ranges: &[RangeInclusive<usize>]) {
    let mut sum = 0;
    for range in ranges.iter().cloned() {
        for value in range {
            if invalid(value) {
                sum += value;
            }
        }
    }
    println!("Part 1: {sum}");
}

fn invalid(num: usize) -> bool {
    let n_digits = num.ilog10() + 1;
    let pow = 10u32.pow(n_digits / 2) as usize;

    if n_digits % 2 == 0 {
        num / pow == num % pow
    } else {
        false
    }
}
