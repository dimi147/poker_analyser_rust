use crate::deck::Deck;
use crate::hand::Hand;
use std::cmp::max;
use std::ops::ControlFlow;

type Suit = u16;

pub fn analyse(deck: Deck) -> Hand {
    let suits = split_suits(deck);
    let merged = merge_suits(&suits);
    run_analysis(&suits, merged)
}

fn split_suits(deck: Deck) -> [Suit; 4] {
    let mut suits: [Suit; 4] = [0; 4];

    for (i, suit) in suits.iter_mut().enumerate() {
        *suit = (deck >> (13 * i)) as Suit;
    }

    return suits;
}

fn merge_suits(suits: &[Suit; 4]) -> Suit {
    let mut merged: Suit = 0;

    for suit in suits {
        merged |= suit;
    }

    return merged;
}

#[derive(Debug)]
struct Data {
    values: [u8; 13],
    numquads: u8,
    numsets: u8,
    numpairs: u8,
    straightflush: Option<u8>,
    straight: Option<u8>,
    flushsuit: Option<u8>,
}

impl Data {
    fn new() -> Self {
        Data {
            values: [0; 13],
            numquads: 0,
            numsets: 0,
            numpairs: 0,
            straightflush: None,
            straight: None,
            flushsuit: None,
        }
    }
}

fn run_analysis(suits: &[Suit; 4], merged: Suit) -> Hand {
    // auto b = std::chrono::high_resolution_clock().now();
    
    let mut data = analyse_suits_separately(&suits);
    // println!("{:?}", data);

    if let Some(hand) = check_straightflush(&data) {
        return hand;
    } else if let Some(hand) = check_quads(&data) {
        return hand;
    } else if let Some(hand) = check_fullhouse(&data) {
        return hand;
    } else if let Some(hand) = check_flush(&data, &suits) {
        return hand;
    }

    analyse_merged_suits(merged, &mut data);
   
    if let Some(hand) = check_straight(&data) {
        return hand;
    } else if let Some(hand) = check_set(&data) {
        return hand;
    } else if let Some(hand) = check_pairs(&data) {
        return hand;
    } else {
        let mut cards: Vec<u8> = vec!();

        data.values.iter().rev().enumerate().for_each(|(i, count)| {
            if *count > 0 {
                cards.push(i as u8);
            }
        });

        Hand::highcard(cards.as_slice()[0..5].try_into().unwrap())
    }

    // auto e = std::chrono::high_resolution_clock().now();
    // std::cout << std::dec << std::chrono::duration_cast<std::chrono::nanoseconds>(e-b).count() << std::endl;
}

fn analyse_straight(suit: Suit, field: &mut Option<u8>, mask: Suit, top: u8) {
    if (mask & suit) == mask {
        *field = Some(match *field {
            Some(x) => max(x, top),
            None => top,
        })
    }
}

fn analyse_suits_separately(suits: &[Suit; 4]) -> Data {
    let mut data = Data::new();

    for (s, suit) in suits.iter().enumerate() {
        let mut count = 0;

        for c in 0..13 {

            if c < 9 {
                analyse_straight(*suit, &mut data.straightflush, 0x1f << c, c + 4);
            }

            if ((0x1 << c) & *suit) > 0 {
                count += 1;
                data.values[c as usize] += 1;

                match data.values[c as usize] {
                    4 => {
                        data.numquads += 1;
                        data.numsets -= 1;
                    },
                    3 => {
                        data.numsets += 1;
                        data.numpairs -= 1;
                    },
                    2 => data.numpairs += 1,
                    _ => {},
                }
            }
        }

        if count >= 5 {
            data.flushsuit = Some(s as u8);
        }

        analyse_straight(*suit, &mut data.straightflush, 0x100f, 3);
    }

    data
}

fn analyse_merged_suits(merged: Suit, data: &mut Data) {
    (0..9).for_each(|v| {
        analyse_straight(merged, &mut data.straight, 0x1f << v, v + 4);
    });

    analyse_straight(merged, &mut data.straight, 0x100f, 3);
}

