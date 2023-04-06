use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub search_term: String,
    pub file_name: String,
    pub ignore_case: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() > 3 {
            return Err("Too many arguments passed in. Should only accept a text to search for and a filename");
        }

        let search_term = args[1].clone();
        let file_name = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        return Ok(Config { search_term, file_name, ignore_case});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_term, &contents)
    } else {
        search(&config.search_term, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(())
}

pub fn search<'a>(terms: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(terms) {
            results.push(line);
        }
    }
   return results;
}

pub fn search_case_insensitive<'a>(terms: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let terms = terms.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&terms) {
            results.push(line);
        }
    }
   return results;
}

/* 
Depracated - Replaced with the above Config constructor
 */
fn parse_config(args: &[String]) -> Config {
    let search_term = args[1].clone();
    let file_name = args[2].clone();

    return Config { search_term, file_name, ignore_case: true }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let term = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(term, contents));
    }

    #[test]
    fn two_resuls() {
        let term = "hor";
        let contents = "\
A horse! A horse!
My army for a horse!";

        assert_eq!(vec!["A horse! A horse!", "My army for a horse!"], search(term, contents));
    }

    #[test]
    fn no_results() {
        let term = "sword";
        let contents = "\
I shot an arrow into the air,
It fell, 
I knew not where.";
        let expected: Vec<&str> = vec![];
    
        assert_eq!(expected, search(term, contents));
    }
    
    #[test]
    fn case_insensitivity_test() {
        let term = "ENG";
        let contents = "\
If it ain't broke,
don't fix it.

An engineer would say:
\"If it ain't broke,
it doesn't have enough features yet\"";
        let expected: Vec<&str> = vec!["An engineer would say:"];
    
        assert_eq!(expected, search_case_insensitive(term, contents));
    }

    #[test]
    fn case_sensitivity_test() {
        let term = "ENG";
        let contents = "\
If it ain't broke,
don't fix it.

An engineer would say:
\"If it ain't broke,
it doesn't have enough features yet\"";
        let expected: Vec<&str> = vec![];
    
        assert_eq!(expected, search(term, contents));
    }
    
}