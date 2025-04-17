pub struct Player{ 
    //each player has a stack
    stack: i32,
    cards:Vec<String>,
}

impl Player{
    pub fn new(val:i32) -> Self{
        let mut stack = val;
        let mut cards = Vec::new();
        Player {stack, cards}
    }

    pub fn get_stack(&self) -> &i32{
        return &self.stack
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