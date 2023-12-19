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
    fn arrangements<'a, F>(records: &[SpringRecord], mut visitor: F)
    where
        F: FnMut(&[Spring]),
    {
        let mut state = Vec::with_capacity(records.len());
        let indices_to_flip: Vec<_> = records
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if v == &SpringRecord::Unknown {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        state.clear();
        records
            .iter()
            .map(|v| match v {
                SpringRecord::Operational => Spring::Operational,
                SpringRecord::Damaged => Spring::Damaged,
                SpringRecord::Unknown => Spring::Operational,
            })
            .for_each(|v| state.push(v));

        let max = 2u64.checked_pow(indices_to_flip.len() as u32).unwrap();

        for mut flip_state in 0..max {
            indices_to_flip.iter().for_each(|v| {
                state[*v] = Spring::Operational;
            });

            let mut indices = indices_to_flip.iter();
            while flip_state > 0 {
                let should_flip = (flip_state & 1) == 1;
                flip_state >>= 1;
                let next_index = indices.next().unwrap();

                if should_flip {
                    state[*next_index] = Spring::Damaged;
                }
            }

            visitor(&state);
        }
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
        let mut state = 0;

        Self::arrangements(&self.records, |a| {
            if self.is_valid(a) {
                state += 1;
            }
        });

        state
    }

    pub fn unfold(&mut self) {
        println!("Unfolding");
        self.counts = std::iter::repeat(self.counts.iter())
            .take(5)
            .flat_map(|v| v.cloned())
            .collect();

        self.records = std::iter::repeat(self.records.iter())
            .take(5)
            .flat_map(|v| v.cloned())
            .collect();
        println!("Done unfolding");
    }
}

fn main() -> std::io::Result<()> {
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

            let counts: Vec<_> = counts
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            SpringRecordAndCounts {
                records: springs,
                counts,
            }
        })
        .collect();

    let total_valid_arrangements: usize = records
        .iter()
        .map(SpringRecordAndCounts::valid_arrangements)
        .sum();

    println!("Valid arrangements: {total_valid_arrangements}.");

    Ok(())
}
