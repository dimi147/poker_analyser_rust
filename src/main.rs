mod hand;
mod deck;
mod analyser;
mod predictor;

use std::io::Write;

fn main() {
    print!("Enter number of opponents: ");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("{}", buf);
    let opponents: u8 = buf.trim().parse().unwrap();
    print!("Enter your cards: ");
    std::io::stdout().flush().unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut it = buf.split(' ');
    let deck: deck::Deck = deck::to_deck(&[it.next().unwrap(), it.next().unwrap()]);
    let mut players = vec![deck];
    
    for _ in 0..opponents {
        buf.clear();
        print!("Enter opponent's cards: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut it = buf.split(' ');
        let deck = deck::to_deck(&[it.next().unwrap(), it.next().unwrap()]);
        players.push(deck);
    }

    let odds = predictor::predict(&players);
    println!("Winning odds {:.2}", odds[0] * 100f32);
    println!("Opponent odds {:.2}", odds[1..].iter().sum::<f32>() * 100f32);
    println!("Tie odds {:.2}", 100f32 - odds.iter().sum::<f32>() * 100f32);
}

