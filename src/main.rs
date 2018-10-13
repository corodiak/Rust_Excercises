use std::io;
mod first_word;
mod guessing_game;
mod structs;
mod avg_med;
mod word_counter;
mod pig_latin;

static mut SELECTION: u32 = 99;

fn main() {
    println!("----------------------------------");
    println!("Select one of the following functions:");
    print_mode_overview();

    let input = get_user_input();
    unsafe {
        SELECTION = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 99,
        };
    }
    select_mode();
}

fn print_mode_overview() {
    println!("0: Exit program.");
    println!("1: First Word");
    println!("2: Guessing Game");
    println!("3: Structs");
    println!("4: Average and Median");
    println!("5: Word counter");
    println!("6: Pig Latin");
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn select_mode() {
    println!();
    unsafe {
        match SELECTION {
            0 => return,
            1 => first_word::first_word_main(),
            2 => guessing_game::guessing_main(),
            3 => structs::structs_main(),
            4 => avg_med::avg_med_main(),
            5 => word_counter::word_counter_main(),
            6 => pig_latin::pig_latin_main(),
            _ => {
                println!("Please select a valid mode.");
                println!();
                main();
            }
        }
    }
}

fn play_again() {
    println!("Do you want to play again? (y/n)");
    let answer = get_user_input();

    match answer.trim() {
        "y" => select_mode(),
        "n" => main(),
        _ => play_again(),
    }
}

