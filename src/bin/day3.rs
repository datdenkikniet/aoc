use std::{collections::HashMap, io::stdin};

#[derive(Clone, Debug)]
struct Number {
    value: u32,
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
}

impl Number {
    pub fn is_adjecent_to(&self, symbol: &Symbol) -> bool {
        let line_adjecent =
        // Same line
        symbol.line_idx == self.line_idx
            // Line before
            || symbol.line_idx + 1 == self.line_idx
            // Line after
            || self.line_idx + 1 == symbol.line_idx;

        let column_adjecent = if self.start_idx == 0 {
            (self.start_idx..self.end_idx + 1).contains(&symbol.idx)
        } else {
            (self.start_idx - 1..self.end_idx + 1).contains(&symbol.idx)
        };

        line_adjecent && column_adjecent
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Symbol {
    is_gear: bool,
    idx: usize,
    line_idx: usize,
}

fn main() -> std::io::Result<()> {
    let lines = stdin().lines().enumerate();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (line_idx, line) in lines {
        let line = line?;

        let mut chars = line.chars().enumerate().peekable();
        while let Some((idx, char)) = chars.next() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.push(Symbol {
                    idx,
                    line_idx,
                    is_gear: char == '*',
                });
            } else if char.is_ascii_digit() {
                let mut value = char as u32 - '0' as u32;
                let start_idx = idx;
                let mut end_idx = idx;

                while let Some(digit) = chars
                    .peek()
                    .map(|(_, v)| {
                        if v.is_ascii_digit() {
                            Some(*v as u32 - '0' as u32)
                        } else {
                            None
                        }
                    })
                    .flatten()
                {
                    value *= 10;
                    value += digit;
                    end_idx += 1;
                    chars.next();
                }

                numbers.push(Number {
                    value,
                    start_idx,
                    end_idx: end_idx + 1,
                    line_idx,
                })
            }
        }
    }

    let mut sum = 0;
    let mut adjecent_parts = 0;

    let mut gears: HashMap<Symbol, Vec<&Number>> = symbols
        .iter()
        .filter_map(|s| {
            if s.is_gear {
                Some((s.clone(), Vec::new()))
            } else {
                None
            }
        })
        .collect();

    for number in &numbers {
        for symbol in &symbols {
            if number.is_adjecent_to(symbol) {
                sum += number.value;
                println!(
                    "{} is adjecent to line {}, col {}",
                    number.value,
                    symbol.line_idx + 1,
                    symbol.idx + 1
                );
                adjecent_parts += 1;

                if let Some(adjecent_parts) = gears.get_mut(symbol) {
                    adjecent_parts.push(number);
                }
            }
        }
    }

    let gear_sum: u32 = gears
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0].value * v[1].value)
        .sum();

    println!("Total adjecent parts: {adjecent_parts}");
    println!("Sum of all of the part numbers: {sum}");
    println!("Gear sum: {gear_sum}");

    Ok(())
}
