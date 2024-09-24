use crate::deck::card_to_string;
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Hand {
    HighCard(HighCard),
    Pairs(Pairs),
    Set(Set),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    Quads(Quads),
    StraightFlush(StraightFlush),
}

impl Hand {
    pub fn highcard(cards: &[u8; 5]) -> Self {
        Hand::HighCard(*cards)
    }

    pub fn pairs(pairs: Vec<u8>, kickers: Vec<u8>) -> Self {
        Hand::Pairs(Pairs{pairs: pairs, kickers: kickers})
    }

    pub fn set(set: u8, kickers: &[u8; 2]) -> Self {
        Hand::Set(Set{set: set, kickers: *kickers})
    }

    pub fn straight(top: u8) -> Self {
        Hand::Straight(top)
    }

    pub fn flush(cards: &[u8; 5], suit: u8) -> Self {
        Hand::Flush(Flush{cards: *cards, suit: suit})
    }

    pub fn fullhouse(set: u8, pair: u8) -> Self {
        Hand::FullHouse(FullHouse{set, pair})
    }

    pub fn quads(quads: u8, kicker: u8) -> Self {
        Hand::Quads(Quads{quads, kicker})
    }

    pub fn straightflush(top: u8, suit: u8) -> Self {
        Hand::StraightFlush(StraightFlush{top, suit})
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        fn cts<T>(val: T) -> String
            where T: TryInto<i32>,
            <T as TryInto<i32>>::Error: std::fmt::Debug
        {
            let int: i32 = val.try_into().unwrap();
            if int < 0i32 {
                return "A".to_string();
            }
            card_to_string(int as u8)[0..1].to_string()
        }

        match self {
            Hand::HighCard(cards) => {
                write!(f, "highcard (")?;
                for c in cards {
                    write!(f, "{},", cts(*c))?;
                }
                write!(f, "{})", 8u8 as char)
            },
            Hand::Pairs(Pairs{pairs, kickers}) => {
                match pairs.len() { 
                    1 => write!(f, "pair {} ({}, {}, {})", cts(pairs[0]), cts(kickers[0]), cts(kickers[1]), cts(kickers[2])),
                    2 => write!(f, "pairs {} + {} ({})", cts(pairs[0]), cts(pairs[1]), cts(kickers[0])),
                     _ => panic!(),
                }
            }
            Hand::Set(set) => write!(f, "set {} ({}, {})", cts(set.set), cts(set.kickers[0]), cts(set.kickers[1])),
            Hand::Straight(top) => write!(f, "straight {}-{}", cts(*top as i32 - 4), cts(*top)),
            Hand::Flush(Flush{cards, suit}) => {
                write!(f, "flush (")?;
                for c in cards {
                    write!(f, "{},", card_to_string(c + suit * 13))?;
                }
                write!(f, "{})", 8u8 as char)
            },
            Hand::FullHouse(FullHouse{set, pair}) => write!(f, "fullhouse {} + {}", set, pair),
            Hand::Quads(Quads{quads, kicker}) => write!(f, "quads {} + {}", quads, kicker),
            Hand::StraightFlush(StraightFlush{top, suit}) => write!(f, "straightflush {}-{}"
                                                                        , card_to_string( match *top as i32 - 4 < 0 {
                                                                            true => 12,
                                                                            false => top - 4
                                                                        } + suit * 13)
                                                                        , card_to_string(top + suit * 13)),
        }
    }
}

type HighCard = [u8; 5];

#[derive(Debug, PartialEq)]
struct Pairs {
    pairs: Vec<u8>,
    kickers: Vec<u8>,
}

impl PartialOrd for Pairs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.pairs.len().partial_cmp(&other.pairs.len()) {
            Some(Ordering::Equal) => {},
            order => return order,
        }

        match self.pairs.partial_cmp(&other.pairs) {
            Some(Ordering::Equal) => {},
            order => return order,
        }

        assert_eq!(self.kickers.len(), other.kickers.len());

        self.kickers.partial_cmp(&other.kickers)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Set {
    set: u8,
    kickers: [u8; 2],
}

type Straight = u8;

#[derive(Debug)]
struct Flush {
    cards: [u8; 5],
    suit: u8,
}

impl PartialEq for Flush {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
    fn ne(&self, other: &Self) -> bool {
        self.cards.ne(&other.cards)
    }
}

impl PartialOrd for Flush {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}

// Default PartialOrd implementation automatically prioritizes
// the first property of the struct so set will be checked first
#[derive(Debug, PartialEq, PartialOrd)]
struct FullHouse {
    set: u8,
    pair: u8,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Quads {
    quads: u8,
    kicker: u8,
}

#[derive(Debug)]
struct StraightFlush {
    top: u8,
    suit: u8,
}

impl PartialEq for StraightFlush {
    fn eq(&self, other: &Self) -> bool {
        self.top.eq(&other.top)
    }
    fn ne(&self, other: &Self) -> bool {
        self.top.ne(&other.top)
    }
}

impl PartialOrd for StraightFlush {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.top.partial_cmp(&other.top)
    }
}

#[cfg(test)]
mod test {
    use super::Hand;
    #[test]
    fn compare_highcard_pair() {
        let highcard = Hand::highcard(&[12,11,10,9,8]);
        let pair = Hand::pairs(vec![2], vec![3,4,5]);
        assert!(pair > highcard);
    }

    #[test]
    fn compare_pair_set() {
        let set = Hand::set(2, &[3, 4]);
        let pair = Hand::pairs(vec![13, 12], vec![11]);
        assert!(set > pair);
    }
}