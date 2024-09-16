use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    let mut number_of_guesses: u32 = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            // ` --list```cargo --list`
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        number_of_guesses +=1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                match number_of_guesses.cmp(&&1) {
                    Ordering::Less => break,
                    Ordering::Equal => println!("Wow you guessed with first try!!!\nWhat a luck!"),
                    Ordering::Greater => println!("It took you {} guesses", number_of_guesses),
                }

                break;
            }
        }
    }
}
