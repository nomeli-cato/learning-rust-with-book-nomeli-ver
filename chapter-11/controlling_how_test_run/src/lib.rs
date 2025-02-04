fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// Running a Subset of Tests by Name
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // Running a Subset of Tests by Name
    // all this test run in paralle if run cargo test
    // to run a one --> cargo test one_hundred
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    // you can --> cargo test add
    // and cargo choose all the test begin with add
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }


    // Ignoring Some Tests Unless Specifically Requested
    #[test]
    fn it_works() {
    assert_eq!(2 + 2, 4);
    }

    // To run ignore test:
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}