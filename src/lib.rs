/*
Copyright (C) 2021  Jayesh Mann <jayeshmann06@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

//! Custom simplified implementation of grep

use std::env;
use std::error::Error;
use std::fs;

/// The necessary configurations for initializing milligrep.
///
pub struct Config {
    /// The pattern to search for in the contents of the file.
    pub query: String,
    /// The file along with the path in which the pattern search will be conducted.
    pub filename: String,
    /// CASE_SENSITIVE flag, either true or false
    pub case_sensitive: bool,
}

impl Config {
    /// Initializes a new Config.
    ///
    /// Returns error if incorrect option is passed.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
/// Runs the library logic.
/// Expects a reference to [`Config`](struct@Config).
///
/// # Errors
///
/// Returns error on the following situations:
///
/// - `file` doesn't exist.
/// - `file` could not be read (unauthorized read permission).
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
/// Returns the line(s) where the pattern is matched in the contents of the file.
/// It is case sensitive.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
/// Returns the line(s) where the pattern is matched in the contents of the file.
/// It is case in-sensitive.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn new_config() {
    //     let arg_array: Vec<String>;
    //     let res = Config::new(&arg_array).expect("Failed to init config.");
    // }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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
