use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u32)]
pub enum Card {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    T = 8,
    J = 9,
    Q = 10,
    K = 11,
    A = 12,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let card = match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' | 't' => Self::T,
            'J' | 'j' => Self::J,
            'Q' | 'q' => Self::Q,
            'K' | 'k' => Self::K,
            'A' | 'a' => Self::A,
            _ => return Err(()),
        };

        Ok(card)
    }
}

impl Card {
    pub fn variants() -> [Card; 13] {
        [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::T,
            Self::J,
            Self::Q,
            Self::K,
            Self::A,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    counts: Vec<(Card, usize)>,
}

impl Hand {
    fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    pub fn count_of<'a>(cards: impl Iterator<Item = &'a Card>, card: &Card) -> usize {
        cards.filter(|v| v == &card).count()
    }

    pub fn new(cards: [Card; 5]) -> Self {
        Self {
            cards,
            counts: Card::variants()
                .into_iter()
                .map(|c| (c, Self::count_of(cards.iter(), &c)))
                .collect(),
        }
    }

    fn n_of_kind(&self, n: usize) -> Option<Card> {
        self.counts
            .iter()
            .find_map(|(k, v)| if v == &n { Some(*k) } else { None })
    }

    pub fn five_of_kind(&self) -> Option<Card> {
        self.n_of_kind(5)
    }

    pub fn four_of_kind(&self) -> Option<(Card, Card)> {
        let four_of_kind = self.n_of_kind(4)?;
        let last_kind = self.n_of_kind(1).unwrap();

        Some((four_of_kind, last_kind))
    }

    pub fn full_house(&self) -> Option<(Card, Card)> {
        let three_of_kind = self.n_of_kind(3)?;
        let two_of_kind = self.n_of_kind(2)?;

        Some((three_of_kind, two_of_kind))
    }

    pub fn three_of_kind(&self) -> Option<(Card, Card, Card)> {
        let three_of_kind = self.n_of_kind(3)?;
        let mut others = self.cards().into_iter().filter(|v| v != &&three_of_kind);
        let (other_1, other_2) = (others.next().unwrap(), others.next().unwrap());

        Some((three_of_kind, *other_1, *other_2))
    }

    pub fn two_pair(&self) -> Option<(Card, Card, Card)> {
        let two_of_kind_1 = self.n_of_kind(2)?;
        let two_of_kind_2 = self.counts.iter().find_map(|(k, v)| {
            if v == &2 && k != &two_of_kind_1 {
                Some(k)
            } else {
                None
            }
        })?;
        let other_kind = self
            .counts
            .iter()
            .map(|(k, _)| k)
            .find(|k| k != &&two_of_kind_1 && k != &two_of_kind_2)
            .unwrap();

        Some((two_of_kind_1, *two_of_kind_2, *other_kind))
    }

    pub fn one_pair(&self) -> Option<(Card, Card, Card, Card)> {
        let two_of_kind = self.n_of_kind(2)?;
        let mut others = self.cards().into_iter().filter(|v| v != &&two_of_kind);
        let (other_1, other_2, other_3) = (
            others.next().unwrap(),
            others.next().unwrap(),
            others.next().unwrap(),
        );

        Some((two_of_kind, *other_1, *other_2, *other_3))
    }

    pub fn high_card(&self) -> Card {
        let mut high_card = self.cards.into_iter().collect::<Vec<_>>();
        high_card.dedup();
        assert!(high_card.len() == 5);
        self.cards.iter().max().unwrap().clone()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other_hand: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let first_max = self
            .cards()
            .iter()
            .zip(other_hand.cards().iter())
            .find_map(|(a, b)| {
                let res = a.cmp(b);
                if res != Ordering::Equal {
                    Some(res)
                } else {
                    None
                }
            })
            .unwrap_or(Ordering::Equal);

        macro_rules! greater_or_first_max {
            ($fn:ident, $ty:ident) => {
                let mine = self.$fn().is_some();
                let other = other_hand.$fn().is_some();

                if mine && !other {
                    return Ordering::Greater;
                } else if !mine && other {
                    return Ordering::Less;
                } else if mine && other {
                    return first_max;
                }
            };
        }

        greater_or_first_max!(five_of_kind, FiveOfAKind);
        greater_or_first_max!(four_of_kind, FourOfAKind);
        greater_or_first_max!(full_house, FullHouse);
        greater_or_first_max!(three_of_kind, ThreeOfAKind);
        greater_or_first_max!(two_pair, TwoPair);
        greater_or_first_max!(one_pair, OnePair);

        first_max
    }
}

fn main() -> std::io::Result<()> {
    let lines = stdin().lines().map(|v| v.unwrap());

    let mut hands: Vec<_> = lines
        .map(|v| {
            let (hand, bid) = v.split_once(' ').unwrap();

            let mut cards = [Card::A; 5];
            cards
                .iter_mut()
                .zip(hand.trim().chars())
                .for_each(|(card, char)| *card = Card::try_from(char).unwrap());
            let bid: usize = bid.trim().parse().unwrap();

            (Hand::new(cards), bid)
        })
        .collect();

    hands.sort_by_key(|a| a.0.clone());

    hands.iter().for_each(|(h, b)| {
        println!("{:?}, {b}", h.cards);
    });

    let sum: usize = hands
        .iter()
        .enumerate()
        .map(|(r, (_, b))| b * (r + 1))
        .sum();

    println!("Sum: {sum}");

    Ok(())
}
