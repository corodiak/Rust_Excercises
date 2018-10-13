use get_user_input;
use play_again;

pub fn pig_latin_main() {
    println!("This function translates a given text into pig latin.");
    println!("Please enter your text: ");
    let input = get_user_input();
    let translation = translate(input);

    println!("Your translated text: {}", translation);
    play_again()
}

fn translate(input: String) -> String {
    let mut output = String::new();
    for word in input.trim().split_whitespace() {
        let first_char = word.chars().next().unwrap();

        match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let s = word.to_string() + "-hay";
                output.push_str(&s);
            }
            _ => {
                let mut s = word.to_string();
                let consonant = s.remove(0);
                s.push('-');
                s.push(consonant);
                s.push_str("ay");
                output.push_str(&s);
            }
        }
        output.push_str(" ");
    }
    output
}
