use crate::deck::Deck;
use crate::player::Player;
use std::io::stdin;
use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Write};

pub struct Game{
    //players
    //dealing each round
    //stacks associated with each player
    buy_in: i32,
    no_players: i32,
    players: Vec<Player>,
    small: i32,
    big: i32,
}

impl Game{
    pub fn new(no_players:i32, buy_in:i32, small:i32, big:i32) -> Self{
        let mut no_players = no_players;
        if (no_players < 2){
            no_players = 2;
        }
        let mut players = Vec::new();

        Game {no_players, players,buy_in, small, big}
    }

    pub fn print_game_info(&self) {
        println!("Player #: {} | Buy in {} | Small {} | Big {}", self.no_players, self.buy_in, self.small, self.big);
    }

    pub fn add_player(&mut self, username:String){
        let mut p = Player::new(self.buy_in, username);
        self.players.push(p);
    }

    pub fn round(&mut self) {
        let mut deck = Deck::create();
        deck.shuffle();
        //each round go through each and see buy in
        //go through each player and prompt and update buy in
        let mut min_bid = self.small;
        let mut round_players =  HashSet::<String>::new();

        for player in &self.players{
            round_players.insert(player.get_username().to_string());
        }
        //preflop

        for player in &mut self.players{
            player.dealt(deck.get_card().to_string(), deck.get_card().to_string());
            let cards = player.view_cards();
            print!("Your cards: {} and {}\n",cards[0], cards[1]);
            io::stdout().flush().unwrap();
        }

        
        //preflop betting
        for player in &mut self.players{

            if !round_players.contains(player.get_username()){
                continue
            }
            print!("Please enter your bet amount (stack {} | round bid {min_bid}): ", player.get_stack());
            io::stdout().flush().unwrap();
            let mut size = String::new();
            stdin().read_line(&mut size).expect("Ruh roh read");
            size = size.trim().to_string();
            let mut bid;
            bid = size.parse::<i32>().unwrap();
            if bid < min_bid || bid > *player.get_stack(){
                print!("Your bid was either too small for this round or too large for your stack");
                round_players.remove(player.get_username());
                continue;

            }
            player.sub(bid);
            min_bid = max(bid, min_bid);

            //print!("Current stack: {}", player.get_stack());
            io::stdout().flush().unwrap();

        }

        
        
        //royal flush, 
        //
    }

    pub fn calculate_best_hand(&self) {
        //priority (hand rank), card rank
    }

}

