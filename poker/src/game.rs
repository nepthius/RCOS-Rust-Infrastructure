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

    fn handle_bets(players: &mut Vec<Player>, round_players: &mut HashSet<String>, min_bid: &mut i32){
        for player in players{

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
            if bid < *min_bid || bid > *player.get_stack(){
                print!("Your bid was either too small for this round or too large for your stack");
                round_players.remove(player.get_username());
                continue;

            }
            player.sub(bid);
            *min_bid = max(bid, *min_bid);

            //print!("Current stack: {}", player.get_stack());
            io::stdout().flush().unwrap();

        }
    }

    pub fn round(&mut self) {
        let mut deck = Deck::create();
        deck.shuffle();
        //each round go through each and see buy in
        //go through each player and prompt and update buy in
        let mut min_bid = self.big;
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
        Game::handle_bets(&mut self.players, &mut round_players, &mut min_bid);
        min_bid = self.big; 

        let mut board = Vec::new();

        for x in 0..3{
            board.push(deck.get_card().to_string());
        }
        println!("Board: {:?}", board);//replace with better board ui

        //flop betting
        Game::handle_bets(&mut self.players, &mut round_players, &mut min_bid);
        min_bid = self.big; 

        board.push(deck.get_card().to_string());
        println!("Board: {:?}", board);//replace with better board ui

        //the turn betting
        Game::handle_bets(&mut self.players, &mut round_players, &mut min_bid);
        min_bid = self.big;
        board.push(deck.get_card().to_string());
        println!("Board: {:?}", board);//replace with better board ui

        //river betting
        Game::handle_bets(&mut self.players, &mut round_players, &mut min_bid);

        
        
        
        //royal flush, 
        //
    }

    pub fn calculate_best_hand(player: &Player, board: &Vec<String>) -> (String, Vec<String>) {
        let mut all_cards = player.view_cards().clone();
        all_cards.extend(board.clone()); 
    
        let mut best_rank = 11;
        let mut best_hand = Vec::new();
        let mut best_label = String::new();
    
        for combo in all_cards.iter().combinations(5) {
            let hand = combo.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
            let (rank, label) = rank_hand(&hand);
    
            if rank < best_rank {
                best_rank = rank;
                best_label = label;
                best_hand = hand;
            }
        }
    
        println!("Best hand for {}: {} => {:?}", player.get_username(), best_label, best_hand);
        (best_label, best_hand)
    }

    fn rank_hand(hand: &Vec<String>) -> (u8, String) {
        let mut values = Vec::new();
        let mut suits = Vec::new();
        

        
        for card in hand {
            //print!("{}", card);
            let (suit, value) = card.split_at(1);
            suits.push(suit);
            values.push(value.parse::<u8>().unwrap());
        }
    
        values.sort_unstable();
        let mut counts = HashMap::new();
        for v in &values {
            *counts.entry(v).or_insert(0) += 1;
        }
    
        let is_flush = suits.iter().all(|&s| s == suits[0]);
        let is_straight = values.windows(2).all(|w| w[1] == w[0] + 1);
    
        let mut freq = counts.values().cloned().collect::<Vec<u8>>();
        freq.sort_unstable_by(|a, b| b.cmp(a));
        match (is_flush, is_straight, freq.as_slice()) {
            (true, true, _) if values.contains(&13) && values.contains(&1) => (1, "Royal Flush".to_string()),
            (true, true, _) => (2, "Straight Flush".to_string()),
            (_, _, [4, 1]) => (3, "Four of a Kind".to_string()),
            (_, _, [3, 2]) => (4, "Full House".to_string()),
            (true, _, _) => (5, "Flush".to_string()),
            (_, true, _) => (6, "Straight".to_string()),
            (_, _, [3, 1, 1]) => (7, "Three of a Kind".to_string()),
            (_, _, [2, 2, 1]) => (8, "Two Pair".to_string()),
            (_, _, [2, 1, 1, 1]) => (9, "One Pair".to_string()),
            _ => (10, "High Card".to_string()),
        }
    }
    

}

