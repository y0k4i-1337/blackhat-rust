use clap::Parser;
use std::error::Error;
use std::fs;

/// A simple grep-like tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Search string
    search: String,

    /// Target file
    file: String,

    /// Ignore case
    #[arg(short, long, env, action = clap::ArgAction::SetTrue)]
    ignore_case: bool,
}

pub fn run(args: Config) -> Result<(), Box<dyn Error>> {
    // propagate error
    let contents = fs::read_to_string(args.file)?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.search, &contents)
    } else {
        search(&args.search, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// define explicit lifetime: result should live as long as contents lives
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    // return results
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
