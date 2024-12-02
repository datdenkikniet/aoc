fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    let reports: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| l.split(' ').map(|v| v.parse::<usize>().unwrap()).collect())
        .collect();

    part1(reports.clone());
    part2(reports.clone());
}

fn part1(reports: Vec<Vec<usize>>) {
    let mut safe_reports = 0;

    for report in reports {
        if report_is_safe(report.iter()) {
            safe_reports += 1;
        }
    }

    println!("Part 1: {safe_reports}");
}

fn part2(reports: Vec<Vec<usize>>) {
    let mut safe_reports = 0;

    for report in reports {
        if part2_rec(report) {
            safe_reports += 1;
        }
    }

    println!("Part 2: {safe_reports}");
}

fn part2_rec(mut report: Vec<usize>) -> bool {
    if report.len() == 0 || report.len() == 1 {
        return true;
    }

    if report_is_safe(report.iter()) {
        return true;
    }

    let mut any_safe = false;
    for i in 0..report.len() {
        let value = report.remove(i);
        any_safe |= report_is_safe(report.iter());
        report.insert(i, value);
    }

    any_safe
}

fn report_is_safe<'a>(mut report: impl Iterator<Item = &'a usize>) -> bool {
    let mut prev = report.next().unwrap();

    let mut all_increasing = true;
    let mut all_decreasing = true;
    let mut all_have_right_diff = true;

    for level in report {
        let min = prev.min(level);
        let max = prev.max(level);
        let diff = max - min;

        all_have_right_diff &= diff >= 1 && diff <= 3;
        all_decreasing &= level < prev;
        all_increasing &= level > prev;

        prev = level;
    }

    (all_increasing || all_decreasing) && all_have_right_diff
}
