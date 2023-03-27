use std::io;

fn main() {
   let s = String::from("hello");
    fn ownership_exp(string: String) {
        println!("function string: {}", string);
    }

    let new_s = String::from("a different string");

    ownership_exp(new_s);

    println!("String after function: {}", s);
}
