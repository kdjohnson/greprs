extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

use ansi_term::Colour::Cyan;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

#[derive(Debug)]
pub struct SearchResult {
    pub line_number: usize,
    pub result_string: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let query = &config.query;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for result in results {
        print!("{:?}: ", result.line_number);
        let colorize = |s: String| {
            let vec: Vec<&str> = s.split(" ").collect();
            for v in vec {
                if v.contains(query) {
                    print!("{} ", &Cyan.bold().paint(v).to_string());
                } else {
                    print!("{} ", v);
                }
            }
        };
        colorize(result.result_string.clone());
        println!("");
    }

    Ok(())
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<SearchResult> {
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push(SearchResult {
                line_number: i,
                result_string: line.to_string(),
            });
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<SearchResult> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        println!("count: {}", i);
        println!("line {}", line);
        if line.to_lowercase().contains(&query) {
            results.push(SearchResult {
                line_number: i,
                result_string: line.to_string(),
            });
        }
    }

    results
}