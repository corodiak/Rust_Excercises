use play_again;
use get_user_input;

pub fn first_word_main() {
    println!("This function returns the first word of a given sentence.");
    println!("Write a sentence: ");

    let sentence = get_user_input();

    let word = first_word(&sentence);

    println!();
    println!("The first word is: {}", word);
    play_again();
}

fn first_word(sentence: &String) -> &str {
    let sentence = sentence.trim();

    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..i];
        }
    }
    &sentence[..]
}

