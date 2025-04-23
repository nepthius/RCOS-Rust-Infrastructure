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

    /*
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
    */

    println!("========================");
    println!("   WELCOME TO RUSTPOKER!!!!!!");
    println!("========================");

    //bassic small/big blind setup
    let mut game = Game::new(8, 1000, 10, 20);
    game.print_game_info();

    //adding players
    let mut player_count = 0;
    while player_count < 2 {
        print!("Enter a username for player {}: ", player_count + 1);
        io::stdout().flush().unwrap();

        let mut username = String::new();
        stdin().read_line(&mut username).expect("Bad Read");
        let username = username.trim().to_string();

        if username.is_empty() {
            println!("Username cannot be empty.");
            continue;
        }
        game.add_player(username);
        player_count += 1;
    }

    println!("The avengers have assembled");
    println!("--------------------------");

    //going through game
    let total_rounds = 3;
    for round_number in 1..=total_rounds {
        println!("\n Starting Round {round_number}");
        game.round();

        println!("\n Round {round_number} Complete");
        println!("Press ENTER to continue...");
        let _ = stdin().read_line(&mut String::new());
    }

    //final stats
    println!("\n===============================");
    println!("        Final Game Stats for you!");
    println!("===============================");
    for player in &game.players {
        println!("Player: {}", player.get_username());
        println!("Stack: {}", player.get_stack());
        let stats = player.get_stats();
        println!("Hands Played: {}", stats.get_hands_played());
        println!("Wins: {}", stats.get_wins());
        println!("Losses: {}", stats.get_losses());
        println!("Net Chips: {}", stats.net_chips());
        println!("Win Rate: {:.2}%", stats.win_rate() * 100.0);
        println!("-------------------------------");
    }

    println!("Th-th-th-that's all folks!");
}

}

