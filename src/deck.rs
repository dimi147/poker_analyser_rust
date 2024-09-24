use rand::distributions::Uniform;
use rand::prelude::Distribution;

pub type Deck = u64;

// const VALUES: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
// const SUITS: [&str; 4] = ["C", "H", "S", "D"];

pub fn to_deck<T: AsRef<str>>(cards: &[T]) -> Deck
{
    let mut deck: Deck = 0;
    for card in cards {
        deck |= (1 << card_from_string(card.as_ref()));
    }
    deck
}

pub fn from_deck(deck: Deck) -> Vec<String> {
    assert_eq!(deck >> 52, 0);
    let mut v = vec![];
    (0..52).for_each(|i|{
        if (deck & 0x1 << i) > 0 {
            v.push(card_to_string(i));
        }
    });
    v
}

pub fn card_to_string(value: u8) -> String {
    // let suit = ;
    // let number = value % 13;
    String::from(
        match value % 13 {
            0 => "2",
            1 => "3",
            2 => "4",
            3 => "5",
            4 => "6",
            5 => "7",
            6 => "8",
            7 => "9",
            8 => "T",
            9 => "J",
            10 => "Q",
            11 => "K",
            12 => "A",
            _ => panic!(),
        }) + match value / 13 {
            0 => "c",
            1 => "h",
            2 => "s",
            3 => "d",
            _ => panic!(),
        }
}

pub fn card_from_string(s: &str) -> u8 {
    // let value = VALUES.iter().position(|&r| r == &s[0..1]).unwrap() as u8;
    // let suit = SUITS.iter().position(|&r| r == &s[0..2]).unwrap() as u8;
    // (suit * 4) + value

    let value: u8 = match &s[0..1] {
        "2" => 0,
        "3" => 1,
        "4" => 2,
        "5" => 3,
        "6" => 4,
        "7" => 5,
        "8" => 6,
        "9" => 7,
        "T" => 8,
        "J" => 9,
        "Q" => 10,
        "K" => 11,
        "A" => 12,
        _ => {panic!()},
    };

    let suit = match &s[1..2].to_lowercase()[..] {
        "c" => 0,
        "h" => 1,
        "s" => 2,
        "d" => 3,
        _ => {panic!()},
    };

    suit * 13 + value

    // let value = Value::try_from(&s[0..1]).unwrap() as u8;
    // let suit = Suit::try_from(&s[1..2]).unwrap() as u8;
    // suit * 4 + value
}

pub fn get_random_card(deck: Deck) -> Deck {
    let distribution = Uniform::new(0, deck.count_zeros() - 12);
    let mut generator = rand::thread_rng();
    let mut value = distribution.sample(&mut generator);

    for i in 0..52 {
        if deck & (0x1 << i) != 0 {
            continue;
        }
        if value == 0 {
            return 0x1 << i;
        }
        value -= 1;
    }

    panic!();
}

#[cfg(test)]
mod test {
    use super::{*};

    #[test]
    fn test_card_string_conversions() {
        (0..52).for_each(|val| {
            let s = super::card_to_string(val);
            assert_eq!(super::card_from_string(&s), val);
        });
    }

    #[test]
    fn test_deck_conversions() {
        let d = 0xfffffffffffff as Deck;
        let c = from_deck(d);
        let d2 = to_deck(&c);
        assert_eq!(d, d2);
    }

    #[test]
    fn test_random_deck_generation() {
        let mut deck = 0;
        (0..52).for_each(|_| {
            deck += get_random_card(deck);
        });
        assert_eq!(deck, 0xfffffffffffff);
    }
}