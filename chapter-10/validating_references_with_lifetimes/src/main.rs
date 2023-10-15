fn main() {
    println!("Validating references with Lifetimes");
    {
        // Preventing Dangling References with Lifetimes
        // let r;

        // {
        //     let x = 5;
        //     r = &x;
        //     // `x` does not live long enough
        // }

        // println!("r: {}", r);
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        // let result = longest(string1.as_str(), string2);
        // println!("The longest string is {}", result);

        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }
    }

    {
            // Lifetime Annotation Syntax
        // &i32        // a reference
        // &'a i32     // a reference with an explicit lifetime
        // &'a mut i32 // a mutable reference with an explicit lifetime
    }

    {
        // Lifetime Annotations in Function Signatures
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("long string is long");
        // let result;
        {
            let string2 = String::from("xyz");
            // `string2` does not live long enough
            // result = longest(string1.as_str(), string2.as_str());
        }
        // println!("The longest string is {}", result);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        // Lifetime Elision
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
        
            &s[..]
        }
        
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }
        
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
        
    }

    {
        // Generic Type Parameters, Trait Bounds, and Lifetimes Together
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
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
    }
}
