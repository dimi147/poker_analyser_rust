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
    
    let mut data = Data::new();

    for (s, suit) in suits.iter().enumerate() {
        // auto suit = suits[s];
        let mut count = 0;

        for c in 0..13 {

            if c < 9 {
                let mask = 0x1f << c;
                if mask & *suit == mask {
                    data.straightflush = match data.straightflush {
                        Some(previous) => Some(max(previous, c + 4)),
                        None => Some(c + 4),
                    };
                }
            }

            if (0x1 << c) & *suit > 0 {
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
        
        if (0x100f & *suit) == 0x100f {
            data.straightflush = match data.straightflush {
                Some(previous) => Some(max(previous, 3)),
                None => Some(3),
            };
        }
    }

    if let (Some(straightflush), Some(flushsuit)) = (data.straightflush, data.flushsuit) {

        return Hand::straightflush(straightflush, flushsuit);

    } else if data.numquads == 1 {
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

        return Hand::quads(value, kicker);

    } else if data.numsets > 0 && data.numsets + data.numpairs > 1 {

        let mut set: Option<u8> = None;
        let mut pair: Option<u8> = None;

        data.values.iter().rev().enumerate().for_each(|(i, count)| {
            if *count == 3 && set.is_none() {
                set = Some(i as u8);
            } else if *count > 1 && pair.is_none() {
                pair = Some(i as u8);
            }
        });

        return Hand::fullhouse(set.unwrap(), pair.unwrap());

    } else if (data.flushsuit.is_some()) {
        let mut cards: Vec<u8> = vec!();
        let suit = suits[data.flushsuit.unwrap() as usize];

        (0..13).rev().for_each(|i: u8| {
            let mask = (1 as Suit) << i;
            if suit & mask > 0 {
                cards.push(i);
            }
        });

        return Hand::flush(cards[cards.len()-5..].try_into().unwrap(), data.flushsuit.unwrap());
    }

    let check_straight = |field: &mut Option<u8>, mask: Suit, top: u8| {
        if (mask & merged) == mask {
            *field = Some(match *field {
                Some(x) => max(x, top),
                None => top,
            })
        }
    };

    (0..9).for_each(|v| {
        let mask = (0x1f as Suit) << v;
        if (mask & merged) == mask {
            data.straight = Some(match data.straight {
                Some(x) => max(x, v + 4),
                None => v + 4,
            })
        }
    });
    // for (auto c = 0; c < 13; ++c) {
    //     if (c < 9) {
    //         auto mask = (Suit_t{0x1f} << c);
    //         if ((mask & merged) == mask)
    //             straight = std::max(straight, c + 4);
    //     }
    // }

    let mask = 0x100f;
    if (mask & merged) == mask {
        data.straight = Some(match data.straight {
            Some(x) => max(x, 3),
            None => 3,
        })
    }

    if let Some(top) = data.straight {

        return Hand::straight(top);

    } else if (data.numsets > 0) {
        let mut set: u8 = 0;
        let mut kickers: [u8; 2] = [0; 2];

        // for (int i = values.size() - 1; i >= 0; --i) {
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

            // if (value == 3)
            //     set = i;
            // else if (values[i] == 1)
            //     if (kicker1 == 0)
            //         kicker1 = i;
            //     else if (kicker2 == 0)
            //         kicker2 = i;
        }

        return Hand::set(set, kickers);

    } else if (data.numpairs > 0) {

        let mut pairs: Vec<u8> = vec!();
        let mut kickers: Vec<u8> = vec!();

        // for (int i = values.size() - 1; i >= 0; --i) {
        //for (i, count) in 
        data.values.iter().rev().enumerate().for_each(|(i, count)| {
            // const auto& count = values[i];
            if *count > 1 && pairs.len() < data.numpairs as usize {
                pairs.push(i as u8);
            } else if *count >= 1 && kickers.len() < 5 - (2 * data.numpairs) as usize{
                kickers.push(i as u8);
            }
        });

        return Hand::pairs(pairs, kickers);

    } else {
        let mut cards: Vec<u8> = vec!();

        data.values.iter().rev().enumerate().for_each(|(i, count)| {
        // for (int i = values.size() - 1; cards.size() < 5; --i)
            if (*count > 0) {
                cards.push(i as u8);
            }
        });

        return Hand::highcard(cards.as_slice()[0..5].try_into().unwrap());
    }
    
    // auto e = std::chrono::high_resolution_clock().now();
    // std::cout << std::dec << std::chrono::duration_cast<std::chrono::nanoseconds>(e-b).count() << std::endl;
}
