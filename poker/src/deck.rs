use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<String>,
}

impl Deck {

    pub fn create() -> Self {
        //Add array of suits
        let mut suits = Vec::new();
        suits.push("C");
        suits.push("H");
        suits.push("D");
        suits.push("S");

        let mut cards = Vec::new();


        for x in 1..14 {
            for suit in &suits{
                cards.push(format!("{}{}", suit,&x.to_string()));
            }
        }

        Deck { cards }
    }

    pub fn get_deck(&self) -> &Vec<String> {
        return &self.cards
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng(); //just get like a random num
        self.cards.shuffle(&mut rng);
    }

    pub fn get_card(&mut self) -> String{
        return self.cards.pop().expect("Empty")
    }
}