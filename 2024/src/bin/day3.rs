fn main() {
    let input: String = std::io::stdin().lines().map(|v| v.unwrap()).collect();

    part1(&input);
    part2(&input);
}

fn part1(mut input: &str) {
    let mut sum = 0;
    for _ in 0..input.len() {
        if let Some((len, val)) = find_mul(&input) {
            input = &input[len..];
            sum += val;
        } else if input.len() > 0 {
            input = &input[1..];
        } else {
            break;
        }
    }

    println!("Part 1: {sum}");
}

fn part2(mut input: &str) {
    let mut sum = 0;
    let mut enabled = true;
    for _ in 0..input.len() {
        if let Some((len, val)) = find_mul(&input) {
            input = &input[len..];
            if enabled {
                sum += val;
            }
        } else if let Some((len, val)) = do_or_dont(&input) {
            input = &input[len..];
            enabled = val;
        } else if input.len() > 0 {
            input = &input[1..];
        } else {
            break;
        }
    }

    println!("Part 2: {sum}");
}

fn do_or_dont(input: &str) -> Option<(usize, bool)> {
    if input.starts_with("do()") {
        Some((4, true))
    } else if input.starts_with("don't()") {
        Some((6, false))
    } else {
        None
    }
}

fn find_mul(input: &str) -> Option<(usize, usize)> {
    let nums_endparen = if input.starts_with("mul(") {
        &input[4..]
    } else {
        return None;
    };

    let mut num1 = 0;
    let mut num2 = 0;
    let mut len = 4;
    let mut parsed_num1 = false;

    for i in nums_endparen.chars().take(8) {
        len += 1;

        if !parsed_num1 {
            if i == ',' {
                parsed_num1 = true;
            } else if i.is_ascii_digit() {
                num1 *= 10;
                num1 += i as u32 - '0' as u32;
            } else {
                return None;
            }
        } else {
            if i == ')' {
                break;
            } else if i.is_ascii_digit() {
                num2 *= 10;
                num2 += i as u32 - '0' as u32;
            } else {
                return None;
            }
        }
    }

    Some((len, (num1 * num2) as usize))
}
