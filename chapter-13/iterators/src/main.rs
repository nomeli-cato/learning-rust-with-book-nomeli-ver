

// The Iterator Trait and the next Method
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}


fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    
    println!("{:?}", v1_iter);
    
    for val in v1_iter {
        println!("Got: {}", val);
    }
    
    // Methods that Consume the Iterator
    let v1_iter_2 = v1.iter();
    // consuming adaptors
    let total: i32 = v1_iter_2.sum();

    assert_eq!(total, 6);

    // Methods that Produce Other Iterators
    // need to be consumned
    // v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // Using Closures that Capture Their Environment
    
}
