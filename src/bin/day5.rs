use std::io::stdin;

fn parse_int_list(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(|v| v.trim().parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct SeedRange {
    start: u64,
    len: u64,
}

impl SeedRange {
    pub fn end(&self) -> u64 {
        self.start + self.len - 1
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    destination_start: u64,
    source_start: u64,
    len: u64,
}

impl Range {
    fn source_end(&self) -> u64 {
        self.source_start + self.len - 1
    }

    pub fn contains(&self, number: u64) -> bool {
        number >= self.source_start && number < self.source_start + self.len
    }

    /// Returns:
    /// List of untransformed ranges
    /// Optional is the transformed range
    pub fn transform(&self, range: &SeedRange) -> (Vec<SeedRange>, Option<SeedRange>) {
        // Range is entirely outside
        if range.end() < self.source_start || range.start > self.source_end() {
            return (vec![range.clone()], None);
        }

        let mut untransformed_ranges = Vec::with_capacity(2);

        if range.start < self.source_start {
            let len = self.source_start - range.start;
            let start = range.start;

            untransformed_ranges.push(SeedRange { start, len });
        }

        if range.end() > self.source_end() {
            let len = range.end() - self.source_end();
            let start = self.source_end() + 1;

            untransformed_ranges.push(SeedRange { start, len });
        }

        let new_start = range.start.max(self.source_start);
        let new_end = range.end().min(self.source_end());

        let transformed_start = self.destination_start + (new_start - self.source_start);
        let transformed_len = new_end - new_start + 1;

        let transformed_range = SeedRange {
            start: transformed_start,
            len: transformed_len,
        };

        if transformed_start == 0 {
            println!(
                "{:?} transformed {:?} into {:?}",
                self, range, transformed_range
            )
        }

        (untransformed_ranges, Some(transformed_range))
    }
}

fn merge_ranges(ranges: &[SeedRange]) -> Vec<SeedRange> {
    let mut output_ranges = Vec::<SeedRange>::new();

    for range in ranges {
        let start = range.start;
        let end = range.end();
        let is_subrange = output_ranges
            .iter()
            .any(|r| r.start <= start && r.end() >= end);

        if !is_subrange {
            output_ranges.retain(|r| !(start <= r.start && end >= r.end()));

            output_ranges.push(range.clone());
        }
    }

    output_ranges
}

fn main() -> std::io::Result<()> {
    let mut lines = stdin().lines().map(|v| v.unwrap());

    let seeds = lines.next().unwrap();
    lines.next();

    let (_, seeds) = seeds.split_once(':').unwrap();
    let mut seeds: Vec<u64> = parse_int_list(seeds.trim());

    let mut seed_ranges = Vec::with_capacity(seeds.len() / 2);
    let mut seeds_iter = seeds.iter();

    while let Some(start) = seeds_iter.next() {
        let len = seeds_iter.next().unwrap();

        seed_ranges.push(SeedRange {
            start: *start,
            len: *len,
        });
    }

    // First line is always just the map name
    while lines.next().is_some() {
        let mut ranges = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let range_info = parse_int_list(&line);

            ranges.push(Range {
                destination_start: range_info[0],
                source_start: range_info[1],
                len: range_info[2],
            })
        }

        seeds.iter_mut().for_each(|v| {
            for range in &ranges {
                if range.contains(*v) {
                    let diff = *v - range.source_start;
                    let new = range.destination_start + diff;
                    *v = new;
                    break;
                }
            }
        });

        let mut unmapped_ranges = seed_ranges;
        let mut new_seed_ranges = Vec::new();
        loop {
            let unmapped_ranges_len = unmapped_ranges.len();
            let mut new_unmapped_ranges = Vec::new();

            for seed_range in &unmapped_ranges {
                for range in &ranges {
                    let (unmapped_ranges, mapped_range) = range.transform(&seed_range);

                    if let Some(mapped_range) = mapped_range {
                        new_seed_ranges.push(mapped_range);
                    }

                    new_unmapped_ranges.extend(unmapped_ranges.into_iter());
                }
            }

            println!(
                "Unmapped ranges: {}. New unmapped ranges: {}",
                unmapped_ranges_len,
                new_unmapped_ranges.len()
            );

            let new_unmapped_ranges = merge_ranges(&new_unmapped_ranges);

            if new_unmapped_ranges == unmapped_ranges {
                new_seed_ranges.extend(new_unmapped_ranges.into_iter());
                break;
            }

            unmapped_ranges = new_unmapped_ranges;
        }

        seed_ranges = merge_ranges(&new_seed_ranges);
        println!(
            "Min seed range: {:?}",
            seed_ranges.iter().min_by(|a, b| a.start.cmp(&b.start))
        );
    }

    println!("Lowest location: {}", seeds.iter().min().unwrap());
    println!(
        "Lowest location (ranges): {:?}",
        seed_ranges
            .iter()
            .min_by(|a, b| a.start.cmp(&b.start))
            .unwrap()
    );

    Ok(())
}

macro_rules! test {
    ($name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $name() {
            let range = Range {
                destination_start: 5,
                source_start: 3,
                len: 3,
            };

            let input: SeedRange = $input;
            let res = range.transform(&input);
            assert_eq!(res, $output);
            assert_eq!(
                res.0.iter().chain(res.1.iter()).map(|v| v.len).sum::<u64>(),
                $input.len
            );
        }
    };
}

test!(
    non_overlapping_neg,
    SeedRange { start: 0, len: 2 },
    (vec![SeedRange { start: 0, len: 2 }], None)
);

test!(
    non_overlapping_pos,
    SeedRange { start: 7, len: 2 },
    (vec![SeedRange { start: 7, len: 2 }], None)
);

test!(
    fully_overlapping,
    SeedRange { start: 3, len: 3 },
    (vec![], Some(SeedRange { start: 5, len: 3 }))
);

test!(
    partially_pos_overlapping,
    SeedRange { start: 3, len: 5 },
    (
        vec![SeedRange { start: 6, len: 2 }],
        Some(SeedRange { start: 5, len: 3 })
    )
);

test!(
    partially_neg_overlapping,
    SeedRange { start: 1, len: 5 },
    (
        vec![SeedRange { start: 1, len: 2 }],
        Some(SeedRange { start: 5, len: 3 })
    )
);

test!(
    both_overlapping,
    SeedRange { start: 1, len: 8 },
    (
        vec![
            SeedRange { start: 1, len: 2 },
            SeedRange { start: 6, len: 3 }
        ],
        Some(SeedRange { start: 5, len: 3 })
    )
);
