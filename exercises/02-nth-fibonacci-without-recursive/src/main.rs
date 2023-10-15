fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    
    let mut prev = 0;
    let mut current = 1;
    
    for _ in 1..n {
        let next = prev + current; // 1 2 3 5 8
        prev = current; // 1 1 2 3 5
        current = next; // 1 2 3 5 8
    }
    
    return current;
}

fn main() {
    let n = 10; // Replace with the desired value of n
    let result = fibonacci_iterative(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
