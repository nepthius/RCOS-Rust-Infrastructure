#[derive(Clone, Hash, Eq, PartialEq)] //allows deep copies of players and hashing (eq means equality)
pub struct Player{ 
    //each player has a stack
    stack: i32,
    cards:Vec<String>,
    username: String,
}

impl Player{
    pub fn new(val:i32, username:String) -> Self{
        let mut stack = val;
        let mut cards = Vec::new();
        Player {stack, cards, username}
    }

    pub fn get_stack(&self) -> &i32{
        return &self.stack
    }

    pub fn get_username(&self) -> &String{
        return &self.username
    }

    pub fn view_cards(&self) -> &Vec<String>{
        return &self.cards
    }

    pub fn add(&mut self, val:i32) {
        self.stack += val;
    }

    pub fn sub(&mut self, val:i32){
        self.stack -= val;
    }

    pub fn dealt(&mut self, card1:String, card2:String) {
        self.cards = Vec::new();
        self.cards.push(card1);
        self.cards.push(card2);
    }
}