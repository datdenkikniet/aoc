use std::io::stdin;

fn main() -> std::io::Result<()> {
    let lines = stdin().lines();

    let mut total = 0;

    let string_reprs = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in lines {
        let line = line?;

        let mut chars = line.chars().peekable();
        let mut start_digit = None;
        let mut end_digit = None;

        while let Some(c) = chars.peek() {
            let mut digit_value = None;

            if c.is_ascii_digit() {
                digit_value = Some(*c as u32 - '0' as u32);
            } else {
                for (substr, value) in string_reprs {
                    if chars
                        .clone()
                        .zip(substr.chars())
                        .filter(|(a, b)| a == b)
                        .count()
                        == substr.len()
                    {
                        digit_value = Some(value);
                        break;
                    }
                }
            }

            if let Some(digit_value) = digit_value {
                if start_digit.is_none() {
                    start_digit = Some(digit_value);
                }

                end_digit = Some(digit_value);
            }

            chars.next();
        }

        let first_digit = start_digit.unwrap();
        let last_digit = end_digit.unwrap();

        total += (first_digit * 10) + last_digit;
        println!("{}: {}", line, first_digit * 10 + last_digit);
    }

    println!("{}", total);

    Ok(())
}
