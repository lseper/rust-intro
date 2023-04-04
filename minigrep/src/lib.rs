use std::fs;
use std::error::Error;

pub struct Config {
    pub search_term: String,
    pub file_name: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() > 3 {
            return Err("Too many arguments passed in. Should only accept a text to search for and a filename");
        }

        let search_term = args[1].clone();
        let file_name = args[2].clone();
        
        return Ok(Config { search_term, file_name });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    println!("With text:\n{contents}");
    return Ok(());
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

/* 
Depracated - Replaced with the above Config constructor
 */
fn parse_config(args: &[String]) -> Config {
    let search_term = args[1].clone();
    let file_name = args[2].clone();

    return Config { search_term, file_name }
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
}