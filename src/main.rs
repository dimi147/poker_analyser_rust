mod hand;
mod deck;
mod analyser;

use hand::Hand;
use deck::Deck;
use analyser::analyse;
// mod highcard;
// mod pair;
// mod deck;
// mod analyzer;
// use highcard::HighCard;
// use pair::Pair;

fn test() {
    let a = Hand::pairs(vec![10], vec![1,4,5]);
    let b = Hand::pairs(vec![0, 1], vec![3]);
    assert!(a < b);

    let a = Hand::fullhouse(8, 10);
    let b = Hand::fullhouse(10, 8);
    assert!(a < b);
}

fn main() {

    test();

    println!("{:?}", analyse(0b1111111 as Deck));



}
