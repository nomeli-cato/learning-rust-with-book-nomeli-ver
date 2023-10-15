use adder;

// cargo test --test integration_test
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

mod common;

#[test]
fn it_adds_two_v2() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}