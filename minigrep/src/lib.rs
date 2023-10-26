use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check to make sure the user passed enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Create variables to hold the query argument and file path argument
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return a new instance of Config
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified by the second argument
    let contents = fs::read_to_string(config.file_path)?;

    let contents = String::from_utf8_lossy(&contents);
    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct OtherConfig {
    pub query: String,
    pub file_path: String,
}
pub fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let contents = fs::read(filename)?;
    let contents = contents.as_slice();
    let contents = String::from_utf8_lossy(contents).into_owned();
    Ok(contents)
}

impl OtherConfig {
    pub fn build(args: &[String]) -> Result<OtherConfig, &'static str> {
        // Check to make sure the user passed enough arguments
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Create variables to hold the query argument and file path argument
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Return a new instance of Config
        Ok(OtherConfig { query, file_path })
    }
}
