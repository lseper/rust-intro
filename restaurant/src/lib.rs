mod front_of_house {
    // public module
    pub mod hosting {
        // public function within the module
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    // making a struct public does NOT make all of its fields implicitly pub
    // you have to specify it explicitly, like in pub toast
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // refers to parent here, so you can call it
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

mod customer {

    use super::hosting;
    pub fn eat_at_restaurant() {
        // this errors, as "use" doesn't apply to the scope within the customer module!
        // to fix, move use crate::front_of_house::hosting into customer
        // OR reference the shortcut in the parent module with use super::hosting within the child customer module, which is done above
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // shortened due to bringing hosting into scope with use
    hosting::add_to_waitlist();

    // All fields of enum are public when specified enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
