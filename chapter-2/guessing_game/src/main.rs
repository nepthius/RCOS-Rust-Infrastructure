use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a word!");
    
    

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut guess = String::new();

        println!("Please enter the word:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure of reading");

        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner, winner chicken dinner");
                break;
            }
        };

    }
     
}