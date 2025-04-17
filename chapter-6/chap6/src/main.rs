fn main() {
    enum Coin{
        Penny,
        Nickel,
        Quarter,
    }

    print!("{}", value_in_cents(Coin::Penny));

    fn value_in_cents(coin:Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Quarter => 25
        }
    }

}
