mod deck;

use deck::Deck;

fn main() {
    println!("Hello, world!");
    
    let deck = Deck::create();
    println!("{:?}", deck.get_deck());
    
}

