use std::io::stdin;

fn parse_int_list(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect()
}

fn calculate_possible_wins(time: u64, record_distance: u64) -> u64 {
    // total time = C
    // held_time = x
    // speed = held_time
    // distance = y
    // distance = (total_time - held_time) * speed
    // distance = (C - x) * x
    // 0 = -x^2 + xC - y
    // a = -1, b = C, c = -y
    // x = (sqrt(C^2 - 4 * d) - C) / 2

    let sqrt_val = ((time.pow(2) - 4 * record_distance) as f64).sqrt().round() as u64;
    let record_held = (time - sqrt_val) / 2;
    let best_distance_hold = time / 2;
    let mut winning_times = (best_distance_hold - record_held) * 2;

    if time % 2 == 0 {
        // Subtract one to account for double-counting the best distance hold.
        winning_times -= 1;
    }

    winning_times
}

fn main() -> std::io::Result<()> {
    let mut lines = stdin().lines().map(|v| v.unwrap());

    let times = lines.next().unwrap();
    let (_, times_str) = times.split_once(':').unwrap();
    let times = parse_int_list(times_str.trim());

    let distances = lines.next().unwrap();
    let (_, distances_str) = distances.split_once(':').unwrap();
    let distances = parse_int_list(distances_str.trim());

    let times_distances = times.into_iter().zip(distances.into_iter());

    let mut total = None;
    for (time, distance) in times_distances {
        let wins = calculate_possible_wins(time, distance);

        if let Some(total) = total.as_mut() {
            *total *= wins;
        } else {
            total = Some(wins);
        }
    }

    let long_time: u64 = times_str
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let long_distance: u64 = distances_str
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let long_wins = calculate_possible_wins(long_time, long_distance);

    println!("{}, {}", total.unwrap(), long_wins);

    Ok(())
}
