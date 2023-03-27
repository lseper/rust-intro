fn main() {
    // immutable - no properties of user1 can be changed (e.g user1.active = false would be invalid)
    let user1 = User {
        active: true,
        username: String::from("zaverose"),
        email: String::from("zaverosearts@gmail.com"),
        sign_in_count: 1
    };
    // user1.active = false; - invalid, as user1 is not mut

    let mut user2 = User {
        active: true,
        username: String::from("lseper"),
        email: String::from("seperliam@gmail.com"),
        sign_in_count: 1
    };

    // object destructure syntax
    let user3 = User {
        email: String::from("email8428@gmail.com"),
        ..user2
    };

    print_user(&user3);
    
    user2.active = false;
    // user2.username = String::from("liamseper");
    
    // changed in user2
    print_user(&user2);
    // this still runs though!
    // println!("{}", user2.active);
    // doesn't change in user3
    print_user(&user3);

    // user1.username has been moved into user4.username
    let user4 = User {
        email: String::from("deer@hacker.com"),
        ..user1
    };
    // errors, as user1.username is now owned by user4
    // active and sign_ins are copied, but since String is not Copy, it is not, so it stays
    // print_user(&user1);

    // these are two different things, a function accepting a Point would not accept a Color, even though
    // they're the same
    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);

    // unit-like struct
    let oracle = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    // shorthand for assigning fields of same name
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn print_user(user: &User) {
    println!("User: {}", user.username);
    println!("Email: {}", user.email);
    println!("Active: {}", user.active);
    println!("Sign-ins: {}", user.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple like structs, data instantiated from this would be different even though they're the same type
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like Struct
struct AlwaysEqual;