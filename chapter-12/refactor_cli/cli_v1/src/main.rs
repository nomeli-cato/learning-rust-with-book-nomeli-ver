use  std::env;

fn main() {
    println!("v1");
    println!("");
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    // don´t use reference because we need to transfer ownership to new variable
    Config { query, file_path }
}

