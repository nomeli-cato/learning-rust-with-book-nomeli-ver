use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    file_path: String,
    ignore_case: bool
}

// Returning a Result Instead of Calling panic!
impl Config {
  pub fn build(
      mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
      args.next();

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
      };

      let file_path = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a file path"),
      };

      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(Config {
          query,
          file_path,
          ignore_case,
      })
  }
}


// Making Code Clearer with Iterator Adaptors
// old version
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.contains(query) {
          results.push(line);
      }
  }

  results
}

// more concise code with iterator
pub fn search_v2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
      .lines()
      .filter(|line| line.contains(query))
      .collect()
}



// Extracting Logic from main
// Returning Errors from the run Function
pub fn run(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    // println!("With text:\n{contents}");

    Ok(contents)
}

