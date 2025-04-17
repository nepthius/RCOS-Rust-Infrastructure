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
}