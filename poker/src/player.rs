pub struct Player{ 
    //each player has a stack
    let mut stack,
}

impl Player{
    pub fn new(val:i32) -> Self{
        let mut stack = val;
        Player {stack}
    }

    pub fn get_stack(&self) -> i32{
        return &stack
    }

    pub fn add(&mut self, val:i32) {
        self.stack += val;
    }

    pub fn sub(&mut self, val:i32){
        self.stack -= val;
    }
}