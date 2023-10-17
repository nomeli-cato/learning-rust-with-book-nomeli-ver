use std::env;

struct Config {
    query: String,
    file_path: String,
}

// Creating a Constructor for Config
impl Config {
    fn new(args: &[String]) -> Config {
        // Improving error message
        // if run code witg cargo without arguments show an error no legible to developer
        // to fix us ensure adding constraint of len of arg
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Creating a Constructor for Config
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
}
