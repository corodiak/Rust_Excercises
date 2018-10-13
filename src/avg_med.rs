use std::str::FromStr;

use play_again;
use get_user_input;

pub fn avg_med_main() {
    println!("This function returns the average and the median value of a given signed integer sequence");
    println!("Please input your sequence:");
    let input = get_user_input();
    let mut input_vec: Vec<i32> = parse_input(input);
    let average = calculate_average(&input_vec);
    let median = calculate_median(  &mut input_vec);

    println!("The average of the input sequence is: {}", average);
    match median.1 {
        Some(x) => println!("The median of the sorted sequence {:?} is: {}", median.0, x),
        None => println!("Couldn't find a median"),
    }
    play_again();
}

/// Parse a string to a vector with i32 values
fn parse_input(input: String) -> Vec<i32> {
    let input= input.trim();
    if input.is_empty() {
        println!("Please enter a valid input.");
        println!();
        avg_med_main();
    }

    for c in input.chars() {
        if !c.is_numeric() && c != '-' && c != ',' && c != ' '{
            println!("Please only enter positive or negative integers separated by ','.");
            println!();
            avg_med_main();
        }
    }

    // Cast the string to a vector
    let mut split_as_vector = input.split(",").collect::<Vec<_>>();
    // Iterate through the vector and trim each entry
    for i in &mut split_as_vector {
        *i = i.trim();
    }
    // Parse each vector entry to a i32
    split_as_vector.iter().map(|s| i32::from_str(s).expect("failed to parse number"))
        .collect::<Vec<_>>()
}

/// Calculates the average of the values of a given vector
fn calculate_average(vector: &Vec<i32>) -> i32 {
    let mut len = 0;
    let mut sum = 0;
    for i in vector {
        sum += i;
        len += 1;
    }
    sum / len
}

/// Sorts a vector and returns a tuple containing the sorted vector and its median
fn calculate_median(vector: &mut Vec<i32>) -> (Vec<i32>, Option<&i32>) {
    vector.sort();
    (vector.to_vec(), vector.get(vector.len()/2))
}