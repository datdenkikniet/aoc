fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    part1(&lines);
}

fn part1(lines: &[String]) {
    let mut sum = 0;

    for line in lines {
        let all_but_last = &line[..line.len() - 1];
        let mut lh = 0;
        let mut idx = 0;

        for (in_idx, v) in all_but_last.chars().enumerate() {
            let value = v as u32 - '0' as u32;
            if lh < value {
                idx = in_idx;
                lh = value;
            }
        }

        let rhs = &line[idx + 1..];
        let mut rh = 0;

        for v in rhs.chars() {
            let value = v as u32 - '0' as u32;
            if rh < value {
                rh = value;
            }
        }

        sum += (lh * 10) + rh;
    }

    println!("Part 1: {sum}");
}
