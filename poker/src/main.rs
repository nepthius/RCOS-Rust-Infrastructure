mod deck;

use deck::Deck;

mod game;
use game::Game;

mod player;
use player::Player;

use std::io::stdin;
use std::io::{self, Write};


fn main() {
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

    print!("Enter a username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    stdin().read_line(&mut username).expect("Bad Read");
    username = username.trim().to_string();
    println!("Welcome {username}");
    println!("------------------");
    game.add_player(username);
    game.round();
    

}

