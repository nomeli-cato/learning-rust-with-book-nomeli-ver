use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // if user choose None or_else use a closures without arguments
        // and execute most stocked()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closures Type inference and Annotation
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;


    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // error because expect string, for the lock type of the first use
    // let n = example_closure(5);

    // Capturing References or Moving Ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("\n");
    let mut mut_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", mut_list);
    
    let mut borrows_mutably = || mut_list.push(7);
    
    borrows_mutably();
    println!("After calling closure: {:?}", mut_list);
    
    println!("\n");

    let move_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", move_list);

    thread::spawn(move || println!("From thread: {:?}", move_list))
        .join()
        .unwrap();

    // println!("{:?}",move_list); give error

    // Moving Captured Values Out of Closures and the Fn Traits
    // example
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // Example of FnMut
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list_of_rectangles = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list_of_rectangles.sort_by_key(|r| r.width);
    println!("{:#?}", list_of_rectangles);


    // Error
    // only can mutate |r|, any variable outside of closure is not allowed to change
    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // // This closure can be called once; 
    // // trying to call it a second time wouldn’t work because value would no longer be in the environment 
    // // Therefore, this closure only implements FnOnce
    // // Error showed:
    // // move occurs because `value` has type `String`, which does not implement the `Copy` trait
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    // Solve
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
