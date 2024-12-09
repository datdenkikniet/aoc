use indicatif::ProgressBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringRecord {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SpringRecordAndCounts {
    records: Vec<SpringRecord>,
    counts: Vec<usize>,
}

impl SpringRecordAndCounts {
    pub fn valid_arrangement_count(&self) -> usize {
        let mut records = self.records.clone();
        let len = Self::valid_arrangements_impl(None, &mut records, &self.counts);
        len
    }

    fn valid_arrangements_impl(
        current_group: Option<usize>,
        records: &mut [SpringRecord],
        counts: &[usize],
    ) -> usize {
        let first = if let Some(record) = records.first() {
            record
        } else {
            return match current_group {
                Some(0) | None => {
                    if counts.is_empty() {
                        1
                    } else {
                        0
                    }
                }
                Some(_) => 0,
            };
        };

        let count = match first {
            SpringRecord::Damaged => match (current_group, counts.first()) {
                (None, None) => 0,
                (None, Some(n)) => {
                    Self::valid_arrangements_impl(Some(n - 1), &mut records[1..], &counts[1..])
                }
                (Some(0), _) => 0,
                (Some(n), _) => {
                    Self::valid_arrangements_impl(Some(n - 1), &mut records[1..], counts)
                }
            },
            SpringRecord::Operational => match current_group {
                None => Self::valid_arrangements_impl(None, &mut records[1..], counts),
                Some(0) => Self::valid_arrangements_impl(None, &mut records[1..], counts),
                Some(_) => 0,
            },
            SpringRecord::Unknown => {
                records[0] = SpringRecord::Damaged;
                let damaged = Self::valid_arrangements_impl(current_group, records, counts);
                records[0] = SpringRecord::Operational;
                let operational = Self::valid_arrangements_impl(current_group, records, counts);
                records[0] = SpringRecord::Unknown;

                damaged + operational
            }
        };

        count
    }

    pub fn unfold(&self) -> Self {
        let mut counts = Vec::new();
        let mut records = Vec::new();

        for i in 0..5 {
            counts.extend(self.counts.iter().cloned());
            records.extend(self.records.iter().cloned());

            if i != 4 {
                records.push(SpringRecord::Unknown);
            }
        }

        Self { counts, records }
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let records: Vec<_> = lines
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();

            let springs: Vec<_> = springs
                .chars()
                .map(|v| match v {
                    '?' => SpringRecord::Unknown,
                    '.' => SpringRecord::Operational,
                    '#' => SpringRecord::Damaged,
                    _ => panic!(),
                })
                .collect();

            let counts: Vec<_> = counts.split(',').map(|v| v.parse().unwrap()).collect();

            SpringRecordAndCounts {
                records: springs,
                counts,
            }
        })
        .collect();

    part1(&records);
    part2(&records);
}

fn part1(records: &[SpringRecordAndCounts]) {
    let total: usize = records
        .iter()
        .map(SpringRecordAndCounts::valid_arrangement_count)
        .sum();

    println!("Part 1: {total}");
}

fn part2(records: &[SpringRecordAndCounts]) {
    let total: usize = records
        .iter()
        .map(SpringRecordAndCounts::unfold)
        .map(|s| s.valid_arrangement_count())
        .sum();

    println!("Part 2: {total}");
}
