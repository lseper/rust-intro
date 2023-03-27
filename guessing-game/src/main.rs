use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number (1 - 10) inclusive!");

    let generated_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        match guess.cmp(&generated_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too big!"),
            Ordering::Equal => {
                println!("{guess} is perfect!");
                break;
            }
        };
    }

    println!("Thanks for playing!")
}
