use std::{env, process};
use tdd::{self, Config};

fn main(){
  let args: Vec<String> = env::args().collect();  

  let config = Config{
    query: args[1].clone(),
    file_path: args[2].clone(),
    ignore_case: true
  };
  
  // cargo run -- frog poem.txt
  if let Err(e)= tdd::run(config){
    println!("Aplication error {e}");
    process::exit(1)
  };


  println!("Env: \n");
  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  if let Err(e) = tdd::run(config) {
      eprintln!("Application error: {e}");
      process::exit(1);
  }
}