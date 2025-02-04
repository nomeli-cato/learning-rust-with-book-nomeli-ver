pub fn add(left: usize, right: usize) -> usize {
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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Adding Custom Failure Messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// Checking for Panics with should_panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


// --snip--
pub struct Guess_v2 {
    value: i32,
}


impl Guess_v2 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }


    // Testing Rectangle implementations
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

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_v2() {
        let result = greeting("Carol");

        // default meesage if is false and the result
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Checking for Panics with should_panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // v2
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_v2() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests
    #[test]
    fn it_works_with_generics() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
    // To assert that an operation returns an Err variant, don’t use the question 
    // mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
}
