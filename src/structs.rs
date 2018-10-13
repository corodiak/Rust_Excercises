use std::str::FromStr;

use play_again;
use get_user_input;

pub fn structs_main() {
    println!("Here you can define 3 rectangles and then get info about their area and if they fit into each other.");
    println!("Please enter positive numbers separated by ',' and without spaces");

    println!("Enter width and height of the first rectangle:");
    let rect1_data = get_rect_data(get_user_input());
    let rect1 = Rectangle::build_rectangle(rect1_data);
//    println!("RECT: {:#?}", rect1);

    println!("Enter width and height of the second rectangle:");
    let rect2_data = get_rect_data(get_user_input());
    let rect2 = Rectangle::build_rectangle(rect2_data);
//    println!("RECT: {:#?}", rect1);

    println!("Enter width and height of the third rectangle:");
    let rect3_data = get_rect_data(get_user_input());
    let rect3 = Rectangle::build_rectangle(rect3_data);
//    println!("RECT: {:#?}", rect1);

    println!();
    println!("The area of rectangle 1 is: {}\n\
              The area of rectangle 2 is: {}\n\
              The area of rectangle 3 is: {}", rect1.area(), rect2.area(), rect3.area());

    println!();
    println!("Can rectangle 1 hold rectangle 2? -> {}\n\
              Can rectangle 1 hold rectangle 3? -> {}", rect1.can_hold(&rect2), rect1.can_hold(&rect3));

    play_again();
}

fn get_rect_data(data_string: String) -> (u32, u32) {
    let data_string = data_string.trim();

    if data_string.is_empty() {
        println!("Please enter a valid input.");
        println!();
        structs_main();
    }

    for c in data_string.chars() {
        if !c.is_numeric() && c != ',' && c != ' ' {
            println!("Please only enter positive integers seperated by ','.");
            println!();
            structs_main();
        }
    }

    let mut split_as_vector = data_string.split(",").collect::<Vec<_>>();
    for i in &mut split_as_vector {
        *i = i.trim();
    }

    let vector = split_as_vector.iter().map(|s| u32::from_str(s).expect("failed to parse number"))
        .collect::<Vec<_>>();

    match split_as_vector.len() {
        1 => (vector[0],vector[0]),
        2 => (vector[0], vector[1]),
        _ => {
            println!("Too many parameters, only the first two will be used.");
            (vector[0], vector[1])
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_rectangle(data: (u32, u32)) -> Rectangle {
        if data.0 == data.1 {
            Rectangle::square(data.0)
        } else {
            Rectangle { width: data.0, height: data.1 }
        }
    }

    fn square(size: u32) -> Rectangle{
        Rectangle {width: size, height: size}
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}