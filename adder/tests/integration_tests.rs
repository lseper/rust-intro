use adder;

mod common;

#[test]
fn it_adds_two_integration() {
    // can call setup code for tests here
    common::setup();
    assert_eq!(4, adder::add_two(2, 2));
}