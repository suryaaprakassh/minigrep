use std::{fs, process};
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if results.len() < 1 {
        println!("The given string is not found in the file!");
        return Ok(());
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let flag: String = String::from("-r");
        if args.contains(&String::from("-h")) {
            println!(
                "\n\n USAGE: \n cli <pattern> <path>\nArguments:\n\n -r turn off case sensitivity"
            );
            process::exit(1);
        }
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();

        let filename = args[2].clone();

        let case_sensitive;

        if args.contains(&flag) {
            case_sensitive = false;
        } else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "fox";
        let contents: &str = "The quick\nbrown fox\njumps over the lazy dog.";

        assert_eq!(vec!["brown fox"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "fox";
        let contents: &str = "The quick\nbrown fox\njumps over the lazy dog.\nFox jumps over.";

        assert_eq!(
            vec!["brown fox", "Fox jumps over."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query: &str = "fox";
        let contents: &str = "The quick\nbrown fox\njumps over the lazy dog.\nFox jumps over.";

        assert_eq!(vec!["brown fox"], search(query, contents));
    }
}
