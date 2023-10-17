use std::process;
use std::env;
use as_library::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // --snip-- 
    if let Err(e) = as_library::run(config) {
        // --snip--
        println!("Application error: {e}");
        process::exit(1);
    }
}
