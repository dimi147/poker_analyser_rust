use crate::deck::Deck;
use crate::analyser;

pub fn predict(players: &[Deck]) -> (f32, f32) {
    let deck = players.iter().sum();
    let combinations = find_all_combinations(deck, 5);
    let odds = compare_player_hands(players, &combinations);
    let tie_odds: f32= odds.iter().sum();
    (odds[0], 1f32 - tie_odds)
}

fn find_all_combinations(mut deck: Deck, k: u32) -> Vec<Deck> {
    let n = deck.count_zeros() - 12;
    assert!(n >= k);
    // println!("n={}", n);
    // let combination_num = calculate_combination_num(n, k as u32);
    let mut combinations = vec![]; //vec![0 as Deck; combination_num];
    find_next_combination(&mut combinations, &mut deck, 52 - n + k, 0);
    return combinations;
}


fn calculate_combination_num(n: u32, k: u32) -> u64 {
    assert!(n >= k);
    let factorial = |begin: u32, end: u32| {
        let mut f: u64 = 1;
        (begin..end+1).for_each(|i| {
            f *= i as u64;
        });
        f
    };

    return factorial(n - k + 1, n) / factorial(1, k);
}

fn find_next_combination(combinations: &mut Vec<u64>, deck: &mut u64, hand_size: u32, index: u32) {
    if deck.count_ones() == hand_size {
        combinations.push(*deck);
        return;
    }

    (index..52).for_each(|i| {
        if *deck & ((0x1 as u64) << i) == 0 {
            *deck |= 0x1 << i;
            find_next_combination(combinations, deck, hand_size, i + 1);
            *deck -= 0x1 << i;
        }
    });
}

fn compare_player_hands(players: &[Deck], combinations: &[Deck]) -> Vec<f32> {
    let mut player_odds = vec![0; players.len()];
    let all_players: Deck = players.iter().sum();

    for combination in combinations.iter().map(|c| c - all_players) {
        let mut hands = vec![analyser::analyse(combination + players[0])];
        let mut winner_index = 0;
        let mut winners = vec![winner_index];

        for i in 1..players.len() {
            let combination = combination + players[i];
            hands.push(analyser::analyse(combination));
            match hands[hands.len() - 1].partial_cmp(&hands[hands.len() - 2]) {
                Some(std::cmp::Ordering::Equal) => winners.push(i),
                Some(std::cmp::Ordering::Less) => {},
                Some(std::cmp::Ordering::Greater) => {
                    winner_index = i;
                    winners.clear();
                    winners.push(i);
                }
                None => panic!(),
            }
        }

        if winners.len() == 1 {
            player_odds[winners[0]] += 1;
        }
    }

    player_odds.iter().map(|i| {
        *i as f32 / combinations.len() as f32
    }).collect()
}

#[cfg(test)]
mod test {
    use super::{*};

    #[test]
    fn test_n50_k5() {
        let combinations: Vec<u64> = find_all_combinations(0b110, 5);
        assert_eq!(combinations.len() as u64, calculate_combination_num(52 - 2, 5));
    }

    #[test]
    #[should_panic(expected = "assertion failed: n >= k")]
    fn test_empty_deck() {
        let combinations: Vec<u64> = find_all_combinations(0xfffffffffffff, 1);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_overflown_deck() {
        let combinations: Vec<u64> = find_all_combinations(0x1fffffffffffff, 0);
    }

    #[test]
    fn test_n46_k1() {
        let combinations = find_all_combinations(0b1111110, 1);
        assert_eq!(combinations.len() as u64, calculate_combination_num(52 - 6, 1));
    }

    #[test]
    fn test_AKsuited_vs_72suited() {
        let players = &vec![0b100001, 0b1100000000000];
        let combinations = find_all_combinations(players[0] + players[1], 5);
        assert_eq!(combinations.len() as u64, calculate_combination_num(48, 5));
        let odds = compare_player_hands(players, &combinations);
        let tie_odds = 1f32 - odds[0] - odds[1];
        assert_eq!((tie_odds * 100000f32).trunc() as i32, 637);
    }
}
