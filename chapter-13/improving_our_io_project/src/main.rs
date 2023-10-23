use improving_our_io_project::{run, search_v2, Config};
use std::{process, env};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Ok(content) = run(&config) {
        println!("{:?}",search_v2(&config.query, &content));
    } else {
        println!("Application error");
        process::exit(1);
    }

}