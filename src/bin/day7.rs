use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    pub fn cmp(&self, other: &Self, joker_as_any: bool) -> std::cmp::Ordering {
        if joker_as_any {
            if self == &Card::J && other != &Card::J {
                std::cmp::Ordering::Less
            } else if self != &Card::J && other == &Card::J {
                std::cmp::Ordering::Greater
            } else {
                Ord::cmp(self, other)
            }
        } else {
            Ord::cmp(self, other)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    counts: Vec<(Card, usize)>,
    jokers: usize,
}

macro_rules! define_hand_types {
    ($(($fn:ident, $ty:ident)),*) => {
        fn cmp(&self, other_hand: &Self, joker_as_any: bool) -> std::cmp::Ordering {
            use std::cmp::Ordering;

            let first_max = || {
                self.cards()
                    .iter()
                    .zip(other_hand.cards().iter())
                    .find_map(|(a, b)| {
                        let res = a.cmp(b, joker_as_any);
                        if res != Ordering::Equal {
                            Some(res)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(Ordering::Equal)
            };

            $(
                let mine = self.$fn(joker_as_any);
                let other = other_hand.$fn(joker_as_any);

                if mine && !other {
                    return Ordering::Greater;
                } else if !mine && other {
                    return Ordering::Less;
                } else if mine && other {
                    return first_max();
                }
            )*

            first_max()
        }
    };
}

impl Hand {
    fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    pub fn count_of<'a>(cards: impl Iterator<Item = &'a Card>, card: &Card) -> usize {
        cards.filter(|v| v == &card).count()
    }

    pub fn new(cards: [Card; 5]) -> Self {
        let counts: Vec<_> = Card::variants()
            .into_iter()
            .map(|c| (c, Self::count_of(cards.iter(), &c)))
            .collect();
        let jokers = Self::jokers(counts.iter());
        Self {
            cards,
            counts,
            jokers,
        }
    }

    fn n_of_kind(&self, n: usize, allow_joker: bool) -> Option<Card> {
        self.counts.iter().find_map(|(k, v)| {
            if v == &n && (allow_joker || k != &Card::J) {
                Some(*k)
            } else {
                None
            }
        })
    }

    fn has_n_of_kind(&self, n: usize, allow_joker: bool) -> bool {
        self.n_of_kind(n, allow_joker).is_some()
    }

    fn jokers<'a>(mut iter: impl Iterator<Item = &'a (Card, usize)>) -> usize {
        iter.find_map(|(k, v)| if k == &Card::J { Some(*v) } else { None })
            .unwrap()
    }

    pub fn five_of_kind(&self, joker_as_any: bool) -> bool {
        self.has_n_of_kind(5, !joker_as_any)
            || (joker_as_any
                && (self.jokers == 1 && self.has_n_of_kind(4, true)
                    || self.jokers == 2 && self.has_n_of_kind(3, false)
                    || self.jokers == 3 && self.has_n_of_kind(2, false)
                    || self.jokers >= 4))
    }

    pub fn four_of_kind(&self, joker_as_any: bool) -> bool {
        self.has_n_of_kind(4, !joker_as_any)
            || (joker_as_any
                && (self.jokers == 1 && self.has_n_of_kind(3, false)
                    || self.jokers == 2 && self.has_n_of_kind(2, false)
                    || self.jokers == 3))
    }

    pub fn full_house(&self, joker_as_any: bool) -> bool {
        let three_of_kind = self.has_n_of_kind(3, !joker_as_any);
        let two_of_kind = self.has_n_of_kind(2, !joker_as_any);

        let card_type_count = self.counts.iter().filter(|(_, c)| *c != 0).count();

        three_of_kind && two_of_kind
            || (joker_as_any
                && ((self.jokers == 1 && card_type_count == 3)
                    || (self.jokers == 2 && card_type_count == 3)))
    }

    pub fn three_of_kind(&self, joker_as_any: bool) -> bool {
        let three_of_kind = self.has_n_of_kind(3, !joker_as_any);

        three_of_kind
            || (joker_as_any
                && ((self.jokers == 1 && self.has_n_of_kind(2, false)) || self.jokers == 2))
    }

    pub fn two_pair(&self, joker_as_any: bool) -> bool {
        let two_of_kind_1 = if let Some(card) = self.n_of_kind(2, !joker_as_any) {
            card
        } else {
            return false;
        };

        let two_of_kind_2 = self
            .counts
            .iter()
            .find_map(|(k, v)| {
                if v == &2 && k != &two_of_kind_1 {
                    Some(k)
                } else {
                    None
                }
            })
            .is_some();

        two_of_kind_2 || (joker_as_any && self.jokers == 1 && self.has_n_of_kind(2, false))
    }

    pub fn one_pair(&self, joker_as_any: bool) -> bool {
        let two_of_kind = self.has_n_of_kind(2, !joker_as_any);

        two_of_kind || (joker_as_any && self.jokers == 1)
    }

    pub fn high_card(&self) -> Card {
        self.cards.iter().max().unwrap().clone()
    }

    define_hand_types!(
        (five_of_kind, FiveOfAKind),
        (four_of_kind, FourOfAKind),
        (full_house, FullHouse),
        (three_of_kind, ThreeOfAKind),
        (two_pair, TwoPair),
        (one_pair, OnePair)
    );
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other_hand: &Self) -> std::cmp::Ordering {
        Self::cmp(&self, other_hand, false)
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

    let mut joker_hands = hands.clone();

    hands.sort_by(|a, b| a.0.cmp(&b.0, false));
    joker_hands.sort_by(|a, b| a.0.cmp(&b.0, true));

    let calc_sum = |input: &[(Hand, usize)]| {
        input
            .iter()
            .enumerate()
            .map(|(r, (_, b))| b * (r + 1))
            .sum()
    };

    let sum: usize = calc_sum(&hands);
    let joker_sum: usize = calc_sum(&joker_hands);

    println!("Sum: {sum}");
    println!("Joker sum: {joker_sum}");

    Ok(())
}
