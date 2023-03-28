mod stats;
use stats::{median, mode};

use std::io;

fn main() {
    let mut arr = receive_arr_from_user();
    let median_value = median(&mut arr).expect("Vector should not be empty! :(");
    let mode_value = mode(&mut arr);
    println!("Median value: {}\nMode Value: {}", median_value, mode_value);
    println!("{:?}", arr);
}

fn receive_arr_from_user() -> Vec<i32> {
    println!("Please input values for an array. When you are finished, enter any non integer value into the console.");
    let mut my_arr: Vec<i32> = Vec::new();
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input ;~;");
        let value: Option<i32> = match user_input.trim().parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        };
        if value != None {
            my_arr.push(value.unwrap_or(0));
            println!("Successfully input number!");
        } else {
            break;
        }
    }
    return my_arr;
}
