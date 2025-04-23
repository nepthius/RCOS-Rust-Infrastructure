pub struct Stats {
    hands_played: u32,
    wins: u32,
    losses: u32,
    chips_won: i32,
    chips_lost: i32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            hands_played: 0,
            wins: 0,
            losses: 0,
            chips_won: 0,
            chips_lost: 0,
        }
    }

    pub fn record_hand(&mut self, won: bool, chips_change: i32) {
        self.hands_played += 1;
        if won {
            self.wins += 1;
            self.chips_won += chips_change;
        } else {
            self.losses += 1;
            self.chips_lost += chips_change.abs();
        }
    }

    pub fn win_rate(&self) -> f32 {
        if self.hands_played == 0 {
            0.0
        } else {
            self.wins as f32 / self.hands_played as f32
        }
    }

    pub fn net_chips(&self) -> i32 {
        self.chips_won - self.chips_lost
    }

    pub fn get_hands_played(&self) -> u32 {
        self.hands_played
    }

    pub fn get_wins(&self) -> u32 {
        self.wins
    }

    pub fn get_losses(&self) -> u32 {
        self.losses
    }

    pub fn get_chips_won(&self) -> i32 {
        self.chips_won
    }

    pub fn get_chips_lost(&self) -> i32 {
        self.chips_lost
    }
}
