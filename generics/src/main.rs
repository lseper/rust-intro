use std::fmt::Display;

fn main() {
    // runs accordingly,
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // breaks, as the lifetime of returned parameter is shorter than its needed
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz"); // here the lifetime starts
    //     result = longest(string1.as_str(), string2.as_str()); // here it ends
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static is a reserved word for specifying lifetimes that exist for an entire program
    // think of it as using var in javascript, explicitly casting the scope to the outermost scope
    let s: &'static str = "I have a static lifetime.";
}

// here is something that implements generics, trait bounds, and lifetimes
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// structs can also have lifetime parameters
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // here the lifetime of self is returned implicitly as a lifetime parameter
    fn level(&self) -> i32 {
        3
    }
    // here the lifetime of self is returned implicitly as a lifetime parameter
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// lifetime parameters
// basically specifies that the lifetime of the return value is one of the two parameters passed in
// therefore, it will check and compile only if any ordering of lifetimes is valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// implicit lifetime rules:
// 1. each input gets its own unique lifetime (e.g fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str)
// 2. if there is one input parameter, then the output lifetime is set to the input lifetime (e.g fn first_word<'a>(sentence: &'a str) -> &'a str)
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self, then the lifetime of self is assigned to all output lifetime parameters
