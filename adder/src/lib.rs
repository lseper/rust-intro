/* There are multiple ways you can run 'cargo test'

    In general, for viewing options, cargo test --help shows you options to run with cargo test
    cargo test -- --help shows you various extra flags you can run after the separator (--)

    ---- For configuring how tests are ran ----
    For running tests completely default, cargo test

    For running tests one at a time (non-concurrent) run cargo test -- --test-threads=1
        This is useful if your tests rely on shared environment data, write to the same file, etc
        And they don't run concurrently in the program, so testing them concurrently doesn't make
        sense (the default is concurrent test running)

    To see output in functions (such as println!) calls, run as cargo test -- --show-output

    ---- For seeing which tests are ran ----

    run cargo test [test_name] OR [partial_test_name]
        runs all tests that either match the full name, or part of the name
    
    For ignoring a test by default, add the #[ignore] annotation to it
        useful for expensive tests that may take a long time to run and should only ran when specified
        To only run ignored tests, run cargo test -- --ignore
        To run all tests, including ignored ones, run cargo test -- --include-ignored

    For only running tests in a specific file, run cargo test -- --test [filename]
 */

pub fn add_two(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// this annotation only compiles this code when running cargo test, and excludes it in cargo build/run
#[cfg(test)]
mod tests {
    use super::*;

    // this test fails
    // #[test]
    // fn panics() {
    //     panic!("This test should fail as it panics")
    // }

    // assert equality
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2, 2));
    }

    // testing with Structs
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // can also assert something is not true
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // allows custom test pass/failure messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // need to add the annotation should_panic for testing things that raise a panic
    // can add a check that it panics in the CORRECT WAY, which is what expected = "msg"
    // is, it asserts that we are hitting hte > 100 panic statement, not hte < 1
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // can also use Result<T, E> for writing tests as well
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
