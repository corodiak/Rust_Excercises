extern crate rand;

use std::cmp::Ordering;
use self::rand::Rng;
use play_again;
use get_user_input;

pub fn guessing_main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    let mut number_of_tries = 1;

//    println!("The secret number is: {}", secret_number);

    loop {
        println!();
        println!("Please input your guess.");

        let guess = get_user_input();


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                number_of_tries += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                number_of_tries += 1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("You needed {} tries.", number_of_tries);
                break;
            }
        }
    }
    play_again();
}
