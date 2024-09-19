pub type Deck = u64;

// const VALUES: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
// const SUITS: [&str; 4] = ["C", "H", "S", "D"];

pub fn to_deck(cards: &Vec<&str>) -> Deck {
    let mut deck: Deck = 0;
    for card in cards {
        deck &= (1 << card_from_string(card));
    }
    deck
}

fn card_to_string(value: u8) -> String {
    // let suit = ;
    // let number = value % 13;
    String::from(
        match value / 13 {
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
        }) + match value % 13 {
            0 => "C",
            1 => "H",
            2 => "S",
            3 => "D",
            _ => panic!(),
        }
}

fn card_from_string(s: &str) -> u8 {
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

    let suit = match &s[1..2] {
        "C" => 0,
        "H" => 1,
        "S" => 2,
        "D" => 3,
        _ => {panic!()},
    };

    suit * 4 + value

    // let value = Value::try_from(&s[0..1]).unwrap() as u8;
    // let suit = Suit::try_from(&s[1..2]).unwrap() as u8;
    // suit * 4 + value
}
