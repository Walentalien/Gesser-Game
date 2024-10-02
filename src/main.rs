use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::env;

fn main() {
    // Create iterator to collect command-line arguments
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() != 2 {
        println!("Usage: guess_game <number_of_attempts>");
        return;
    }

    // Parse the number of attempts from the command-line argument
    let max_attempts: u32 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for the maximum attempts.");
            return;
        }
    };

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses: u32 = 0;

    loop {
        // Check if the player has exceeded the allowed attempts
        if number_of_guesses >= max_attempts {
            println!("You've exceeded the maximum number of attempts. You lose!");
            println!("The secret number was: {}", secret_number);
            break;
        }

        println!("Please input your guess. Attempt {}/{}", number_of_guesses + 1, max_attempts);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        number_of_guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if number_of_guesses == 1 {
                    println!("Wow, you guessed it on your first try! What luck!");
                } else {
                    println!("It took you {} guesses.", number_of_guesses);
                }
                break;
            }
        }
    }
}
