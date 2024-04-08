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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(arg) => arg,
        };

        let file_path = match args.next() {
            None => return Err("Didn't get a file path"),
            Some(arg) => arg,
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}
