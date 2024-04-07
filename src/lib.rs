use std::error::Error;
use std::{env, fs};

#[cfg(test)]
mod tests {
    use crate::{search, search_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["Rust:"], search_insensitive(query, contents));
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search(&config.query, &file_contents)
    } else {
        search_insensitive(&config.query, &file_contents)
    };

    for line in result {
        println!("{}", line)
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
}
