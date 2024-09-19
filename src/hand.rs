use std::cmp::Ordering;

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

    pub fn set(set: u8, kickers: [u8; 2]) -> Self {
        Hand::Set(Set{set: set, kickers: kickers})
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

// impl PartialOrd for Set {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match self.set.partial_cmp(&other.set) {
//             Some(Ordering::Equal) => {},
//             order => return order,
//         }

//         assert_eq!(self.kickers.len(), other.kickers.len());

//         self.kickers.partial_cmp(&other.kickers)
//     }
// }

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