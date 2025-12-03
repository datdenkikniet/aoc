fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[String]) {
    let mut sum = 0;

    for line in lines {
        sum += biggest(line, 2);
    }

    println!("Part 1: {sum}");
}

fn part2(lines: &[String]) {
    let mut sum = 0;

    for line in lines {
        sum += biggest(line, 12);
    }

    println!("Part 2: {sum}");
}

fn biggest(mut input: &str, k: usize) -> u64 {
    let mut sum = 0;

    for i in (0..k).rev() {
        let all_min_k = &input[..input.len() - i];
        let (num, num_idx) = do_big(all_min_k);
        input = &input[num_idx + 1..];

        sum *= 10;
        sum += num;
    }

    sum
}

fn do_big(input: &str) -> (u64, usize) {
    let mut lh = 0;
    let mut idx = 0;

    for (in_idx, v) in input.chars().enumerate() {
        let value = v as u64 - '0' as u64;
        if value > lh {
            idx = in_idx;
            lh = value;
        }
    }

    (lh, idx)
}
