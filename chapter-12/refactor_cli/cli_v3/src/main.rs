use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

// Returning a Result Instead of Calling panic!
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Extracting Logic from main

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Calling Config::build and Handling Errors
    let config2 = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config2.query);
    println!("In file {}", config2.file_path);
    
    run(config2);
}
