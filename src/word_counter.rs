use std::collections::HashMap;

use get_user_input;
use play_again;

pub fn word_counter_main() {
    println!("Write a text and get information how often you used each word.");
    println!("Please enter your text: ");
    let input = get_user_input();
    let mut map = HashMap::new();

    for word in input.trim().split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("You yoused the words this much: \n\
             {:?}", map);

    play_again();
}
