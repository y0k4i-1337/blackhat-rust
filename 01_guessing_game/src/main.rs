use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //  generate random number from interval [1, 100]
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // infinite loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // because of .expect, it will crash on error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing variable guess with type annotation (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare guess and secret_number and handle each possible return
        match guess.cmp(&secret_number) {
            Ordering::Equal =>  {
                println!("Yo win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
