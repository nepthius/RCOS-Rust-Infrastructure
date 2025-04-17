mod deck;

use deck::Deck;

fn main() {
    println!("Hello, world!");
    
    let mut deck = Deck::create();
    println!("{:?}", deck.get_deck());
    deck.shuffle();
    println!("{:?}", deck.get_deck());
    println!("{}", deck.get_card());
    println!("{:?}", deck.get_deck());
}

