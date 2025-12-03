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
    part2(&ranges);
    assert!(!invalid_pt2(60606));
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

fn part2(ranges: &[RangeInclusive<usize>]) {
    let mut sum = 0;
    for range in ranges.iter().cloned() {
        for value in range {
            if invalid_pt2(value) {
                sum += value;
            }
        }
    }
    println!("Part 2: {sum}");
}

fn invalid_pt2(num: usize) -> bool {
    let log = num.ilog10();
    let add = if log % 2 == 0 { 0 } else { 1 };

    for n_digits in 1..=(log / 2 + add) {
        let pow = 10usize.pow(n_digits);
        let digits = num % pow;
        let mut left = num / pow;

        // Remove leading-zero false positives.
        // Example: num: 60606, num_digits: 2
        if digits < pow / 10 {
            continue;
        }

        while (left % pow) == digits {
            left /= pow;
        }

        if left == 0 {
            return true;
        }
    }

    return false;
}
