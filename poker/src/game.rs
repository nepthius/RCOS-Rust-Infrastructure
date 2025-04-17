use crate::deck::Deck;
use crate::player::Player;

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

    pub fn add_player(&mut self, ){
        let mut p = Player::new(self.buy_in);
        self.players.push(p);
    }

    pub fn round(&self) {
        let mut deck = Deck::create();
        deck.shuffle();
        //each round go through each and see buy in
        //go through each player and prompt and update buy in
    }

}

