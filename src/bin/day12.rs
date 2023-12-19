use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spring {
    Operational,
    Damaged,
}

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

impl std::fmt::Display for SpringRecordAndCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for record in &self.records {
            match record {
                SpringRecord::Operational => f.write_char('.')?,
                SpringRecord::Damaged => f.write_char('#')?,
                SpringRecord::Unknown => f.write_char('?')?,
            }
        }

        Ok(())
    }
}

impl SpringRecordAndCounts {
    fn arrangements(records: &[SpringRecord]) -> Vec<Vec<Spring>> {
        let arrangements_this_iter = match records.last() {
            None => return Vec::new(),
            Some(SpringRecord::Damaged) => &[Spring::Damaged][..],
            Some(SpringRecord::Operational) => &[Spring::Operational],
            Some(SpringRecord::Unknown) => &[Spring::Operational, Spring::Damaged],
        };

        let nested_arrangements = Self::arrangements(&records[..records.len() - 1]);
        let mut output_arrangements = Vec::new();

        if nested_arrangements.is_empty() {
            output_arrangements = arrangements_this_iter.iter().map(|v| vec![*v]).collect();
        } else {
            for arrangement in arrangements_this_iter {
                for mut nested in nested_arrangements.clone() {
                    nested.push(arrangement.clone());
                    output_arrangements.push(nested);
                }
            }
        }

        output_arrangements
    }

    fn groups(arrangement: &[Spring]) -> impl Iterator<Item = (usize, Spring)> + '_ {
        struct GroupIter<'a> {
            arrangement: &'a [Spring],
            current_idx: usize,
        }

        impl Iterator for GroupIter<'_> {
            type Item = (usize, Spring);

            fn next(&mut self) -> Option<Self::Item> {
                if self.current_idx < self.arrangement.len() {
                    let next_group_ty = self.arrangement[self.current_idx];
                    let mut next_group_len = 1;
                    self.current_idx += 1;

                    while Some(&next_group_ty) == self.arrangement.get(self.current_idx) {
                        next_group_len += 1;
                        self.current_idx += 1;
                    }

                    Some((next_group_len, next_group_ty))
                } else {
                    None
                }
            }
        }

        GroupIter {
            arrangement,
            current_idx: 0,
        }
    }

    fn is_valid(&self, arrangement: &[Spring]) -> bool {
        let mut counts = self.counts.iter().map(Clone::clone);
        let groups = Self::groups(arrangement);

        for (group_len, group_ty) in groups {
            match group_ty {
                Spring::Operational => continue,
                Spring::Damaged if counts.next() == Some(group_len) => continue,
                Spring::Damaged => return false,
            }
        }

        counts.next().is_none()
    }

    pub fn valid_arrangements(&self) -> usize {
        let output_arrangements = Self::arrangements(&self.records);

        output_arrangements
            .into_iter()
            .filter(|a| self.is_valid(&a))
            .count()
    }
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin().lines().map(|v| v.unwrap());

    let total_valid_arrangements: usize = lines
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

            let counts: Vec<_> = counts
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            let record = SpringRecordAndCounts {
                records: springs,
                counts,
            };

            record.valid_arrangements()
        })
        .sum();

    println!("Valid arrangements: {total_valid_arrangements}.");

    Ok(())
}
