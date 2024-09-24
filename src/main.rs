mod hand;
mod deck;
mod analyser;
mod predictor;

fn main() {

    let mut deck = 0;
    
    let mut i = 0;
    while deck != 0xfffffffffffff {
        deck += deck::get_random_card(deck);
        println!("{}", i);
        i += 1;
    }

}

