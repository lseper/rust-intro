
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKindString {
    // can define different data for each with no problem
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // basic enum
    let four = IpAddrKind::V4;

    let sophisticated = IpAddrKindString::V4(127, 0, 0, 1);
    let loopback = IpAddrKindString::V6(String::from("::1"));

    // could be None
    let trouble: Option<i8> = Option::Some(16);
    // couldn't be None
    let ok: i8 = 4;
    // is None
    let absent: Option<i8> = Option::None;
    // causes an error as it doesn't know how to + Option<i8> with i8
    // println!("{}", trouble + ok);

    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }


}

// pattern matching
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // exhaustive
    }
}

