fn part1(sequences: &Vec<Vec<i64>>) -> i64 {
    assert!(sequences[sequences.len() - 1].iter().all(|v| *v == 0));

    let mut diff = 0;
    for sequence in sequences.iter().rev().skip(1) {
        let last_val = sequence[sequence.len() - 1];
        diff = last_val + diff;
    }

    diff
}

fn part2(sequences: &Vec<Vec<i64>>) -> i64 {
    assert!(sequences[sequences.len() - 1].iter().all(|v| *v == 0));

    let mut diff = 0;
    for sequence in sequences.iter().rev().skip(1) {
        let last_val = sequence[0];
        diff = last_val - diff;
    }

    diff
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let sequences: Vec<_> = lines
        .map(|v| {
            v.split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for mut sequence in sequences {
        let mut this_line_sequences = vec![sequence.clone()];
        while !sequence.iter().all(|v| v == &0) {
            let mut iter = sequence.iter();

            let mut diffs = Vec::with_capacity(sequence.len() - 1);
            if let Some(mut previous_value) = iter.next().cloned() {
                while let Some(next_value) = iter.next() {
                    diffs.push(next_value - previous_value);
                    previous_value = *next_value;
                }
            }

            this_line_sequences.push(diffs.clone());
            sequence = diffs;
        }

        part1_sum += part1(&this_line_sequences);
        part2_sum += part2(&this_line_sequences);
    }

    println!("Part 1: {part1_sum}");
    println!("Part 2: {part2_sum}");
    Ok(())
}
