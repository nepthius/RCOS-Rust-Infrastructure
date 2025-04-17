mod deck;

use deck::Deck;

mod game;
use game::Game;

mod player;
use player::Player;


fn main() {
    println!("Hello, world!");
    
    //deck testing
    /*
    let mut deck = Deck::create();
    println!("{:?}", deck.get_deck());
    deck.shuffle();
    println!("{:?}", deck.get_deck());
    println!("{}", deck.get_card());
    println!("{:?}", deck.get_deck());
    */

    let mut game = Game::new(8, 1000, 10, 20);
    game.print_game_info();
    

}

