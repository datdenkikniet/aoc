use std::{collections::HashMap, io::stdin};

#[derive(Debug, Clone)]
struct ScratchCard {
    card_number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn wins(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|v| self.winning_numbers.contains(v))
            .count()
    }
}

fn main() -> std::io::Result<()> {
    let lines = stdin().lines();

    let mut sum = 0;
    let mut cards = Vec::new();
    for line in lines {
        let line = line?;

        let (card_no, numbers) = line.split_once(':').unwrap();

        let card_no: u32 = card_no.split(' ').last().unwrap().trim().parse().unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once('|').unwrap();
        let winning_numbers = winning_numbers.trim();

        let winning_numbers: Vec<u32> = winning_numbers
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse().unwrap())
            .collect();

        let my_numbers: Vec<_> = my_numbers
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<u32>().unwrap())
            .collect();

        let mut total = None;

        for my_number in &my_numbers {
            if winning_numbers.contains(&my_number) {
                if let Some(total) = total.as_mut() {
                    *total *= 2;
                } else {
                    total = Some(1);
                }

                println!("{my_number} is winning on card {card_no}.");
            }
        }

        if let Some(total) = total {
            sum += total;
        }

        cards.push(ScratchCard {
            card_number: card_no,
            winning_numbers,
            my_numbers,
        });
    }

    let reversed = cards.iter().enumerate().rev();
    let mut state = HashMap::new();
    let mut total_cards = 0;

    for (idx, card) in reversed {
        let wins = card.wins();
        let succeeding_cards = cards.iter().skip(idx + 1).take(wins);

        let mut this_card = 1;
        for successive in succeeding_cards {
            this_card += state.get(&successive.card_number).unwrap();
        }

        total_cards += this_card;
        state.insert(card.card_number, this_card);
    }

    println!("Sum: {sum}");
    println!("Total cards: {total_cards}");

    Ok(())
}
