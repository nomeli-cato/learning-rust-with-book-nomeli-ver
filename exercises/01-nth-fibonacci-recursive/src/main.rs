fn fibonacci_recursive(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
        /*
        
        6 -> (5)=5 + (4)=3
            5 -> (4)=3 + (3)=2 === 5
                4 -> (3)=2 + (2)=1 == 3
                    3 -> (2)=1 + (1) = 2
                        2 -> (1) + (0)
                    2 -> (1) + (0) 
                3 -> (2)=1 + (1) == 2
                    2 -> (1) + (0) = 1


            4 -> (3)=2 + (2)=1 == 3
                3 -> (2)=1 + (1) = 2
                    2 -> (1) + (0) = 1  

                2 -> (1) + (0) = 1 
            
         */
    }
}

fn main() {
    let n = 10; // Replace with the desired value of n
    let result = fibonacci_recursive(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