fn check_straightflush(data: &Data) -> Option<Hand> {
    if let (Some(straightflush), Some(flushsuit)) = (data.straightflush, data.flushsuit) {
        return Some(Hand::straightflush(straightflush, flushsuit));
    } 

    None
}

fn check_quads(data: &Data) -> Option<Hand> {
    if data.numquads == 1 {
        let mut value = 0;
        let mut kicker = 0;

        data.values.iter().rev().enumerate().try_for_each(|(i, count)| {
            match count {
                4 => value = i as u8,
                _ => kicker = max(kicker, i as u8),
            }

            if value > 0 && kicker > 0 {
                return ControlFlow::Break((i, count));
            }

            ControlFlow::Continue(())
        });

        return Some(Hand::quads(value, kicker));

    } 

    None
}

fn check_fullhouse(data: &Data) -> Option<Hand> {
    if data.numsets > 0 && data.numsets + data.numpairs > 1 {

        let mut set: Option<u8> = None;
        let mut pair: Option<u8> = None;

        data.values.iter().rev().enumerate().for_each(|(i, count)| {
            if *count == 3 && set.is_none() {
                set = Some(i as u8);
            } else if *count > 1 && pair.is_none() {
                pair = Some(i as u8);
            }
        });

        return Some(Hand::fullhouse(set.unwrap(), pair.unwrap()));

    } 

    None
}

fn check_flush(data: &Data, suits: &[Suit; 4]) -> Option<Hand> {
    if data.flushsuit.is_some() {
        let mut cards: Vec<u8> = vec!();
        let suit = suits[data.flushsuit.unwrap() as usize];

        (0..13).rev().for_each(|i: u8| {
            let mask = (1 as Suit) << i;
            if suit & mask > 0 {
                cards.push(i);
            }
        });

        return Some(Hand::flush(cards[cards.len()-5..].try_into().unwrap(), data.flushsuit.unwrap()));
    }

    None
}

fn check_straight(data: &Data) -> Option<Hand> {
    if let Some(top) = data.straight {
        return Some(Hand::straight(top));
    } 

    None
}

fn check_set(data: &Data) -> Option<Hand> {
    if data.numsets > 0 {
        let mut set: u8 = 0;
        let mut kickers: [u8; 2] = [0; 2];

        for (i, value) in data.values.iter().rev().enumerate() {

            match value {
                3 => set = i as u8,
                1 => match kickers[0] {
                    0 => kickers[0] = i as u8,
                    _ => match kickers[1] {
                        0 => kickers[1] = i as u8,
                        _ => {},
                    }
                }
                _ => {},
            }
        }

        return Some(Hand::set(set, &kickers));

    } 

    None
}

fn check_pairs(data: &Data) -> Option<Hand> {
    if data.numpairs > 0 {

        let mut pairs: Vec<u8> = vec!();
        let mut kickers: Vec<u8> = vec!();

        data.values.iter().rev().enumerate().for_each(|(i, count)| {
            let numpairs = match data.numpairs > 1 { true => 2, false => 1};
            if *count > 1 && pairs.len() < numpairs as usize {
                pairs.push(i as u8);
            } else if *count >= 1 && kickers.len() < 5 - (2 * numpairs) as usize {
                kickers.push(i as u8);
            }
        });

        return Some(Hand::pairs(pairs, kickers));

    } 

    None
}

#[cfg(test)]
mod test {
    use crate::hand;
    use crate::deck;

    use super::analyse;
    
    #[test]
    fn test() {
        use crate::hand::Hand;
        let a = Hand::pairs(vec![10], vec![1,4,5]);
        let b = Hand::pairs(vec![0, 1], vec![3]);
        assert!(a < b);
    
        let a = Hand::fullhouse(8, 10);
        let b = Hand::fullhouse(10, 8);
        assert!(a < b);

        assert_eq!(super::analyse(0b1111111 as deck::Deck), hand::Hand::straightflush(6, 0));
    }

    fn test_check_pair() {
        let deck = deck::to_deck(&["Ad", "Ac", "7d", "8c", "Th", "9s", "3s"]);
        let hand = analyse(deck);
        assert_eq!(hand, hand::Hand::pairs(vec![12], vec![9, 8, 7]));
    }
}