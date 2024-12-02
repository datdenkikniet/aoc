fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    part1(&lines);
}

fn part1(lines: &[String]) {
    let mut safe_reports = 0;

    for line in lines {
        let mut levels = line.split(' ').map(|v| v.parse::<usize>().unwrap());
        let mut prev = levels.next().unwrap();

        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut all_have_right_diff = true;

        for level in levels {
            let min = prev.min(level);
            let max = prev.max(level);
            let diff = max - min;

            all_have_right_diff = all_have_right_diff && diff >= 1 && diff <= 3;
            all_decreasing = all_decreasing && level < prev;
            all_increasing = all_increasing && level > prev;

            prev = level;
        }

        if (all_increasing || all_decreasing) && all_have_right_diff {
            safe_reports += 1;
        }
    }

    println!("Part 1: {safe_reports}");
}
