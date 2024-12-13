fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();

    let (start, end) = line.split_once('-').unwrap();
    let start: usize = start.parse().unwrap();
    let end: usize = end.parse().unwrap();

    part1(start, end);
    part2(start, end);
}

fn part2(start: usize, end: usize) {
    let mut valid = 0;
    for password in start..=end {
        valid += is_valid_p2(password) as usize;
    }
    println!("Part 2: {valid}");
}

fn is_valid_p2(mut password: usize) -> bool {
    let mut groups = Vec::new();

    for _ in 0..5 {
        let digit_1 = password % 10;
        let digit_2 = (password / 10) % 10;

        if let Some((digit, len)) = groups.last_mut() {
            if *digit == digit_1 {
                *len += 1;
            } else {
                groups.push((digit_1, 1));
            }
        } else {
            groups.push((digit_1, 1));
        }

        if digit_1 < digit_2 {
            return false;
        }

        password /= 10;
    }

    // Password is already divided & now the last digit.
    let last_digit = password;

    if let Some((_, len)) = groups.last_mut().filter(|(digit, _)| *digit == last_digit) {
        *len += 1;
    }

    groups.iter().any(|(_, len)| *len == 2)
}

fn part1(start: usize, end: usize) {
    let mut valid = 0;
    for password in start..=end {
        valid += is_valid_p1(password) as usize;
    }
    println!("Part 1: {valid}")
}

fn is_valid_p1(mut password: usize) -> bool {
    let mut has_repeating = false;

    for _ in 0..5 {
        let digit_1 = password % 10;
        let digit_2 = (password / 10) % 10;

        has_repeating |= digit_1 == digit_2;

        if digit_1 < digit_2 {
            return false;
        }

        password /= 10;
    }

    has_repeating
}
