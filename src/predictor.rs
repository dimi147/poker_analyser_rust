use super::deck::Deck;

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

fn compare_player_hands(players: &Vec<Deck>, combinations: &Vec<Deck>) -> Vec<f32> {
    let mut player_odds = vec![0; players.len()];

    combinations.iter().for_each(|combination: &u64| {
        let mut combination = *combination;
        players.iter().for_each(|player| {
            combination -= player;
        });

        let mut hands = vec![];

        players.iter().for_each(|player| {
            combination += player;
            hands.push(crate::analyser::analyse(combination));
            combination -= player;
        });

        let mut winner_index = 0;
        let mut winners = vec![0];

        (1..hands.len()).for_each(|i| {
            match hands[i].partial_cmp(&hands[winner_index]) {
                Some(std::cmp::Ordering::Equal) => winners.push(i),
                Some(std::cmp::Ordering::Less) => {},
                Some(std::cmp::Ordering::Greater) => {
                    winner_index = i;
                    winners.clear();
                    winners.push(i);
                }
                None => panic!(),
            }
        });

        if winners.len() == 1 {
            player_odds[winners[0]] += 1;
        }
    });

    println!("{:?}", player_odds);

    player_odds.iter().map(|i| {
        *i as f32 / combinations.len() as f32
    }).collect()
}

#[cfg(test)]
mod test {
    use super::{*};
    use crate::deck::{*};

    #[test]
    fn test() {
        let combinations: Vec<u64> = find_all_combinations(0b110, 5);
        assert_eq!(combinations.len() as u64, calculate_combination_num(52 - 2, 5));
    }

    #[test]
    #[should_panic(expected = "assertion failed: n >= k")]
    fn test1() {
        let combinations: Vec<u64> = find_all_combinations(0xfffffffffffff, 1);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test2() {
        let combinations: Vec<u64> = find_all_combinations(0x1fffffffffffff, 0);
    }

    #[test]
    fn test3() {
        let combinations = find_all_combinations(0b1111110, 1);
        assert_eq!(combinations.len() as u64, calculate_combination_num(52 - 6, 1));
    }

    #[test]
    fn test_predictor() {
        let players = &vec![0b100001, 0b1100000000000];
        let combinations = find_all_combinations(players[0] + players[1], 5);
        assert_eq!(combinations.len() as u64, calculate_combination_num(48, 5));
        let odds = compare_player_hands(players, &combinations);
        println!("{:?}", players);
        println!("{:?}", odds);
    }
}
/*
    static void calculate(const unsigned k, const std::vector<CardValue_52_t>& cards, std::vector<std::vector<CardValue_52_t>>& combinations, 
                          std::vector<CardValue_52_t>& combination, unsigned index = 0)
    {
        if (combination.size() == k) {
            // combinations.push_back(combination);
            return;
        }

        for (auto i = index; i < cards.size(); ++i) {
            combination.push_back(cards[i]);
            calculate(k, cards, combinations, combination, i + 1);
            combination.pop_back();
        }
    }
};

class Predictor {
public:
    Predictor(IAnalyzer& analyzer) : m_analyzer{analyzer} {}

    void predict(const std::vector<std::vector<CardValue_52_t>>& playerHands) {
#ifdef DEBUG
        auto tstart = std::chrono::high_resolution_clock::now();
#endif

        auto cards = getAvailableCards(playerHands);
        auto combinations = CombinationCalculator::calculate(cards, 7 - playerHands[0].size());
        std::vector<unsigned> wins(playerHands.size(), 0);
        // std::vector<unsigned> ties(playerHands.size(), 0);

        for (auto& combination : combinations) {
            auto winner = comparePlayerHandsForCombination(playerHands, combination);
            if (winner >= 0)
                wins[winner] += 1;
        }

        for (auto p = 0; p < playerHands.size(); ++p) {
            std::cout << "player "<< p << ": " << 100. * wins[p] / combinations.size() << "%" << std::endl;
        }

#ifdef DEBUG
        auto tend = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(tend - tstart);
        std::cout << "combination analysis takes " << duration.count() << " ms\n";
#endif
    }

private:
    std::vector<CardValue_52_t> getAvailableCards(const std::vector<std::vector<CardValue_52_t>>& players) {
        std::vector<bool> deck(52, true);
        for (auto& player : players)
            for (auto card : player)
                deck[card] = false;

        std::vector<CardValue_52_t> cards;
        for (auto c = 0; c < deck.size(); ++c)
            if (deck[c])
                cards.push_back(c);
        
        return cards;
    }

    int comparePlayerHandsForCombination(const std::vector<std::vector<CardValue_52_t>>& players, 
                                          std::vector<CardValue_52_t>& combination) {
        std::unique_ptr<Hand> winningHand = std::make_unique<HighCard>(std::vector<CardValue_13_t>{5, 3, 2, 1, 0});
        std::vector<unsigned> winners;

        for (auto p = 0; p < players.size(); ++p) {
            auto& player = players[p];

            for (auto card : player)
                combination.push_back(card);

            auto hand = m_analyzer.analyze(combination);

            for (auto c = 0; c < player.size(); ++c)
                combination.pop_back();
            
            if (*winningHand < *hand) {
                winningHand = std::move(hand);
                winners.clear();
                winners.push_back(p);
            } else if (*winningHand == *hand) {
                winners.push_back(p);
            }
        }

        if (winners.size() == 1)
            return winners[0];
        return -1;
    }

    IAnalyzer&  m_analyzer;
};

*/