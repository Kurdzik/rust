use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);

    println!("\nGuess the number!");
    println!("Enter 'quit' to exit");
    println!("=================");   

    loop {
        println!("\n\nPlease input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().to_lowercase().contains("quit") {
                    break;
                } else {
                    println!("Input must be a number");
                    continue;
                }
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
